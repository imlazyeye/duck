use crate::prelude::{ExpressionBox, IntoStatementBox, Statement, StatementBox};

/// Representation of a try/catch/finally block in gml.
#[derive(Debug, PartialEq, Clone)]
pub struct TryCatch {
    pub try_body: StatementBox,
    pub catch_expression: ExpressionBox,
    pub catch_body: StatementBox,
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
        Statement::TryCatch(try_catch)
    }
}
impl IntoStatementBox for TryCatch {}
