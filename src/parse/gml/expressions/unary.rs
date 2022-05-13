use crate::parse::{Expr, ExprKind, IntoExpr, ParseVisitor, Stmt, Token};

/// Representation of a unary operation in gml.
#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub struct Unary {
    /// The unary operator.
    pub op: UnaryOp,
    /// The right hand side of the unary operation.
    pub right: Expr,
}
impl Unary {
    /// Creates a new unary operation.
    pub fn new(op: UnaryOp, right: Expr) -> Self {
        Self { op, right }
    }
}
impl From<Unary> for ExprKind {
    fn from(unary: Unary) -> Self {
        Self::Unary(unary)
    }
}
impl IntoExpr for Unary {}
impl ParseVisitor for Unary {
    fn visit_child_exprs<E: FnMut(&Expr)>(&self, mut visitor: E) {
        visitor(&self.right);
    }
    fn visit_child_exprs_mut<E: FnMut(&mut Expr)>(&mut self, mut visitor: E) {
        visitor(&mut self.right);
    }
    fn visit_child_stmts<S: FnMut(&Stmt)>(&self, mut _visitor: S) {}
    fn visit_child_stmts_mut<S: FnMut(&mut Stmt)>(&mut self, _visitor: S) {}
}

/// The various unary operations supported in gml.
#[derive(Debug, PartialEq, Clone, serde::Serialize)]
#[serde(tag = "type", content = "token", rename_all = "snake_case")]
pub enum UnaryOp {
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
impl std::fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOp::Increment(t)
            | UnaryOp::Decrement(t)
            | UnaryOp::Not(t)
            | UnaryOp::Positive(t)
            | UnaryOp::Negative(t)
            | UnaryOp::BitwiseNot(t) => f.pad(&t.to_string()),
        }
    }
}
