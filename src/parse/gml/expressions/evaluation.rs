use crate::parse::{Expression, ExpressionBox, IntoExpressionBox, ParseVisitor, StatementBox, Token};

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

/// The various evaluation operations supported in gml.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum EvaluationOperator {
    /// +
    Plus(Token),
    /// -
    Minus(Token),
    /// /
    Slash(Token),
    /// *
    Star(Token),
    /// div
    Div(Token),
    /// mod, %
    Modulo(Token),
    /// &
    And(Token),
    /// |
    Or(Token),
    /// ^
    Xor(Token),
    /// <<
    BitShiftLeft(Token),
    /// >>
    BitShiftRight(Token),
}
