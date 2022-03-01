use crate::prelude::{Expression, ExpressionBox, IntoExpressionBox, ParseVisitor};

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
        Self::Assignment(assignment)
    }
}
impl IntoExpressionBox for Assignment {}
impl ParseVisitor for Assignment {
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, _expression_visitor: E) {}
    fn visit_child_statements<S: FnMut(&crate::prelude::StatementBox)>(&self, _statement_visitor: S) {}
}

/// The various assignment operations supported in gml.
///
/// TODO: Add the actual token
#[derive(Debug, PartialEq, Clone)]
#[allow(clippy::enum_variant_names)]
pub enum AssignmentOperator {
    /// =
    Equal,
    /// +=
    PlusEqual,
    /// -=
    MinusEqual,
    /// *=
    StarEqual,
    /// /=
    SlashEqual,
    /// ^=
    XorEqual,
    /// |=
    OrEqual,
    /// &=
    AndEqual,
    /// ??=
    NullCoalecenceEqual,
    /// %=
    ModEqual,
}
