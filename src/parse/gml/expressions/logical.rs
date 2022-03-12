use crate::parse::{Expression, ExpressionBox, IntoExpressionBox, ParseVisitor, StatementBox, Token};

/// Representation of an logical expression in gml.
#[derive(Debug, PartialEq, Clone)]
pub struct Logical {
    /// The left hand side of the logical assessment.
    pub left: ExpressionBox,
    /// The operator used in this logical assesment..
    pub operator: LogicalOperator,
    /// The right hand side of the logical assessment.
    pub right: ExpressionBox,
}
impl Logical {
    /// Creates a new logical assessment.
    pub fn new(left: ExpressionBox, operator: LogicalOperator, right: ExpressionBox) -> Self {
        Self { left, operator, right }
    }
}
impl From<Logical> for Expression {
    fn from(logical: Logical) -> Self {
        Self::Logical(logical)
    }
}
impl IntoExpressionBox for Logical {}
impl ParseVisitor for Logical {
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, mut visitor: E) {
        visitor(&self.left);
        visitor(&self.right);
    }
    fn visit_child_expressions_mut<E: FnMut(&mut ExpressionBox)>(&mut self, mut visitor: E) {
        visitor(&mut self.left);
        visitor(&mut self.right);
    }
    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, mut _visitor: S) {}
    fn visit_child_statements_mut<S: FnMut(&mut StatementBox)>(&mut self, _visitor: S) {}
}

/// The various logical operations supported in gml.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LogicalOperator {
    /// and, &&
    And(Token),
    /// or, ||
    Or(Token),
    /// xor, ^^
    Xor(Token),
}
