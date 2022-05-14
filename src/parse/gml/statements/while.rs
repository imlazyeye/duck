use crate::parse::{Expr, IntoStmt, ParseVisitor, Stmt, StmtKind};

/// Representation of a while loop in gml.
#[derive(Debug, PartialEq, Clone)]
pub struct While {
    /// The condition of this loop.
    pub condition: Expr,
    /// The body of the loop.
    pub body: Stmt,
}
impl While {
    /// Creates a new while loop.
    pub fn new(condition: Expr, body: Stmt) -> Self {
        Self { condition, body }
    }
}
impl From<While> for StmtKind {
    fn from(while_loop: While) -> Self {
        Self::WhileLoop(while_loop)
    }
}
impl IntoStmt for While {}
impl ParseVisitor for While {
    fn visit_child_exprs<E: FnMut(&Expr)>(&self, mut visitor: E) {
        visitor(&self.condition);
    }
    fn visit_child_exprs_mut<E: FnMut(&mut Expr)>(&mut self, mut visitor: E) {
        visitor(&mut self.condition);
    }
    fn visit_child_stmts<S: FnMut(&Stmt)>(&self, mut visitor: S) {
        visitor(&self.body);
    }
    fn visit_child_stmts_mut<S: FnMut(&mut Stmt)>(&mut self, mut visitor: S) {
        visitor(&mut self.body);
    }
}
