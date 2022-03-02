use crate::parsing::{ExpressionBox, IntoStatementBox, ParseVisitor, Statement, StatementBox};

/// Representation of a try/catch/finally block in gml.
#[derive(Debug, PartialEq, Clone)]
pub struct TryCatch {
    /// The statement to try.
    pub try_body: StatementBox,
    /// The capture of the error in the catch.
    pub catch_expression: ExpressionBox,
    /// The statement to run on catch.
    pub catch_body: StatementBox,
    /// The finally body, if any.
    pub finally_body: Option<StatementBox>,
}
impl TryCatch {
    /// Creates a new try/catch.
    pub fn new(try_body: StatementBox, catch_expression: ExpressionBox, catch_body: StatementBox) -> Self {
        Self {
            try_body,
            catch_expression,
            catch_body,
            finally_body: None,
        }
    }

    /// Creates a new try/catch with a finally block.
    pub fn new_with_finally(
        try_body: StatementBox,
        catch_expression: ExpressionBox,
        catch_body: StatementBox,
        finally_body: StatementBox,
    ) -> Self {
        Self {
            try_body,
            catch_expression,
            catch_body,
            finally_body: Some(finally_body),
        }
    }
}
impl From<TryCatch> for Statement {
    fn from(try_catch: TryCatch) -> Self {
        Self::TryCatch(try_catch)
    }
}
impl IntoStatementBox for TryCatch {}
impl ParseVisitor for TryCatch {
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, mut expression_visitor: E) {
        expression_visitor(&self.catch_expression);
    }

    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, mut statement_visitor: S) {
        statement_visitor(&self.try_body);
        statement_visitor(&self.catch_body);
        if let Some(finally_stmt) = &self.finally_body {
            statement_visitor(finally_stmt);
        }
    }
}
