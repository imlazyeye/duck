use super::{Expr, Stmt};

/// Used to visit the children of a Stmt/Expr as we recurse down the tree.
pub trait ParseVisitor {
    /// Visits all expressions this T contains.
    fn visit_child_exprs<E: FnMut(&Expr)>(&self, visitor: E);
    /// Visits all expressions this T contains mutably.
    fn visit_child_exprs_mut<E: FnMut(&mut Expr)>(&mut self, visitor: E);
    /// Visits all statements this T contains.
    fn visit_child_stmts<S: FnMut(&Stmt)>(&self, visitor: S);
    /// Visits all statements this T contains mutably.
    fn visit_child_stmts_mut<S: FnMut(&mut Stmt)>(&mut self, visitor: S);
}