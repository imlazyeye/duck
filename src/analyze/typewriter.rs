use super::{Application, Constraints, Deref, Marker, Scope, Symbol, Type};
use crate::{
    parse::{Ast, Identifier, Stmt},
    FileId,
};
use codespan_reporting::diagnostic::Diagnostic;
use hashbrown::HashMap;

#[derive(Debug, Default)]
pub struct TypeWriter;
impl TypeWriter {
    pub fn write_types(&mut self, ast: &mut Ast) -> Page {
        let mut page = Page::default();
        page.apply_stmts(ast.stmts_mut());
        page
    }
}

#[derive(Debug, Default)]
pub struct Page {
    pub scope: Scope,
    pub substitutions: HashMap<Marker, Symbol>,
    pub file_id: FileId,
}

impl Page {
    pub fn apply_stmts(&mut self, stmts: &mut Vec<Stmt>) {
        // Constrain everything
        let mut constraints = Constraints::create_collection(&mut self.scope, stmts);

        // Sub everything
        while let Some(pattern) = constraints.pop() {
            self.substitutions.insert(pattern.marker, pattern.symbol.clone());
            for target in constraints.iter_mut() {
                if let Some(sub) = self.simplify(&target.symbol) {
                    target.symbol = sub;
                } else if target.marker == pattern.marker {
                    // We can do a distributive transfer (if a == b and a == c, *b == c)
                    match &target.symbol {
                        Symbol::Constant(_) => {}
                        Symbol::Variable(marker) => {
                            self.substitutions.insert(*marker, pattern.symbol.clone());
                        }
                        Symbol::Application(_) => {}
                        Symbol::Deref(deref) => match deref {
                            Deref::Array(dereffed_marker) => {
                                let new_symbol = Symbol::Application(Application::Array {
                                    member_type: Box::new(pattern.symbol.clone()),
                                });
                                self.substitutions.insert(*dereffed_marker, new_symbol);
                            }
                            Deref::Object(_, _) => todo!(),
                        },
                        Symbol::Union(_) => {}
                    }
                }
            }
        }
        for (marker, symbol) in self.substitutions.iter() {
            println!("{} => {}", marker, symbol);
        }
    }

    /// ### Errors
    /// Returns an error if the field is not in scope.
    pub fn field_type(&self, identifier: &Identifier) -> Result<Type, Diagnostic<FileId>> {
        self.scope
            .field_marker(identifier)
            .map(|marker| self.seek_type_for(marker))
    }

    pub fn return_type(&self) -> Type {
        let tpe = self.seek_type_for(Marker::RETURN_VALUE);
        if let Type::Generic {
            marker: Marker::RETURN_VALUE,
        } = tpe
        {
            Type::Undefined
        } else {
            tpe
        }
    }

    pub fn seek_type_for(&self, marker: Marker) -> Type {
        let symbol = Symbol::Variable(marker);
        self.simplify(&symbol).unwrap_or(symbol).into()
    }

    fn simplify(&self, symbol: &Symbol) -> Option<Symbol> {
        fn find_simplification(page: &Page, symbol: &Symbol) -> Option<Symbol> {
            match symbol {
                Symbol::Variable(marker) => {
                    if let Some(sub) = page.substitutions.get(marker) {
                        return Some(sub.clone());
                    }
                }
                Symbol::Application(Application::Array { member_type }) => {
                    if let Some(member_sub) = find_simplification(page, member_type) {
                        return Some(Symbol::Application(Application::Array {
                            member_type: Box::new(member_sub),
                        }));
                    }
                }
                Symbol::Application(Application::Object { fields }) => {
                    let mut new_fields = fields.clone();
                    let mut any_changed = false;
                    for (_, field) in new_fields.iter_mut() {
                        if let Some(new_symbol) = find_simplification(page, field) {
                            any_changed = true;
                            *field = new_symbol;
                        }
                    }
                    if any_changed {
                        return Some(Symbol::Application(Application::Object { fields: new_fields }));
                    }
                }
                Symbol::Deref(Deref::Array(inner_marker)) => {
                    let member_type = page.simplify(&Symbol::Variable(*inner_marker)).and_then(|sub| {
                        if let Symbol::Application(Application::Array { member_type }) = sub {
                            Some(member_type)
                        } else {
                            None
                        }
                    });
                    if let Some(member_type) = member_type {
                        return Some(member_type.as_ref().clone());
                    }
                }
                Symbol::Deref(Deref::Object(inner_marker, field_name)) => {
                    if let Some(Symbol::Application(Application::Object { fields })) =
                        page.simplify(&Symbol::Variable(*inner_marker))
                    {
                        return Some(
                            fields
                                .get(field_name)
                                .expect("struct did not have required field")
                                .clone(),
                        );
                    }
                }
                Symbol::Constant(_) => {}
                Symbol::Union(_) => todo!(),
            }
            None
        }

        if let Some(mut inner) = find_simplification(self, symbol) {
            while let Some(new_symbol) = find_simplification(self, &inner) {
                inner = new_symbol;
            }
            Some(inner)
        } else {
            None
        }
    }
}
