use super::{Application, Constraint, Constraints, Inspection, Marker, Scope, Term, Type};
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
pub struct Substitutions {
    collection: HashMap<Marker, Term>,
}
impl Substitutions {
    pub fn add(&mut self, marker: Marker, term: Term) {}
}

#[derive(Debug, Default)]
pub struct Page {
    pub scope: Scope,
    pub substitutions: HashMap<Marker, Term>,
    pub file_id: FileId,
}

impl Page {
    pub fn apply_stmts(&mut self, stmts: &mut Vec<Stmt>) {
        // Constrain everything
        let mut constraints = Constraints::create_collection(&mut self.scope, stmts);
        for con in constraints.iter() {
            println!("{con}");
        }

        // Sub everything
        while let Some(mut pattern) = constraints.pop() {
            if let Some(sub) = self.simplify(&pattern.term) {
                pattern.term = sub;
            }
            for target in constraints.iter_mut() {
                if let Some(sub) = self
                    .simplify(&target.term)
                    .filter(|sub| sub != &Term::Marker(target.marker))
                {
                    // We have a new value for this target of this constraint
                    println!("setting {} to {sub}", target.marker);
                    target.term = sub;
                } else if target.marker == pattern.marker {
                    self.unify(&target.term, &pattern.term);
                }
            }
            self.substitutions.insert(pattern.marker, pattern.term.clone());
        }
        for (marker, term) in self.substitutions.iter() {
            println!("{} => {}", marker, term);
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
        let term = Term::Marker(marker);
        self.simplify(&term).unwrap_or(term).into()
    }

    fn simplify(&self, term: &Term) -> Option<Term> {
        fn find_simplification(page: &Page, term: &Term) -> Option<Term> {
            match term {
                Term::Marker(marker) => {
                    if let Some(sub) = page.substitutions.get(marker) {
                        if Term::Marker(*marker) == *sub {
                            return None;
                        }
                        return Some(sub.clone());
                    }
                }
                Term::Application(Application::Array { member_type }) => {
                    if let Some(member_sub) = find_simplification(page, member_type) {
                        return Some(Term::Application(Application::Array {
                            member_type: Box::new(member_sub),
                        }));
                    }
                }
                Term::Application(Application::Object { fields }) => {
                    let mut new_fields = fields.clone();
                    let mut any_changed = false;
                    for (_, field) in new_fields.iter_mut() {
                        if let Some(new_symbol) = find_simplification(page, field) {
                            any_changed = true;
                            *field = new_symbol;
                        }
                    }
                    if any_changed {
                        return Some(Term::Application(Application::Object { fields: new_fields }));
                    }
                }
                Term::Application(Application::Call { call_target, arguments }) => {
                    let mut any_changed = false;
                    let call_target = if let Some(sub) = find_simplification(page, call_target) {
                        any_changed = true;
                        Box::new(sub)
                    } else {
                        call_target.clone()
                    };
                    let mut new_arguments = arguments.clone();
                    for arg in new_arguments.iter_mut() {
                        if let Some(new_symbol) = find_simplification(page, arg) {
                            any_changed = true;
                            *arg = new_symbol;
                        }
                    }
                    if any_changed {
                        return Some(Term::Application(Application::Call {
                            call_target,
                            arguments: new_arguments,
                        }));
                    }
                }
                Term::Inspection(Inspection {
                    marker: inner_marker,
                    field: field_name,
                }) => {
                    if let Some(Term::Application(Application::Object { fields })) =
                        page.simplify(&Term::Marker(*inner_marker))
                    {
                        return Some(
                            fields
                                .get(field_name)
                                .expect("struct did not have required field")
                                .clone(),
                        );
                    }
                }
                Term::Type(tpe) => {}
                Term::Union(_) => todo!(),
            }
            None
        }

        if let Some(mut inner) = find_simplification(self, term) {
            while let Some(new_symbol) = find_simplification(self, &inner) {
                inner = new_symbol;
            }
            Some(inner)
        } else {
            None
        }
    }

    fn unify(&mut self, lhs: &Term, rhs: &Term) {
        println!("unifying {lhs} and {rhs}");
        match lhs {
            Term::Marker(inner_marker) => {
                if !matches!(rhs, Term::Marker(_)) {
                    println!("inserting {inner_marker} = {rhs}");
                    self.substitutions.insert(*inner_marker, rhs.clone());
                }
            }
            Term::Type(tpe) => {
                if let Term::Type(other_tpe) = rhs {
                    assert_eq!(tpe, other_tpe)
                } else {
                    self.unify(rhs, lhs)
                }
            }
            Term::Application(application) => match application {
                Application::Array { member_type } => match rhs {
                    Term::Application(Application::Array {
                        member_type: rhs_member_type,
                    }) => self.unify(member_type, rhs_member_type),
                    _ => panic!("no"),
                },
                Application::Object { fields } => match rhs {
                    Term::Application(Application::Object { fields: rhs_fields }) => {
                        for (name, field) in fields {
                            self.unify(field, rhs_fields.get(name).expect("missing field on struct"))
                        }
                    }
                    _ => panic!("no"),
                },
                Application::Call { call_target, arguments } => todo!(),
            },
            _ => panic!("no"),
        }
    }
}
