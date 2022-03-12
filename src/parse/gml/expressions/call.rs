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
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, mut visitor: E) {
        visitor(&self.left);
        for arg in &self.arguments {
            visitor(arg);
        }
    }
    fn visit_child_expressions_mut<E: FnMut(&mut ExpressionBox)>(&mut self, mut visitor: E) {
        visitor(&mut self.left);
        for arg in &mut self.arguments {
            visitor(arg);
        }
    }
    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, mut _visitor: S) {}
    fn visit_child_statements_mut<S: FnMut(&mut StatementBox)>(&mut self, _visitor: S) {}
}
