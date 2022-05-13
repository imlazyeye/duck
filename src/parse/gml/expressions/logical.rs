use crate::parse::{Expr, ExprKind, IntoExpr, ParseVisitor, Stmt, Token};

/// Representation of an logical expression in gml.
#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub struct Logical {
    /// The left hand side of the logical assessment.
    pub left: Expr,
    /// The operator used in this logical assesment..
    pub op: LogicalOp,
    /// The right hand side of the logical assessment.
    pub right: Expr,
}
impl Logical {
    /// Creates a new logical assessment.
    pub fn new(left: Expr, op: LogicalOp, right: Expr) -> Self {
        Self { left, op, right }
    }
}
impl From<Logical> for ExprKind {
    fn from(logical: Logical) -> Self {
        Self::Logical(logical)
    }
}
impl IntoExpr for Logical {}
impl ParseVisitor for Logical {
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

/// The various logical operations supported in gml.
#[derive(Debug, PartialEq, Clone, Copy, serde::Serialize)]
#[serde(tag = "type", content = "token", rename_all = "snake_case")]
pub enum LogicalOp {
    /// and, &&
    And(Token),
    /// or, ||
    Or(Token),
    /// xor, ^^
    Xor(Token),
}

impl std::fmt::Display for LogicalOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogicalOp::And(t) | LogicalOp::Or(t) | LogicalOp::Xor(t) => f.pad(&t.to_string()),
        }
    }
}
