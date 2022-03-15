use crate::parse::{Expr, ExprType, IntoExpr, ParseVisitor, Stmt, Token};

/// Representation of an assignment expression in gml.
#[derive(Debug, PartialEq, Clone)]
pub struct Evaluation {
    /// The left hand side of the evaluation.
    pub left: Expr,
    /// The operator used in this evaluation.
    pub operator: EvaluationOp,
    /// The right hand side of the evaluation.
    pub right: Expr,
}
impl Evaluation {
    /// Creates a new evaluation.
    pub fn new(left: Expr, operator: EvaluationOp, right: Expr) -> Self {
        Self { left, operator, right }
    }
}
impl From<Evaluation> for ExprType {
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
