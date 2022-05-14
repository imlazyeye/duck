use crate::parse::{Expr, IntoStmt, ParseVisitor, Stmt, StmtKind};

/// Representation of a for loop in gml.
#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub struct For {
    /// The initializing statement in the for loop.
    pub initializer: Stmt,
    /// The condition checked each tick in the loop.
    pub condition: Expr,
    /// The iterator statement run at the end of every tick.
    pub iterator: Stmt,
    /// The body of the for loop.
    pub body: Stmt,
}
impl For {
    /// Creates a new for loop.
    pub fn new(initializer: Stmt, condition: Expr, iterator: Stmt, body: Stmt) -> Self {
        Self {
            initializer,
            condition,
            iterator,
            body,
        }
    }
}
impl From<For> for StmtKind {
    fn from(for_loop: For) -> Self {
        Self::For(for_loop)
    }
}
impl IntoStmt for For {}
impl ParseVisitor for For {
    fn visit_child_exprs<E: FnMut(&Expr)>(&self, mut visitor: E) {
        visitor(&self.condition);
    }
    fn visit_child_exprs_mut<E: FnMut(&mut Expr)>(&mut self, mut visitor: E) {
        visitor(&mut self.condition);
    }
    fn visit_child_stmts<S: FnMut(&Stmt)>(&self, mut visitor: S) {
        visitor(&self.initializer);
        visitor(&self.iterator);
        visitor(&self.body);
    }
    fn visit_child_stmts_mut<S: FnMut(&mut Stmt)>(&mut self, mut visitor: S) {
        visitor(&mut self.initializer);
        visitor(&mut self.iterator);
        visitor(&mut self.body);
    }
}
