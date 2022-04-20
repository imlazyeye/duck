use crate::parse::{Expr, ExprKind, IntoExpr, ParseVisitor, Stmt, Token};

/// A mathmatical evaluation.
#[derive(Debug, PartialEq, Clone)]
pub struct Evaluation {
    /// The left hand side of the evaluation.
    pub left: Expr,
    /// The operator used in this evaluation.
    pub op: EvaluationOp,
    /// The right hand side of the evaluation.
    pub right: Expr,
}
impl Evaluation {
    /// Creates a new evaluation.
    pub fn new(left: Expr, op: EvaluationOp, right: Expr) -> Self {
        Self { left, op, right }
    }
}
impl From<Evaluation> for ExprKind {
    fn from(evaluation: Evaluation) -> Self {
        Self::Evaluation(evaluation)
    }
}
impl IntoExpr for Evaluation {}
impl ParseVisitor for Evaluation {
    fn visit_child_exprs<E: FnMut(&Expr)>(&self, mut visitor: E) {
        visitor(&self.left);
        visitor(&self.right);
    }
    fn visit_child_exprs_mut<E: FnMut(&mut Expr)>(&mut self, mut visitor: E) {
        visitor(&mut self.left);
        visitor(&mut self.right);
    }
    fn visit_child_stmts<S: FnMut(&Stmt)>(&self, mut _visitor: S) {}
    fn visit_child_stmts_mut<S: FnMut(&mut Stmt)>(&mut self, _visitor: S) {}
}

/// The various evaluation operations supported in gml.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum EvaluationOp {
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
impl std::fmt::Display for EvaluationOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvaluationOp::Plus(t)
            | EvaluationOp::Minus(t)
            | EvaluationOp::Slash(t)
            | EvaluationOp::Star(t)
            | EvaluationOp::Div(t)
            | EvaluationOp::Modulo(t)
            | EvaluationOp::And(t)
            | EvaluationOp::Or(t)
            | EvaluationOp::Xor(t)
            | EvaluationOp::BitShiftLeft(t)
            | EvaluationOp::BitShiftRight(t) => f.pad(&t.to_string()),
        }
    }
}
