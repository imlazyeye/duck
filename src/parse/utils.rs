use super::{ExpressionBox, StatementBox};

/// Used to visit the children of a Statement/Expression as we recurse down the tree.
pub trait ParseVisitor {
    /// Visits all expressions this T contains.
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, visitor: E);
    /// Visits all expressions this T contains mutably.
    fn visit_child_expressions_mut<E: FnMut(&mut ExpressionBox)>(&mut self, visitor: E);
    /// Visits all statements this T contains.
    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, visitor: S);
    /// Visits all statements this T contains mutably.
    fn visit_child_statements_mut<S: FnMut(&mut StatementBox)>(&mut self, visitor: S);
}
