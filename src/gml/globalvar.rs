use crate::prelude::{ExpressionBox, IntoStatementBox, ParseVisitor, Statement, StatementBox};

/// Representation of a globalvar in gml.
#[derive(Debug, PartialEq, Clone)]
pub struct Globalvar {
    /// The name of the declared globalvar.
    pub name: String,
}
impl Globalvar {
    /// Creates a new globalvar with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}
impl From<Globalvar> for Statement {
    fn from(glob: Globalvar) -> Self {
        Self::GlobalvarDeclaration(glob)
    }
}
impl IntoStatementBox for Globalvar {}
impl ParseVisitor for Globalvar {
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, _expression_visitor: E) {}
    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, _statement_visitor: S) {}
}
