use crate::parse::{Expr, ExprType, IntoExpr, ParseVisitor, Stmt, Token};

/// Representation of an logical expression in gml.
#[derive(Debug, PartialEq, Clone)]
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
impl From<Logical> for ExprType {
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
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LogicalOp {
    /// and, &&
    And(Token),
    /// or, ||
    Or(Token),
    /// xor, ^^
    Xor(Token),
}
