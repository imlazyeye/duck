use crate::parse::{Expression, ExpressionBox, IntoExpressionBox, ParseVisitor, StatementBox, Token};

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
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, mut visitor: E) {
        visitor(&self.right);
    }
    fn visit_child_expressions_mut<E: FnMut(&mut ExpressionBox)>(&mut self, mut visitor: E) {
        visitor(&mut self.right);
    }
    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, mut _visitor: S) {}
    fn visit_child_statements_mut<S: FnMut(&mut StatementBox)>(&mut self, _visitor: S) {}
}

/// The various unary operations supported in gml.
#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    /// ++
    Increment(Token),
    /// --
    Decrement(Token),
    /// not, !
    Not(Token),
    /// +
    Positive(Token),
    /// -
    Negative(Token),
    /// ~
    BitwiseNot(Token),
}
