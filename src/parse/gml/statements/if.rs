use crate::parse::{Expr, IntoStmt, ParseVisitor, Stmt, StmtKind};

/// Representation of an if statement in gml.
///
/// I'm aware that its absolutely chaotic that I named this thing `If`.
#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub struct If {
    /// The condition this if statement is checking for.
    pub condition: Expr,
    /// The body of the if statement.
    pub body: Stmt,
    /// The statement attached to this if statement as an else path.
    pub else_stmt: Option<Stmt>,
    /// Whether or not this if statement uses gml's dreaded `then` keyword.
    ///
    /// For the uninitiated:
    /// ```gml
    /// if foo then {
    ///     //
    /// }
    /// ```
    /// The above is valid gml. The keyword does absolutely nothing.
    #[serde(skip)]
    pub uses_then_keyword: bool,
}
impl If {
    /// Creates a new if statement.
    pub fn new(condition: Expr, body: Stmt) -> Self {
        Self {
            condition,
            body,
            else_stmt: None,
            uses_then_keyword: false,
        }
    }

    /// Creates a new if statement with an else statement.
    pub fn new_with_else(condition: Expr, body: Stmt, else_stmt: Stmt) -> Self {
        Self {
            condition,
            body,
            else_stmt: Some(else_stmt),
            uses_then_keyword: false,
        }
    }

    /// Creates a new if statement that uses a `then` keyword.
    pub fn new_with_then_keyword(condition: Expr, body: Stmt, else_stmt: Option<Stmt>) -> Self {
        Self {
            condition,
            body,
            else_stmt,
            uses_then_keyword: true,
        }
    }
}
impl From<If> for StmtKind {
    fn from(if_stmt: If) -> Self {
        Self::If(if_stmt)
    }
}
impl IntoStmt for If {}
impl ParseVisitor for If {
    fn visit_child_exprs<E: FnMut(&Expr)>(&self, mut visitor: E) {
        visitor(&self.condition);
    }
    fn visit_child_exprs_mut<E: FnMut(&mut Expr)>(&mut self, mut visitor: E) {
        visitor(&mut self.condition);
    }
    fn visit_child_stmts<S: FnMut(&Stmt)>(&self, mut visitor: S) {
        visitor(&self.body);
        if let Some(else_stmt) = &self.else_stmt {
            visitor(else_stmt);
        }
    }
    fn visit_child_stmts_mut<S: FnMut(&mut Stmt)>(&mut self, mut visitor: S) {
        visitor(&mut self.body);
        if let Some(else_stmt) = &mut self.else_stmt {
            visitor(else_stmt);
        }
    }
}
