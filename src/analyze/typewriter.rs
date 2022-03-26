use super::*;
use crate::{
    parse::{Identifier, Stmt},
    FileId,
};
use codespan_reporting::diagnostic::Diagnostic;
use hashbrown::{HashMap, HashSet};

#[derive(Debug, PartialEq, Clone)]
pub struct Typewriter {
    pub scope: Scope,
    pub substitutions: HashMap<Marker, Term>,
    pub unresolved: HashSet<Marker>,
    pub collection: Vec<Constraint>,
}

impl Typewriter {
    pub fn new(scope: Scope) -> Self {
        Self {
            scope,
            substitutions: HashMap::default(),
            unresolved: HashSet::default(),
            collection: Vec::default(),
        }
    }

    pub fn write(&mut self, stmts: &[Stmt]) {
        println!("\n--- Start TypeWriter::write... ---\n");
        let constraints = Constraints::new(&mut self.scope, stmts);
        Unification::apply_constraints(constraints.collection, self);
        println!("\nFinal substitutions:");
        self.substitutions
            .iter()
            .for_each(|(marker, term)| println!("{}", Printer::substitution(marker, term)));
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
        self.substitutions.get(&marker).cloned().unwrap_or(Term::Marker(marker))
    }
}
