use super::StatementBox;

/// A collection of statements.
#[derive(Debug, Clone)]
pub struct Ast {
    statements: Vec<StatementBox>,
}
impl Ast {
    /// Creates a new Ast with the given statements.
    pub fn new(statements: Vec<StatementBox>) -> Self {
        Self { statements }
    }

    /// Consumes the Ast into its inner collection of statements.
    pub fn unpack(self) -> Vec<StatementBox> {
        self.statements
    }

    /// Get a reference to the ast's statements.
    pub fn statements(&self) -> &[StatementBox] {
        self.statements.as_ref()
    }
}
