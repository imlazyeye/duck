use super::{Constraints, Marker, Printer, Scope, Term, Type, Unifier};
use crate::{
    parse::{Ast, Identifier, Stmt},
    FileId,
};
use codespan_reporting::diagnostic::Diagnostic;
use itertools::Itertools;

#[derive(Debug, Default)]
pub struct TypeWriter {
    pub printer: Printer,
}
impl TypeWriter {
    pub fn write_types(&mut self, ast: &mut Ast) -> Page {
        let mut page = Page::default();
        page.apply_stmts(ast.stmts_mut(), &mut self.printer);
        page
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Page {
    pub scope: Scope,
    pub unifier: Unifier,
    pub file_id: FileId,
}

impl Page {
    pub fn apply_stmts(&mut self, stmts: &mut Vec<Stmt>, printer: &mut Printer) {
        let constraints = Constraints::new(&mut self.scope, stmts, printer);
        self.unifier.apply_constraints(constraints.collection, printer);
        println!(
            "{}",
            &self
                .unifier
                .collection
                .iter()
                .map(|(marker, term)| format!("{} => {}", printer.marker(marker), printer.term(term)))
                .join("\n")
        );
    }

    /// ### Errors
    /// Returns an error if the field is not in scope.
    pub fn read_field(&self, identifier: &Identifier) -> Result<Type, Diagnostic<FileId>> {
        self.scope
            .field_marker(identifier)
            .map(|marker| self.marker_to_type(marker))
    }

    pub fn return_term(&self) -> Term {
        let tpe = self.marker_to_term(Marker::RETURN_VALUE);
        if let Term::Marker(Marker::RETURN_VALUE) = tpe {
            Term::Type(Type::Undefined)
        } else {
            tpe
        }
    }

    pub fn marker_to_type(&self, marker: Marker) -> Type {
        self.marker_to_term(marker).into()
    }

    pub fn marker_to_term(&self, marker: Marker) -> Term {
        self.unifier
            .collection
            .get(&marker)
            .cloned()
            .unwrap_or(Term::Marker(marker))
    }
}
