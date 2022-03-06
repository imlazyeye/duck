use crate::parse::{ExpressionBox, IntoStatementBox, ParseVisitor, Statement, StatementBox, Token, TokenType};

/// Representation of a block (group of statements) in gml.
///
/// Currently only describes blocks of statements that are
/// surrounded in braces, but its definition may be expanded in the future.
#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    /// The statements contained in this block.
    pub body: Vec<StatementBox>,
    /// The delimiter style of this block.
    pub delimiters: Option<(Token, Token)>,
}
impl Block {
    /// Creates a new block.
    pub fn new(body: Vec<StatementBox>, delimiters: Option<(Token, Token)>) -> Self {
        Self { body, delimiters }
    }

    /// Creates a new block with lazy, curly brace delimiters.
    #[cfg(test)]
    pub fn lazy(body: impl Into<Vec<StatementBox>>) -> Self {
        Self::new(
            body.into(),
            Some((Token::lazy(TokenType::LeftBrace), Token::lazy(TokenType::RightBrace))),
        )
    }
}
impl From<Block> for Statement {
    fn from(block: Block) -> Self {
        Self::Block(block)
    }
}
impl IntoStatementBox for Block {}
impl ParseVisitor for Block {
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, _expression_visitor: E) {}

    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, mut statement_visitor: S) {
        for statement in self.body.iter() {
            statement_visitor(statement);
        }
    }
}
