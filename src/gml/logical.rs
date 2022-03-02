use crate::prelude::{Expression, ExpressionBox, IntoExpressionBox, ParseVisitor, StatementBox};

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
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, mut expression_visitor: E) {
        expression_visitor(&self.left);
        expression_visitor(&self.right);
    }
    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, _statement_visitor: S) {}
}

/// The various logical operations supported in gml.
///
/// TODO: Add the actual token
#[derive(Debug, PartialEq, Clone)]
pub enum LogicalOperator {
    /// and, &&
    And,
    /// or, ||
    Or,
    /// xor, ^^
    Xor,
}
