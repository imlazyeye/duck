use crate::prelude::{Expression, ExpressionBox, IntoExpressionBox, ParseVisitor, StatementBox};

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
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, _expression_visitor: E) {}
    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, _statement_visitor: S) {}
}

/// The various postfix operations supported in gml.
///
/// TODO: Add the actual token
#[derive(Debug, PartialEq, Clone)]
pub enum PostfixOperator {
    /// ++
    Increment,
    /// --
    Decrement,
}
