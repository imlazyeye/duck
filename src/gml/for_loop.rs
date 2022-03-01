use crate::prelude::{ExpressionBox, IntoStatementBox, Statement, StatementBox};

/// Representation of a for loop in gml.
#[derive(Debug, PartialEq, Clone)]
pub struct ForLoop {
    /// The initializing statement in the for loop.
    pub initializer: StatementBox,
    /// The condition checked each tick in the loop.
    pub condition: ExpressionBox,
    /// The iterator statement run at the end of every tick.
    pub iterator: StatementBox,
    /// The body of the for loop.
    pub body: StatementBox,
}
impl ForLoop {
    /// Creates a new for loop.
    pub fn new(
        initializer: StatementBox,
        condition: ExpressionBox,
        iterator: StatementBox,
        body: StatementBox,
    ) -> Self {
        Self {
            initializer,
            condition,
            iterator,
            body,
        }
    }
}
impl From<ForLoop> for Statement {
    fn from(for_loop: ForLoop) -> Self {
        Statement::ForLoop(for_loop)
    }
}
impl IntoStatementBox for ForLoop {}
