use crate::prelude::{ExpressionBox, IntoStatementBox, ParseVisitor, Statement, StatementBox};

/// Representation of an if statement in gml.
///
/// I'm aware that its absolutely chaotic that I named this thing `If`.
#[derive(Debug, PartialEq, Clone)]
pub struct If {
    /// The condition this if statement is checking for.
    pub condition: ExpressionBox,
    /// The body of the if statement.
    pub body: StatementBox,
    /// The statement attached to this if statement as an else path.
    pub else_statement: Option<StatementBox>,
    /// Whether or not this if statement uses gml's dreaded `then` keyword.
    ///
    /// For the uninitiated:
    /// ```gml
    /// if foo then {
    ///     //
    /// }
    /// ```
    /// The above is valid gml. The keyword does absolutely nothing.
    pub uses_then_keyword: bool,
}
impl If {
    /// Creates a new if statement.
    pub fn new(condition: ExpressionBox, body: StatementBox) -> Self {
        Self {
            condition,
            body,
            else_statement: None,
            uses_then_keyword: false,
        }
    }

    /// Creates a new if statement with an else statement.
    pub fn new_with_else(condition: ExpressionBox, body: StatementBox, else_statement: StatementBox) -> Self {
        Self {
            condition,
            body,
            else_statement: Some(else_statement),
            uses_then_keyword: false,
        }
    }

    /// Creates a new if statement that uses a `then` keyword.
    pub fn new_with_then_keyword(
        condition: ExpressionBox,
        body: StatementBox,
        else_statement: Option<StatementBox>,
    ) -> Self {
        Self {
            condition,
            body,
            else_statement,
            uses_then_keyword: true,
        }
    }
}
impl From<If> for Statement {
    fn from(if_stmt: If) -> Self {
        Self::If(if_stmt)
    }
}
impl IntoStatementBox for If {}
impl ParseVisitor for If {
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, mut expression_visitor: E) {
        expression_visitor(&self.condition);
    }

    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, mut statement_visitor: S) {
        statement_visitor(&self.body);
        if let Some(else_statement) = &self.else_statement {
            statement_visitor(else_statement);
        }
    }
}
