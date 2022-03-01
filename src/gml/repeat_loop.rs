use crate::prelude::{ExpressionBox, IntoStatementBox, ParseVisitor, Statement, StatementBox};

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
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, mut expression_visitor: E) {
        expression_visitor(&self.tick_counts);
    }

    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, mut statement_visitor: S) {
        statement_visitor(&self.body);
    }
}
