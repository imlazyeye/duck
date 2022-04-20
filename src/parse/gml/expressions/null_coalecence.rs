use crate::parse::{Expr, ExprKind, IntoExpr, ParseVisitor, Stmt};

/// Representation of a null coalecence evaluation in gml.
#[derive(Debug, PartialEq, Clone)]
pub struct NullCoalecence {
    /// The left hand side of the null coalecence evaluation.
    pub left: Expr,
    /// The right hand side of the null coalecence evaluation.
    pub right: Expr,
}
impl NullCoalecence {
    /// Creates a new null coalecence evaluation.
    pub fn new(left: Expr, right: Expr) -> Self {
        Self { left, right }
    }
}
impl From<NullCoalecence> for ExprKind {
    fn from(null: NullCoalecence) -> Self {
        Self::NullCoalecence(null)
    }
}
impl IntoExpr for NullCoalecence {}
impl ParseVisitor for NullCoalecence {
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
