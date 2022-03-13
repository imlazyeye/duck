use crate::parse::{Expr, IntoStmt, ParseVisitor, Stmt, StmtType};

/// Representation of a while loop in gml.
#[derive(Debug, PartialEq, Clone)]
pub struct WhileLoop {
    /// The condition of this loop.
    pub condition: Expr,
    /// The body of the loop.
    pub body: Stmt,
}
impl WhileLoop {
    /// Creates a new while loop.
    pub fn new(condition: Expr, body: Stmt) -> Self {
        Self { condition, body }
    }
}
impl From<WhileLoop> for StmtType {
    fn from(while_loop: WhileLoop) -> Self {
        Self::WhileLoop(while_loop)
    }
}
impl IntoStmt for WhileLoop {}
impl ParseVisitor for WhileLoop {
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
