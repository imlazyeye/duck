use crate::parse::{Expression, ExpressionBox, IntoExpressionBox, ParseVisitor, StatementBox, Token};

/// Representation of a postfix operation in gml.
#[derive(Debug, PartialEq, Clone)]
pub struct Postfix {
    /// The left hand side of the postfix operation.
    pub left: ExpressionBox,
    /// The postfix operator.
    pub operator: PostfixOperator,
}
impl Postfix {
    /// Creates a new postfix operation.
    pub fn new(left: ExpressionBox, operator: PostfixOperator) -> Self {
        Self { operator, left }
    }
}
impl From<Postfix> for Expression {
    fn from(postfix: Postfix) -> Self {
        Self::Postfix(postfix)
    }
}
impl IntoExpressionBox for Postfix {}
impl ParseVisitor for Postfix {
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, mut visitor: E) {
        visitor(&self.left);
    }
    fn visit_child_expressions_mut<E: FnMut(&mut ExpressionBox)>(&mut self, mut visitor: E) {
        visitor(&mut self.left);
    }
    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, mut _visitor: S) {}
    fn visit_child_statements_mut<S: FnMut(&mut StatementBox)>(&mut self, _visitor: S) {}
}

/// The various postfix operations supported in gml.
#[derive(Debug, PartialEq, Clone)]
pub enum PostfixOperator {
    /// ++
    Increment(Token),
    /// --
    Decrement(Token),
}
