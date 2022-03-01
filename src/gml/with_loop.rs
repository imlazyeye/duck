use crate::prelude::{ExpressionBox, IntoStatementBox, ParseVisitor, Statement, StatementBox};

/// Representation of a with loop in gml.
#[derive(Debug, PartialEq, Clone)]
pub struct WithLoop {
    /// The expression representing the symbol being iterated over.
    pub identity: ExpressionBox,
    /// The body of the loop.
    pub body: StatementBox,
}
impl WithLoop {
    /// Creates a new with loop.
    pub fn new(identity: ExpressionBox, body: StatementBox) -> Self {
        Self { identity, body }
    }
}
impl From<WithLoop> for Statement {
    fn from(with_loop: WithLoop) -> Self {
        Self::WithLoop(with_loop)
    }
}
impl IntoStatementBox for WithLoop {}
impl ParseVisitor for WithLoop {
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, mut expression_visitor: E) {
        expression_visitor(&self.identity);
    }

    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, mut statement_visitor: S) {
        statement_visitor(&self.body);
    }
}
