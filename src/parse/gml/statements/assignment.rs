use crate::parse::{Expr, IntoStmt, ParseVisitor, Stmt, StmtKind, Token};

/// Representation of an assignment statement in gml.
#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub struct Assignment {
    /// The left hand side of the assignment, aka the target.
    pub left: Expr,
    /// The operator used in this assignment.
    pub op: AssignmentOp,
    /// The right hand side of the assignment, aka the value.
    pub right: Expr,
}
impl Assignment {
    /// Creates a new assignment.
    pub fn new(left: Expr, op: AssignmentOp, right: Expr) -> Self {
        Self { left, op, right }
    }
}
impl From<Assignment> for StmtKind {
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
#[derive(Debug, PartialEq, Copy, Clone, serde::Serialize)]
#[serde(tag = "type", content = "token", rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
pub enum AssignmentOp {
    /// =, :=
    Identity(Token),
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
impl AssignmentOp {
    /// Returns the inner token.
    pub fn token(&self) -> &Token {
        match self {
            AssignmentOp::Identity(token) => token,
            AssignmentOp::PlusEqual(token) => token,
            AssignmentOp::MinusEqual(token) => token,
            AssignmentOp::StarEqual(token) => token,
            AssignmentOp::SlashEqual(token) => token,
            AssignmentOp::XorEqual(token) => token,
            AssignmentOp::OrEqual(token) => token,
            AssignmentOp::AndEqual(token) => token,
            AssignmentOp::NullCoalecenceEqual(token) => token,
            AssignmentOp::ModEqual(token) => token,
        }
    }
}
