use crate::parse::{Expr, IntoStmt, ParseVisitor, Stmt, StmtKind};

/// Representation of a do/until loop in gml.
#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub struct DoUntil {
    /// The body of the loop.
    pub body: Stmt,
    /// The condition of this loop.
    pub condition: Expr,
}
impl DoUntil {
    /// Creates a new do until loop.
    pub fn new(body: Stmt, condition: Expr) -> Self {
        Self { condition, body }
    }
}
impl From<DoUntil> for StmtKind {
    fn from(do_until: DoUntil) -> Self {
        Self::DoUntil(do_until)
    }
}
impl IntoStmt for DoUntil {}
impl ParseVisitor for DoUntil {
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
