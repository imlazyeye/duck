use crate::parse::{ExpressionBox, IntoStatementBox, ParseVisitor, Statement, StatementBox};

/// A delete statement, used to manually free memory.
#[derive(Debug, PartialEq, Clone)]
pub struct Delete {
    /// The value being freed.
    pub value: ExpressionBox,
}
impl Delete {
    /// Creates a new delete statement with the given value.
    pub fn new(value: ExpressionBox) -> Self {
        Self { value }
    }
}
impl From<Delete> for Statement {
    fn from(ret: Delete) -> Self {
        Self::Delete(ret)
    }
}
impl IntoStatementBox for Delete {}
impl ParseVisitor for Delete {
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, mut visitor: E) {
        visitor(&self.value);
    }
    fn visit_child_expressions_mut<E: FnMut(&mut ExpressionBox)>(&mut self, mut visitor: E) {
        visitor(&mut self.value);
    }
    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, mut _visitor: S) {}
    fn visit_child_statements_mut<S: FnMut(&mut StatementBox)>(&mut self, _visitor: S) {}
}
