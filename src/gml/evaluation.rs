use crate::prelude::{Expression, ExpressionBox, IntoExpressionBox, ParseVisitor, StatementBox};

/// Representation of an assignment expression in gml.
#[derive(Debug, PartialEq, Clone)]
pub struct Evaluation {
    /// The left hand side of the evaluation.
    pub left: ExpressionBox,
    /// The operator used in this evaluation.
    pub operator: EvaluationOperator,
    /// The right hand side of the evaluation.
    pub right: ExpressionBox,
}
impl Evaluation {
    /// Creates a new evaluation.
    pub fn new(left: ExpressionBox, operator: EvaluationOperator, right: ExpressionBox) -> Self {
        Self { left, operator, right }
    }
}
impl From<Evaluation> for Expression {
    fn from(evaluation: Evaluation) -> Self {
        Self::Evaluation(evaluation)
    }
}
impl IntoExpressionBox for Evaluation {}
impl ParseVisitor for Evaluation {
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, _expression_visitor: E) {}
    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, _statement_visitor: S) {}
}

/// The various evaluation operations supported in gml.
///
/// TODO: Add the actual token
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum EvaluationOperator {
    /// +
    Plus,
    /// -
    Minus,
    /// /
    Slash,
    /// *
    Star,
    /// div
    Div,
    /// mod, %
    Modulo,
    /// &
    And,
    /// |
    Or,
    /// ^
    Xor,
    /// <<
    BitShiftLeft,
    /// >>
    BitShiftRight,
}
