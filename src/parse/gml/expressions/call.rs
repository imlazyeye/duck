use crate::parse::{Expression, ExpressionBox, IntoExpressionBox, ParseVisitor, StatementBox};

/// Representation of an assignment expression in gml.
#[derive(Debug, PartialEq, Clone)]
pub struct Call {
    /// The leftside of the call (the value being invoked).
    pub left: ExpressionBox,
    /// The arguments passed into this call.
    pub arguments: Vec<ExpressionBox>,
    /// Whether or not the `new` operator is present.
    pub uses_new: bool,
}
impl Call {
    /// Creates a new call.
    pub fn new(left: ExpressionBox, arguments: Vec<ExpressionBox>) -> Self {
        Self {
            left,
            arguments,
            uses_new: false,
        }
    }

    /// Creates a new call for a constructor (using the `new` operator).
    pub fn new_with_new_operator(left: ExpressionBox, arguments: Vec<ExpressionBox>) -> Self {
        Self {
            left,
            arguments,
            uses_new: true,
        }
    }
}
impl From<Call> for Expression {
    fn from(call: Call) -> Self {
        Self::Call(call)
    }
}
impl IntoExpressionBox for Call {}
impl ParseVisitor for Call {
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, mut expression_visitor: E) {
        expression_visitor(&self.left);
        for arg in &self.arguments {
            expression_visitor(arg);
        }
    }
    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, _statement_visitor: S) {}
}
