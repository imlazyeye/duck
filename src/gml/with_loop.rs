use crate::prelude::{ExpressionBox, IntoStatementBox, Statement, StatementBox};

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
        Statement::WithLoop(with_loop)
    }
}
impl IntoStatementBox for WithLoop {}
