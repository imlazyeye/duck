use crate::parse::{Expr, IntoStmt, ParseVisitor, Stmt, StmtKind};

/// A throw statement, contianing the value thrown.
#[derive(Debug, PartialEq, Clone)]
pub struct Throw {
    /// The value that is thrown as an exception.
    pub value: Expr,
}
impl Throw {
    /// Creates a new throw statement.
    pub fn new(value: Expr) -> Self {
        Self { value }
    }
}
impl From<Throw> for StmtKind {
    fn from(ret: Throw) -> Self {
        Self::Throw(ret)
    }
}
impl IntoStmt for Throw {}
impl ParseVisitor for Throw {
    fn visit_child_exprs<E: FnMut(&Expr)>(&self, mut visitor: E) {
        visitor(&self.value);
    }
    fn visit_child_exprs_mut<E: FnMut(&mut Expr)>(&mut self, mut visitor: E) {
        visitor(&mut self.value);
    }
    fn visit_child_stmts<S: FnMut(&Stmt)>(&self, mut _visitor: S) {}
    fn visit_child_stmts_mut<S: FnMut(&mut Stmt)>(&mut self, _visitor: S) {}
}
