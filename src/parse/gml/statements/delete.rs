use crate::parse::{Expr, IntoStmt, ParseVisitor, Stmt, StmtKind};

/// A delete statement, used to manually free memory.
#[derive(Debug, PartialEq, Clone)]
pub struct Delete {
    /// The value being freed.
    pub value: Expr,
}
impl Delete {
    /// Creates a new delete statement with the given value.
    pub fn new(value: Expr) -> Self {
        Self { value }
    }
}
impl From<Delete> for StmtKind {
    fn from(ret: Delete) -> Self {
        Self::Delete(ret)
    }
}
impl IntoStmt for Delete {}
impl ParseVisitor for Delete {
    fn visit_child_exprs<E: FnMut(&Expr)>(&self, mut visitor: E) {
        visitor(&self.value);
    }
    fn visit_child_exprs_mut<E: FnMut(&mut Expr)>(&mut self, mut visitor: E) {
        visitor(&mut self.value);
    }
    fn visit_child_stmts<S: FnMut(&Stmt)>(&self, mut _visitor: S) {}
    fn visit_child_stmts_mut<S: FnMut(&mut Stmt)>(&mut self, _visitor: S) {}
}
