use crate::parse::{Expr, ExprType, IntoExpr, ParseVisitor, Stmt, Token};

/// Representation of a equality expression in gml.
#[derive(Debug, PartialEq, Clone)]
pub struct Equality {
    /// The left hand side of the equality.
    pub left: Expr,
    /// The operator used in this equality.
    pub operator: EqualityOperator,
    /// The right hand side of the equality.
    pub right: Expr,
}
impl Equality {
    /// Creates a new equality.
    pub fn new(left: Expr, operator: EqualityOperator, right: Expr) -> Self {
        Self { left, operator, right }
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
pub enum EqualityOperator {
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
