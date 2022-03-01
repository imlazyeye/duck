use crate::prelude::{Expression, ExpressionBox, IntoExpressionBox, ParseVisitor, StatementBox};

/// Representation of a null coalecence evaluation in gml.
#[derive(Debug, PartialEq, Clone)]
pub struct NullCoalecence {
    /// The left hand side of the null coalecence evaluation.
    pub left: ExpressionBox,
    /// The right hand side of the null coalecence evaluation.
    pub right: ExpressionBox,
}
impl NullCoalecence {
    /// Creates a new null coalecence evaluation.
    pub fn new(left: ExpressionBox, right: ExpressionBox) -> Self {
        Self { left, right }
    }
}
impl From<NullCoalecence> for Expression {
    fn from(null: NullCoalecence) -> Self {
        Self::NullCoalecence(null)
    }
}
impl IntoExpressionBox for NullCoalecence {}
impl ParseVisitor for NullCoalecence {
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, _expression_visitor: E) {}
    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, _statement_visitor: S) {}
}
