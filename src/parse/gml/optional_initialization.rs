use super::{Expr, Identifier, Stmt};

/// Assignment.
/// Representation of an optional initilization, such as local variables, enum fields, and function
/// parameters. Will either contain an Expr with an Identifier, or a Stmt with an
#[derive(Debug, PartialEq, Clone)]
pub enum OptionalInitilization {
    /// Uninitialized definition, containing only their name as an identifier as an
    /// expression.
    Uninitialized(Expr),
    /// Initialized definitions containing their full assignment statement.
    Initialized(Stmt),
}
impl OptionalInitilization {
    /// Retrieves the ExpresionBox that contains this definitions name.
    pub fn name_expr(&self) -> &Expr {
        match self {
            OptionalInitilization::Uninitialized(expr) => expr,
            OptionalInitilization::Initialized(stmt) => {
                &stmt.inner().as_assignment().unwrap_or_else(|| unreachable!()).left
            }
        }
    }
    /// Retrieves the identifier that contains this definitions name.
    pub fn name_identifier(&self) -> &Identifier {
        match self {
            OptionalInitilization::Uninitialized(expr) => {
                expr.inner().as_identifier().unwrap_or_else(|| unreachable!())
            }
            OptionalInitilization::Initialized(stmt) => stmt
                .inner()
                .as_assignment()
                .unwrap_or_else(|| unreachable!())
                .left
                .inner()
                .as_identifier()
                .unwrap_or_else(|| unreachable!()),
        }
    }
    /// Retrieves the name of the definition.
    pub fn name(&self) -> &str {
        self.name_identifier().lexeme.as_str()
    }
    /// Retrieves the right-side expression in the assignment, if there is any assignment
    pub fn assignment_value(&self) -> Option<&Expr> {
        match self {
            OptionalInitilization::Uninitialized(_) => None,
            OptionalInitilization::Initialized(stmt) => {
                Some(&stmt.inner().as_assignment().unwrap_or_else(|| unreachable!()).right)
            }
        }
    }
}

impl std::fmt::Display for OptionalInitilization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OptionalInitilization::Uninitialized(_) => f.pad(self.name()),
            OptionalInitilization::Initialized(_) => {
                f.pad(&format!("{} = {}", self.name(), self.assignment_value().unwrap()))
            }
        }
    }
}
