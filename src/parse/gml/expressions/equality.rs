use crate::parse::{Expr, ExprType, IntoExpr, ParseVisitor, Stmt, Token};

/// Representation of a equality expression in gml.
#[derive(Debug, PartialEq, Clone)]
pub struct Equality {
    /// The left hand side of the equality.
    pub left: Expr,
    /// The operator used in this equality.
    pub op: EqualityOp,
    /// The right hand side of the equality.
    pub right: Expr,
}
impl Equality {
    /// Creates a new equality.
    pub fn new(left: Expr, op: EqualityOp, right: Expr) -> Self {
        Self { left, op, right }
    }
}
impl From<Equality> for ExprType {
    fn from(equality: Equality) -> Self {
        Self::Equality(equality)
    }
}
impl IntoExpr for Equality {}
impl ParseVisitor for Equality {
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

/// The various equality operations supported in gml.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum EqualityOp {
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

impl std::fmt::Display for EqualityOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EqualityOp::Equal(t)
            | EqualityOp::NotEqual(t)
            | EqualityOp::GreaterThan(t)
            | EqualityOp::GreaterThanOrEqual(t)
            | EqualityOp::LessThan(t)
            | EqualityOp::LessThanOrEqual(t) => f.pad(&t.to_string()),
        }
    }
}
