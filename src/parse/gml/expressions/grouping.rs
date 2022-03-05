use crate::parse::{Expression, ExpressionBox, IntoExpressionBox, ParseVisitor, StatementBox};

/// Representation of a grouping in gml.
#[derive(Debug, PartialEq, Clone)]
pub struct Grouping {
    /// The inner expression contained by this grouping.
    pub inner: ExpressionBox,
}
impl Grouping {
    /// Creates a new grouping.
    pub fn new(inner: ExpressionBox) -> Self {
        Self { inner }
    }
}
impl From<Grouping> for Expression {
    fn from(grouping: Grouping) -> Self {
        Self::Grouping(grouping)
    }
}
impl IntoExpressionBox for Grouping {}
impl ParseVisitor for Grouping {
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, mut expression_visitor: E) {
        expression_visitor(&self.inner);
    }
    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, _statement_visitor: S) {}
}
