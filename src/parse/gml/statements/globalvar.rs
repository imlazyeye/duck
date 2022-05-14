use crate::parse::{Expr, Identifier, IntoStmt, ParseVisitor, Stmt, StmtKind};

/// Representation of a globalvar in gml.
#[derive(Debug, PartialEq, Clone)]
pub struct Globalvar {
    /// The name of the declared globalvar.
    pub name: Identifier,
}
impl Globalvar {
    /// Creates a new globalvar with the given name.
    pub fn new(name: Identifier) -> Self {
        Self { name }
    }
}
impl From<Globalvar> for StmtKind {
    fn from(glob: Globalvar) -> Self {
        Self::Globalvar(glob)
    }
}
impl IntoStmt for Globalvar {}
impl ParseVisitor for Globalvar {
    fn visit_child_exprs<E: FnMut(&Expr)>(&self, mut _visitor: E) {}
    fn visit_child_exprs_mut<E: FnMut(&mut Expr)>(&mut self, _visitor: E) {}
    fn visit_child_stmts<S: FnMut(&Stmt)>(&self, mut _visitor: S) {}
    fn visit_child_stmts_mut<S: FnMut(&mut Stmt)>(&mut self, _visitor: S) {}
}
