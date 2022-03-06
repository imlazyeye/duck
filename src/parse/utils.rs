use super::{ExpressionBox, StatementBox};

/// Used to visit the children of a Statement/Expression as we recurse down the tree.
pub trait ParseVisitor {
    /// Visits all expressions this T contains.
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, expression_visitor: E);

    /// Visits all statements this T contains.
    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, statement_visitor: S);
}

/// Used for unit tests to assert that items are equal while ignoring their location information.
pub trait LooseEq {
    /// Returns if the two items are loosely equal, ignoring fields that are not important to the
    /// comparision.
    fn loose_eq(&self, other: &Self) -> bool;
}
