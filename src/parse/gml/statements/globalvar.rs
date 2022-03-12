use crate::parse::{ExpressionBox, Identifier, IntoStatementBox, ParseVisitor, Statement, StatementBox};

/// Representation of a globalvar in gml.
#[derive(Debug, PartialEq, Clone)]
pub struct Globalvar {
    /// The name of the declared globalvar.
    pub name: Identifier,
}
impl Globalvar {
    /// Creates a new globalvar with the given name.
    pub fn new(name: Identifier) -> Self {
        Self { name }
    }
}
impl From<Globalvar> for Statement {
    fn from(glob: Globalvar) -> Self {
        Self::GlobalvarDeclaration(glob)
    }
}
impl IntoStatementBox for Globalvar {}
impl ParseVisitor for Globalvar {
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, mut _visitor: E) {}
    fn visit_child_expressions_mut<E: FnMut(&mut ExpressionBox)>(&mut self, _visitor: E) {}
    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, mut _visitor: S) {}
    fn visit_child_statements_mut<S: FnMut(&mut StatementBox)>(&mut self, _visitor: S) {}
}
