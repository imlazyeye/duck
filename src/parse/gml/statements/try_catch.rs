use crate::parse::{Expr, IntoStmt, ParseVisitor, Stmt, StmtKind};

/// Representation of a try/catch/finally block in gml.
#[derive(Debug, PartialEq, Clone)]
pub struct TryCatch {
    /// The statement to try.
    pub try_body: Stmt,
    /// The capture of the error in the catch.
    pub catch_expr: Expr,
    /// The statement to run on catch.
    pub catch_body: Stmt,
    /// The finally body, if any.
    pub finally_body: Option<Stmt>,
}
impl TryCatch {
    /// Creates a new try/catch.
    pub fn new(try_body: Stmt, catch_expr: Expr, catch_body: Stmt) -> Self {
        Self {
            try_body,
            catch_expr,
            catch_body,
            finally_body: None,
        }
    }

    /// Creates a new try/catch with a finally block.
    pub fn new_with_finally(try_body: Stmt, catch_expr: Expr, catch_body: Stmt, finally_body: Stmt) -> Self {
        Self {
            try_body,
            catch_expr,
            catch_body,
            finally_body: Some(finally_body),
        }
    }
}
impl From<TryCatch> for StmtKind {
    fn from(try_catch: TryCatch) -> Self {
        Self::TryCatch(try_catch)
    }
}
impl IntoStmt for TryCatch {}
impl ParseVisitor for TryCatch {
    fn visit_child_exprs<E: FnMut(&Expr)>(&self, mut visitor: E) {
        visitor(&self.catch_expr);
    }
    fn visit_child_exprs_mut<E: FnMut(&mut Expr)>(&mut self, mut visitor: E) {
        visitor(&mut self.catch_expr);
    }
    fn visit_child_stmts<S: FnMut(&Stmt)>(&self, mut visitor: S) {
        visitor(&self.try_body);
        visitor(&self.catch_body);
        if let Some(finally_stmt) = &self.finally_body {
            visitor(finally_stmt);
        }
    }
    fn visit_child_stmts_mut<S: FnMut(&mut Stmt)>(&mut self, mut visitor: S) {
        visitor(&mut self.try_body);
        visitor(&mut self.catch_body);
        if let Some(finally_stmt) = &mut self.finally_body {
            visitor(finally_stmt);
        }
    }
}
