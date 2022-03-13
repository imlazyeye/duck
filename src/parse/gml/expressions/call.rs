use crate::parse::{Expr, ExprType, IntoExpr, ParseVisitor, Stmt};

/// Representation of an assignment expression in gml.
#[derive(Debug, PartialEq, Clone)]
pub struct Call {
    /// The leftside of the call (the value being invoked).
    pub left: Expr,
    /// The arguments passed into this call.
    pub arguments: Vec<Expr>,
    /// Whether or not the `new` operator is present.
    pub uses_new: bool,
}
impl Call {
    /// Creates a new call.
    pub fn new(left: Expr, arguments: Vec<Expr>) -> Self {
        Self {
            left,
            arguments,
            uses_new: false,
        }
    }

    /// Creates a new call for a constructor (using the `new` operator).
    pub fn new_with_new_operator(left: Expr, arguments: Vec<Expr>) -> Self {
        Self {
            left,
            arguments,
            uses_new: true,
        }
    }
}
impl From<Call> for ExprType {
    fn from(call: Call) -> Self {
        Self::Call(call)
    }
}
impl IntoExpr for Call {}
impl ParseVisitor for Call {
    fn visit_child_exprs<E: FnMut(&Expr)>(&self, mut visitor: E) {
        visitor(&self.left);
        for arg in &self.arguments {
            visitor(arg);
        }
    }
    fn visit_child_exprs_mut<E: FnMut(&mut Expr)>(&mut self, mut visitor: E) {
        visitor(&mut self.left);
        for arg in &mut self.arguments {
            visitor(arg);
        }
    }
    fn visit_child_stmts<S: FnMut(&Stmt)>(&self, mut _visitor: S) {}
    fn visit_child_stmts_mut<S: FnMut(&mut Stmt)>(&mut self, _visitor: S) {}
}
