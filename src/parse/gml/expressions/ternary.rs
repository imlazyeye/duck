use crate::parse::{Expression, ExpressionBox, IntoExpressionBox, ParseVisitor, StatementBox};

/// Representation of a ternary evaluation in gml.
#[derive(Debug, PartialEq, Clone)]
pub struct Ternary {
    /// The left hand side of the evaluation.
    pub condition: ExpressionBox,
    /// The expression yielded if the condition is true.
    pub true_value: ExpressionBox,
    /// The expression yielded if the condition is false.
    pub false_value: ExpressionBox,
}
impl Ternary {
    /// Creates a new ternary.
    pub fn new(condition: ExpressionBox, true_value: ExpressionBox, false_value: ExpressionBox) -> Self {
        Self {
            condition,
            true_value,
            false_value,
        }
    }
}
impl From<Ternary> for Expression {
    fn from(ternary: Ternary) -> Self {
        Self::Ternary(ternary)
    }
}
impl IntoExpressionBox for Ternary {}
impl ParseVisitor for Ternary {
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, mut visitor: E) {
        visitor(&self.condition);
        visitor(&self.true_value);
        visitor(&self.false_value);
    }
    fn visit_child_expressions_mut<E: FnMut(&mut ExpressionBox)>(&mut self, mut visitor: E) {
        visitor(&mut self.condition);
        visitor(&mut self.true_value);
        visitor(&mut self.false_value);
    }
    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, mut _visitor: S) {}
    fn visit_child_statements_mut<S: FnMut(&mut StatementBox)>(&mut self, _visitor: S) {}
}
