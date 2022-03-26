use super::*;
use crate::{
    parse::{Identifier, Stmt},
    FileId,
};
use codespan_reporting::diagnostic::Diagnostic;
use colored::Colorize;
use itertools::Itertools;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TypeWriter {
    pub scope: Scope,
    pub unifier: Unifier,
    pub file_id: FileId,
}

impl TypeWriter {
    pub fn write(&mut self, stmts: &[Stmt]) {
        println!("\n--- Start TypeWriter::write... ---\n");
        let constraints = Constraints::new(&mut self.scope, stmts);
        self.unifier.apply_constraints(constraints.collection);
        println!("\nFinal substitutions:");
        println!(
            "{}",
            &self
                .unifier
                .substitutions
                .iter()
                .map(|(marker, term)| format!(
                    "{}    {} => {}",
                    "SUB".bright_green(),
                    Printer::marker(marker),
                    Printer::term(term)
                ))
                .join("\n")
        );
        println!("\n--- Ending TypeWriter::write... ---\n");
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
            .substitutions
            .get(&marker)
            .cloned()
            .unwrap_or(Term::Marker(marker))
    }
}
