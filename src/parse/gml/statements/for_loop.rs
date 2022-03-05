use crate::parse::{ExpressionBox, IntoStatementBox, ParseVisitor, Statement, StatementBox};

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
        Self::ForLoop(for_loop)
    }
}
impl IntoStatementBox for ForLoop {}
impl ParseVisitor for ForLoop {
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, mut expression_visitor: E) {
        expression_visitor(&self.condition);
    }

    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, mut statement_visitor: S) {
        statement_visitor(&self.initializer);
        statement_visitor(&self.iterator);
        statement_visitor(&self.body);
    }
}
