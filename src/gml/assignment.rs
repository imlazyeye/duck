use crate::prelude::{Expression, ExpressionBox, IntoExpressionBox};

/// Representation of an assignment expression in gml.
#[derive(Debug, PartialEq, Clone)]
pub struct Assignment {
    /// The left hand side of the assignment, aka the target.
    pub left: ExpressionBox,
    /// The operator used in this assignment.
    pub operator: AssignmentOperator,
    /// The right hand side of the assignment, aka the value.
    pub right: ExpressionBox,
}
impl Assignment {
    /// Creates a new assignment.
    pub fn new(left: ExpressionBox, operator: AssignmentOperator, right: ExpressionBox) -> Self {
        Self { left, operator, right }
    }
}
impl From<Assignment> for Expression {
    fn from(assignment: Assignment) -> Self {
        Expression::Assignment(assignment)
    }
}
impl IntoExpressionBox for Assignment {}

/// The various assignment operations supported in gml.
///
/// TODO: Add the actual token
#[derive(Debug, PartialEq, Clone)]
#[allow(clippy::enum_variant_names)]
pub enum AssignmentOperator {
    Equal,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    XorEqual,
    OrEqual,
    AndEqual,
    NullCoalecenceEqual,
    ModEqual,
}
