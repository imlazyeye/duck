use crate::prelude::{Expression, ExpressionBox, IntoExpressionBox, ParseVisitor, StatementBox};

/// Representation of a unary operation in gml.
#[derive(Debug, PartialEq, Clone)]
pub struct Unary {
    /// The unary operator.
    pub operator: UnaryOperator,
    /// The right hand side of the unary operation.
    pub right: ExpressionBox,
}
impl Unary {
    /// Creates a new unary operation.
    pub fn new(operator: UnaryOperator, right: ExpressionBox) -> Self {
        Self { operator, right }
    }
}
impl From<Unary> for Expression {
    fn from(unary: Unary) -> Self {
        Self::Unary(unary)
    }
}
impl IntoExpressionBox for Unary {}
impl ParseVisitor for Unary {
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, _expression_visitor: E) {}
    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, _statement_visitor: S) {}
}

/// The various unary operations supported in gml.
///
/// TODO: Add the actual token
#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    /// ++
    Increment,
    /// --
    Decrement,
    /// not, !
    Not,
    /// +
    Positive,
    /// -
    Negative,
    /// ~
    BitwiseNot,
}
