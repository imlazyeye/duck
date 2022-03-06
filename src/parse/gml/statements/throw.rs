use crate::parse::{ExpressionBox, IntoStatementBox, ParseVisitor, Statement, StatementBox};

/// A throw statement, contianing the value thrown.
#[derive(Debug, PartialEq, Clone)]
pub struct Throw {
    /// The value that is thrown as an exception.
    pub value: ExpressionBox,
}
impl Throw {
    /// Creates a new throw statement.
    pub fn new(value: ExpressionBox) -> Self {
        Self { value }
    }
}
impl From<Throw> for Statement {
    fn from(ret: Throw) -> Self {
        Self::Throw(ret)
    }
}
impl IntoStatementBox for Throw {}
impl ParseVisitor for Throw {
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, mut expression_visitor: E) {
        expression_visitor(&self.value);
    }

    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, _statement_visitor: S) {}
}
