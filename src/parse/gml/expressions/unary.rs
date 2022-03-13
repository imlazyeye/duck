use crate::parse::{Expr, ExprType, IntoExpr, ParseVisitor, Stmt, Token};

/// Representation of a unary operation in gml.
#[derive(Debug, PartialEq, Clone)]
pub struct Unary {
    /// The unary operator.
    pub operator: UnaryOperator,
    /// The right hand side of the unary operation.
    pub right: Expr,
}
impl Unary {
    /// Creates a new unary operation.
    pub fn new(operator: UnaryOperator, right: Expr) -> Self {
        Self { operator, right }
    }
}
impl From<Unary> for ExprType {
    fn from(unary: Unary) -> Self {
        Self::Unary(unary)
    }
}
impl IntoExpr for Unary {}
impl ParseVisitor for Unary {
    fn visit_child_exprs<E: FnMut(&Expr)>(&self, mut visitor: E) {
        visitor(&self.right);
    }
    fn visit_child_exprs_mut<E: FnMut(&mut Expr)>(&mut self, mut visitor: E) {
        visitor(&mut self.right);
    }
    fn visit_child_stmts<S: FnMut(&Stmt)>(&self, mut _visitor: S) {}
    fn visit_child_stmts_mut<S: FnMut(&mut Stmt)>(&mut self, _visitor: S) {}
}

/// The various unary operations supported in gml.
#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    /// ++
    Increment(Token),
    /// --
    Decrement(Token),
    /// not, !
    Not(Token),
    /// +
    Positive(Token),
    /// -
    Negative(Token),
    /// ~
    BitwiseNot(Token),
}
