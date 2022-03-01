use crate::prelude::{ExpressionBox, IntoStatementBox, Statement, StatementBox};

/// Representation of a repeat loop in gml.
#[derive(Debug, PartialEq, Clone)]
pub struct RepeatLoop {
    /// The expression dictating the amount of ticks.
    pub tick_counts: ExpressionBox,
    /// The body of the loop.
    pub body: StatementBox,
}
impl RepeatLoop {
    /// Creates a new repeat loop.
    pub fn new(tick_counts: ExpressionBox, body: StatementBox) -> Self {
        Self { tick_counts, body }
    }
}
impl From<RepeatLoop> for Statement {
    fn from(repeat_loop: RepeatLoop) -> Self {
        Statement::RepeatLoop(repeat_loop)
    }
}
impl IntoStatementBox for RepeatLoop {}
