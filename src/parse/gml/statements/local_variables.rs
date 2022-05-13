use crate::parse::{Expr, IntoStmt, Field, ParseVisitor, Stmt, StmtKind};

/// Representation of a local variable declaration. Due to gml's syntax, this can include multiple
/// definitions!
#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub struct LocalVariables {
    /// The various declarations in this series.
    pub declarations: Vec<Field>,
}
impl LocalVariables {
    /// Creates a new local variable series.
    pub fn new(declarations: Vec<Field>) -> Self {
        Self { declarations }
    }
}
impl From<LocalVariables> for StmtKind {
    fn from(series: LocalVariables) -> Self {
        Self::LocalVariables(series)
    }
}
impl IntoStmt for LocalVariables {}
impl ParseVisitor for LocalVariables {
    fn visit_child_exprs<E: FnMut(&Expr)>(&self, mut visitor: E) {
        for declaration in self.declarations.iter() {
            match declaration {
                Field::Uninitialized(expr) => visitor(expr),
                Field::Initialized(_) => {}
            }
        }
    }
    fn visit_child_exprs_mut<E: FnMut(&mut Expr)>(&mut self, mut visitor: E) {
        for declaration in self.declarations.iter_mut() {
            match declaration {
                Field::Uninitialized(expr) => visitor(expr),
                Field::Initialized(_) => {}
            }
        }
    }
    fn visit_child_stmts<S: FnMut(&Stmt)>(&self, mut visitor: S) {
        for declaration in self.declarations.iter() {
            match declaration {
                Field::Uninitialized(_) => {}
                Field::Initialized(stmt) => visitor(stmt),
            }
        }
    }
    fn visit_child_stmts_mut<S: FnMut(&mut Stmt)>(&mut self, mut visitor: S) {
        for declaration in self.declarations.iter_mut() {
            match declaration {
                Field::Uninitialized(_) => {}
                Field::Initialized(stmt) => visitor(stmt),
            }
        }
    }
}
