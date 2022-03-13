use crate::parse::{Expr, ExprType, IntoExpr, ParseVisitor, Stmt, Token};

/// Representation of a postfix operation in gml.
#[derive(Debug, PartialEq, Clone)]
pub struct Postfix {
    /// The left hand side of the postfix operation.
    pub left: Expr,
    /// The postfix operator.
    pub operator: PostfixOperator,
}
impl Postfix {
    /// Creates a new postfix operation.
    pub fn new(left: Expr, operator: PostfixOperator) -> Self {
        Self { operator, left }
    }
}
impl From<Postfix> for ExprType {
    fn from(postfix: Postfix) -> Self {
        Self::Postfix(postfix)
    }
}
impl IntoExpr for Postfix {}
impl ParseVisitor for Postfix {
    fn visit_child_exprs<E: FnMut(&Expr)>(&self, mut visitor: E) {
        visitor(&self.left);
    }
    fn visit_child_exprs_mut<E: FnMut(&mut Expr)>(&mut self, mut visitor: E) {
        visitor(&mut self.left);
    }
    fn visit_child_stmts<S: FnMut(&Stmt)>(&self, mut _visitor: S) {}
    fn visit_child_stmts_mut<S: FnMut(&mut Stmt)>(&mut self, _visitor: S) {}
}

/// The various postfix operations supported in gml.
#[derive(Debug, PartialEq, Clone)]
pub enum PostfixOperator {
    /// ++
    Increment(Token),
    /// --
    Decrement(Token),
}
