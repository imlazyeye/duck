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
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, mut visitor: E) {
        visitor(&self.condition);
    }
    fn visit_child_expressions_mut<E: FnMut(&mut ExpressionBox)>(&mut self, mut visitor: E) {
        visitor(&mut self.condition);
    }
    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, mut visitor: S) {
        visitor(&self.initializer);
        visitor(&self.iterator);
        visitor(&self.body);
    }
    fn visit_child_statements_mut<S: FnMut(&mut StatementBox)>(&mut self, mut visitor: S) {
        visitor(&mut self.initializer);
        visitor(&mut self.iterator);
        visitor(&mut self.body);
    }
}
