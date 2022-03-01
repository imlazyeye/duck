use crate::prelude::{Expression, ExpressionBox, IntoExpressionBox, ParseVisitor, StatementBox};

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
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, _expression_visitor: E) {}
    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, _statement_visitor: S) {}
}

/// The various equality operations supported in gml.
///
/// TODO: Add the actual token
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum EqualityOperator {
    // =, ==
    Equal,
    // !=
    NotEqual,
    // >
    GreaterThan,
    // >=
    GreaterThanOrEqual,
    // <
    LessThan,
    // <=
    LessThanOrEqual,
}
