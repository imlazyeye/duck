use crate::parse::{ExpressionBox, IntoStatementBox, ParseVisitor, Statement, StatementBox};

/// Representation of a while loop in gml.
#[derive(Debug, PartialEq, Clone)]
pub struct WhileLoop {
    /// The condition of this loop.
    pub condition: ExpressionBox,
    /// The body of the loop.
    pub body: StatementBox,
}
impl WhileLoop {
    /// Creates a new while loop.
    pub fn new(condition: ExpressionBox, body: StatementBox) -> Self {
        Self { condition, body }
    }
}
impl From<WhileLoop> for Statement {
    fn from(while_loop: WhileLoop) -> Self {
        Self::WhileLoop(while_loop)
    }
}
impl IntoStatementBox for WhileLoop {}
impl ParseVisitor for WhileLoop {
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
