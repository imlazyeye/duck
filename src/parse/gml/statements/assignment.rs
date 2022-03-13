use crate::parse::{Expr, IntoStmt, ParseVisitor, Stmt, StmtType, Token};

/// Representation of an assignment statement in gml.
#[derive(Debug, PartialEq, Clone)]
pub struct Assignment {
    /// The left hand side of the assignment, aka the target.
    pub left: Expr,
    /// The operator used in this assignment.
    pub operator: AssignmentOperator,
    /// The right hand side of the assignment, aka the value.
    pub right: Expr,
}
impl Assignment {
    /// Creates a new assignment.
    pub fn new(left: Expr, operator: AssignmentOperator, right: Expr) -> Self {
        Self { left, operator, right }
    }
}
impl From<Assignment> for StmtType {
    fn from(assignment: Assignment) -> Self {
        Self::Assignment(assignment)
    }
}
impl IntoStmt for Assignment {}
impl ParseVisitor for Assignment {
    fn visit_child_exprs<E: FnMut(&Expr)>(&self, mut visitor: E) {
        visitor(&self.left);
        visitor(&self.right);
    }
    fn visit_child_exprs_mut<E: FnMut(&mut Expr)>(&mut self, mut visitor: E) {
        visitor(&mut self.left);
        visitor(&mut self.right);
    }
    fn visit_child_stmts<S: FnMut(&Stmt)>(&self, mut _visitor: S) {}
    fn visit_child_stmts_mut<S: FnMut(&mut Stmt)>(&mut self, _visitor: S) {}
}

/// The various assignment operations supported in gml.
#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(clippy::enum_variant_names)]
pub enum AssignmentOperator {
    /// =
    Equal(Token),
    /// +=
    PlusEqual(Token),
    /// -=
    MinusEqual(Token),
    /// *=
    StarEqual(Token),
    /// /=
    SlashEqual(Token),
    /// ^=
    XorEqual(Token),
    /// |=
    OrEqual(Token),
    /// &=
    AndEqual(Token),
    /// ??=
    NullCoalecenceEqual(Token),
    /// %=
    ModEqual(Token),
}
impl AssignmentOperator {
    /// Returns the inner token.
    pub fn token(&self) -> &Token {
        match self {
            AssignmentOperator::Equal(token) => token,
            AssignmentOperator::PlusEqual(token) => token,
            AssignmentOperator::MinusEqual(token) => token,
            AssignmentOperator::StarEqual(token) => token,
            AssignmentOperator::SlashEqual(token) => token,
            AssignmentOperator::XorEqual(token) => token,
            AssignmentOperator::OrEqual(token) => token,
            AssignmentOperator::AndEqual(token) => token,
            AssignmentOperator::NullCoalecenceEqual(token) => token,
            AssignmentOperator::ModEqual(token) => token,
        }
    }
}
