use super::Stmt;

/// A collection of statements.
#[derive(Debug, Clone)]
pub struct Ast {
    stmts: Vec<Stmt>,
}
impl Ast {
    /// Creates a new Ast with the given statements.
    pub fn new(stmts: Vec<Stmt>) -> Self {
        Self { stmts }
    }

    /// Consumes the Ast into its inner collection of statements.
    pub fn unpack(self) -> Vec<Stmt> {
        self.stmts
    }

    /// Get a reference to the ast's statements.
    pub fn stmts(&self) -> &[Stmt] {
        self.stmts.as_ref()
    }

    /// Get a mutable reference to the ast's statements.
    pub fn stmts_mut(&mut self) -> &mut Vec<Stmt> {
        &mut self.stmts
    }
}

/// An identifier for an individual Ast.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
pub struct AstId(u64);