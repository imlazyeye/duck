use crate::parse::{Expr, IntoStmt, ParseVisitor, Stmt, StmtKind};

/// A return statement, with an optional return value.
#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub struct Return {
    /// The value, if any, that this statement returns.
    pub value: Option<Expr>,
}
impl Return {
    /// Creates a new return statement with an optional value.
    pub fn new(value: Option<Expr>) -> Self {
        Self { value }
    }
}
impl From<Return> for StmtKind {
    fn from(ret: Return) -> Self {
        Self::Return(ret)
    }
}
impl IntoStmt for Return {}
impl ParseVisitor for Return {
    fn visit_child_exprs<E: FnMut(&Expr)>(&self, mut visitor: E) {
        if let Some(value) = &self.value {
            visitor(value);
        }
    }
    fn visit_child_exprs_mut<E: FnMut(&mut Expr)>(&mut self, mut visitor: E) {
        if let Some(value) = &mut self.value {
            visitor(value);
        }
    }
    fn visit_child_stmts<S: FnMut(&Stmt)>(&self, mut _visitor: S) {}
    fn visit_child_stmts_mut<S: FnMut(&mut Stmt)>(&mut self, _visitor: S) {}
}
