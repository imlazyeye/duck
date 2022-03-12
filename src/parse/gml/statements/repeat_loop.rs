use crate::parse::{ExpressionBox, IntoStatementBox, ParseVisitor, Statement, StatementBox};

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
        Self::RepeatLoop(repeat_loop)
    }
}
impl IntoStatementBox for RepeatLoop {}
impl ParseVisitor for RepeatLoop {
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, mut visitor: E) {
        visitor(&self.tick_counts);
    }
    fn visit_child_expressions_mut<E: FnMut(&mut ExpressionBox)>(&mut self, mut visitor: E) {
        visitor(&mut self.tick_counts);
    }
    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, mut visitor: S) {
        visitor(&self.body);
    }
    fn visit_child_statements_mut<S: FnMut(&mut StatementBox)>(&mut self, mut visitor: S) {
        visitor(&mut self.body);
    }
}
