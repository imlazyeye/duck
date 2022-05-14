use crate::parse::{Expr, IntoStmt, ParseVisitor, Stmt, StmtKind};

/// Representation of a with loop in gml.
#[derive(Debug, PartialEq, Clone)]
pub struct With {
    /// The expression representing the symbol being iterated over.
    pub identity: Expr,
    /// The body of the loop.
    pub body: Stmt,
}
impl With {
    /// Creates a new with loop.
    pub fn new(identity: Expr, body: Stmt) -> Self {
        Self { identity, body }
    }
}
impl From<With> for StmtKind {
    fn from(with_loop: With) -> Self {
        Self::WithLoop(with_loop)
    }
}
impl IntoStmt for With {}
impl ParseVisitor for With {
    fn visit_child_exprs<E: FnMut(&Expr)>(&self, mut visitor: E) {
        visitor(&self.identity);
    }
    fn visit_child_exprs_mut<E: FnMut(&mut Expr)>(&mut self, mut visitor: E) {
        visitor(&mut self.identity);
    }
    fn visit_child_stmts<S: FnMut(&Stmt)>(&self, mut visitor: S) {
        visitor(&self.body);
    }
    fn visit_child_stmts_mut<S: FnMut(&mut Stmt)>(&mut self, mut visitor: S) {
        visitor(&mut self.body);
    }
}
