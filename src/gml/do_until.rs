use crate::prelude::{ExpressionBox, IntoStatementBox, Statement, StatementBox};

/// Representation of a do/until loop in gml.
#[derive(Debug, PartialEq, Clone)]
pub struct DoUntil {
    /// The body of the loop.
    pub body: StatementBox,
    /// The condition of this loop.
    pub condition: ExpressionBox,
}
impl DoUntil {
    /// Creates a new do until loop.
    pub fn new(body: StatementBox, condition: ExpressionBox) -> Self {
        Self { condition, body }
    }
}
impl From<DoUntil> for Statement {
    fn from(do_until: DoUntil) -> Self {
        Statement::DoUntil(do_until)
    }
}
impl IntoStatementBox for DoUntil {}
