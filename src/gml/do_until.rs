use crate::prelude::{ExpressionBox, IntoStatementBox, ParseVisitor, Statement, StatementBox};

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
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, mut expression_visitor: E) {
        expression_visitor(&self.condition);
    }

    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, mut statement_visitor: S) {
        statement_visitor(&self.body);
    }
}
