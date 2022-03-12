use crate::parse::{ExpressionBox, IntoStatementBox, ParseVisitor, Statement, StatementBox};

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
        Self::DoUntil(do_until)
    }
}
impl IntoStatementBox for DoUntil {}
impl ParseVisitor for DoUntil {
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, mut visitor: E) {
        visitor(&self.condition);
    }
    fn visit_child_expressions_mut<E: FnMut(&mut ExpressionBox)>(&mut self, mut visitor: E) {
        visitor(&mut self.condition);
    }
    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, mut visitor: S) {
        visitor(&self.body);
    }
    fn visit_child_statements_mut<S: FnMut(&mut StatementBox)>(&mut self, mut visitor: S) {
        visitor(&mut self.body);
    }
}
