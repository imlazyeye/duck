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
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, mut expression_visitor: E) {
        expression_visitor(&self.value);
    }

    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, _statement_visitor: S) {}
}
