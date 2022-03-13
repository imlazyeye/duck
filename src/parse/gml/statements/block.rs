use crate::parse::{Expr, IntoStmt, ParseVisitor, Stmt, StmtType, Token};

/// Representation of a block (group of statements) in gml.
///
/// Currently only describes blocks of statements that are
/// surrounded in braces, but its definition may be expanded in the future.
#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    /// The statements contained in this block.
    pub body: Vec<Stmt>,
    /// The delimiter style of this block.
    pub delimiters: Option<(Token, Token)>,
}
impl Block {
    /// Creates a new block.
    pub fn new(body: Vec<Stmt>, delimiters: Option<(Token, Token)>) -> Self {
        Self { body, delimiters }
    }

    /// Creates a new block with lazy, curly brace delimiters.
    #[cfg(test)]
    pub fn lazy(body: impl Into<Vec<Stmt>>) -> Self {
        use crate::parse::TokenType;
        Self::new(
            body.into(),
            Some((Token::lazy(TokenType::LeftBrace), Token::lazy(TokenType::RightBrace))),
        )
    }
}
impl From<Block> for StmtType {
    fn from(block: Block) -> Self {
        Self::Block(block)
    }
}
impl IntoStmt for Block {}
impl ParseVisitor for Block {
    fn visit_child_exprs<E: FnMut(&Expr)>(&self, mut _visitor: E) {}
    fn visit_child_exprs_mut<E: FnMut(&mut Expr)>(&mut self, _visitor: E) {}
    fn visit_child_stmts_mut<S: FnMut(&mut Stmt)>(&mut self, mut visitor: S) {
        for stmt in self.body.iter_mut() {
            visitor(stmt);
        }
    }
    fn visit_child_stmts<S: FnMut(&Stmt)>(&self, mut visitor: S) {
        for stmt in self.body.iter() {
            visitor(stmt);
        }
    }
}
