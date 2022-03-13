use crate::parse::{Expr, ExprType, IntoExpr, ParseVisitor, Stmt};

/// Representation of a ternary evaluation in gml.
#[derive(Debug, PartialEq, Clone)]
pub struct Ternary {
    /// The left hand side of the evaluation.
    pub condition: Expr,
    /// The expression yielded if the condition is true.
    pub true_value: Expr,
    /// The expression yielded if the condition is false.
    pub false_value: Expr,
}
impl Ternary {
    /// Creates a new ternary.
    pub fn new(condition: Expr, true_value: Expr, false_value: Expr) -> Self {
        Self {
            condition,
            true_value,
            false_value,
        }
    }
}
impl From<Ternary> for ExprType {
    fn from(ternary: Ternary) -> Self {
        Self::Ternary(ternary)
    }
}
impl IntoExpr for Ternary {}
impl ParseVisitor for Ternary {
    fn visit_child_exprs<E: FnMut(&Expr)>(&self, mut visitor: E) {
        visitor(&self.condition);
        visitor(&self.true_value);
        visitor(&self.false_value);
    }
    fn visit_child_exprs_mut<E: FnMut(&mut Expr)>(&mut self, mut visitor: E) {
        visitor(&mut self.condition);
        visitor(&mut self.true_value);
        visitor(&mut self.false_value);
    }
    fn visit_child_stmts<S: FnMut(&Stmt)>(&self, mut _visitor: S) {}
    fn visit_child_stmts_mut<S: FnMut(&mut Stmt)>(&mut self, _visitor: S) {}
}
