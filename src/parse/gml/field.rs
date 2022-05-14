use super::{Expr, Identifier, Stmt};

/// Representation of a field, such as local variables, enum fields, and function
/// parameters, which can be optionally initialized. Will either contain an Expr with an Identifier,
/// or a Stmt with an Assignment.
#[derive(Debug, PartialEq, Clone, serde::Serialize)]
#[serde(tag = "field", content = "value", rename_all = "snake_case")]
pub enum Field {
    /// Uninitialized definition, containing only their name as an identifier as an
    /// expression.
    Uninitialized(Expr),
    /// Initialized definitions containing their full assignment statement.
    Initialized(Stmt),
}
impl Field {
    /// Retrieves the ExpresionBox that contains this definitions name.
    pub fn name_expr(&self) -> &Expr {
        match self {
            Field::Uninitialized(expr) => expr,
            Field::Initialized(stmt) => &stmt.kind().as_assignment().unwrap_or_else(|| unreachable!()).left,
        }
    }
    /// Retrieves the identifier that contains this definitions name.
    pub fn name_identifier(&self) -> &Identifier {
        match self {
            Field::Uninitialized(expr) => expr.kind().as_identifier().unwrap_or_else(|| unreachable!()),
            Field::Initialized(stmt) => stmt
                .kind()
                .as_assignment()
                .unwrap_or_else(|| unreachable!())
                .left
                .kind()
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
            Field::Uninitialized(_) => None,
            Field::Initialized(stmt) => Some(&stmt.kind().as_assignment().unwrap_or_else(|| unreachable!()).right),
        }
    }
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Field::Uninitialized(_) => f.pad(self.name()),
            Field::Initialized(_) => f.pad(&format!("{} = {}", self.name(), self.assignment_value().unwrap())),
        }
    }
}
