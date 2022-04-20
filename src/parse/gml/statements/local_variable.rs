use crate::parse::{Expr, IntoStmt, OptionalInitilization, ParseVisitor, Stmt, StmtKind};

/// Representation of a local variable declaration.
///
/// Slightly more complicated than other types due to how many local variables can be declared in
/// one swoop.
#[derive(Debug, PartialEq, Clone)]
pub struct LocalVariableSeries {
    /// The various declarations in this series.
    pub declarations: Vec<OptionalInitilization>,
}
impl LocalVariableSeries {
    /// Creates a new local variable series.
    pub fn new(declarations: Vec<OptionalInitilization>) -> Self {
        Self { declarations }
    }
}
impl From<LocalVariableSeries> for StmtKind {
    fn from(series: LocalVariableSeries) -> Self {
        Self::LocalVariableSeries(series)
    }
}
impl IntoStmt for LocalVariableSeries {}
impl ParseVisitor for LocalVariableSeries {
    fn visit_child_exprs<E: FnMut(&Expr)>(&self, mut visitor: E) {
        for declaration in self.declarations.iter() {
            match declaration {
                OptionalInitilization::Uninitialized(expr) => visitor(expr),
                OptionalInitilization::Initialized(_) => {}
            }
        }
    }
    fn visit_child_exprs_mut<E: FnMut(&mut Expr)>(&mut self, mut visitor: E) {
        for declaration in self.declarations.iter_mut() {
            match declaration {
                OptionalInitilization::Uninitialized(expr) => visitor(expr),
                OptionalInitilization::Initialized(_) => {}
            }
        }
    }
    fn visit_child_stmts<S: FnMut(&Stmt)>(&self, mut visitor: S) {
        for declaration in self.declarations.iter() {
            match declaration {
                OptionalInitilization::Uninitialized(_) => {}
                OptionalInitilization::Initialized(stmt) => visitor(stmt),
            }
        }
    }
    fn visit_child_stmts_mut<S: FnMut(&mut Stmt)>(&mut self, mut visitor: S) {
        for declaration in self.declarations.iter_mut() {
            match declaration {
                OptionalInitilization::Uninitialized(_) => {}
                OptionalInitilization::Initialized(stmt) => visitor(stmt),
            }
        }
    }
}
