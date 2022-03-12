use crate::parse::{Expression, ExpressionBox, IntoExpressionBox, ParseVisitor, StatementBox, Token};

/// Representation of a equality expression in gml.
#[derive(Debug, PartialEq, Clone)]
pub struct Equality {
    /// The left hand side of the equality.
    pub left: ExpressionBox,
    /// The operator used in this equality.
    pub operator: EqualityOperator,
    /// The right hand side of the equality.
    pub right: ExpressionBox,
}
impl Equality {
    /// Creates a new equality.
    pub fn new(left: ExpressionBox, operator: EqualityOperator, right: ExpressionBox) -> Self {
        Self { left, operator, right }
    }
}
impl From<Equality> for Expression {
    fn from(equality: Equality) -> Self {
        Self::Equality(equality)
    }
}
impl IntoExpressionBox for Equality {}
impl ParseVisitor for Equality {
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, mut visitor: E) {
        visitor(&self.left);
        visitor(&self.right);
    }
    fn visit_child_expressions_mut<E: FnMut(&mut ExpressionBox)>(&mut self, mut visitor: E) {
        visitor(&mut self.left);
        visitor(&mut self.right);
    }
    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, mut _visitor: S) {}
    fn visit_child_statements_mut<S: FnMut(&mut StatementBox)>(&mut self, _visitor: S) {}
}

/// The various equality operations supported in gml.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum EqualityOperator {
    /// =, ==
    Equal(Token),
    /// !=
    NotEqual(Token),
    /// >
    GreaterThan(Token),
    /// >=
    GreaterThanOrEqual(Token),
    /// <
    LessThan(Token),
    /// <=
    LessThanOrEqual(Token),
}
