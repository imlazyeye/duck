use super::{ExpressionBox, StatementBox};

/// Assignment.
/// Representation of an optional initilization, such as local variables, enum fields, and function
/// parameters. Will either contain an ExpressionBox with an Identifier, or a StatementBox with an
#[derive(Debug, PartialEq, Clone)]
pub enum OptionalInitilization {
    /// Uninitialized definition, containing only their name as an identifier as an
    /// expression.
    Uninitialized(ExpressionBox),
    /// Initialized definitions containing their full assignment statement.
    Initialized(StatementBox),
}
impl OptionalInitilization {
    /// Retrieves the expression box that contains this definitions name.
    pub fn name_expression(&self) -> &ExpressionBox {
        match self {
            OptionalInitilization::Uninitialized(expression_box) => expression_box,
            OptionalInitilization::Initialized(statement_box) => {
                &statement_box
                    .statement()
                    .as_assignment()
                    .unwrap_or_else(|| unreachable!())
                    .left
            }
        }
    }
    /// Retrieves the name of the definition.
    pub fn name(&self) -> &str {
        self.name_expression()
            .expression()
            .as_identifier()
            .unwrap_or_else(|| unreachable!())
            .lexeme
            .as_str()
    }
    /// Retrieves the right-side expression in the assignment, if there is any assignment
    pub fn assignment_value(&self) -> Option<&ExpressionBox> {
        match self {
            OptionalInitilization::Uninitialized(_) => None,
            OptionalInitilization::Initialized(statement_box) => Some(
                &statement_box
                    .statement()
                    .as_assignment()
                    .unwrap_or_else(|| unreachable!())
                    .right,
            ),
        }
    }
}
