use crate::parse::{ExpressionBox, IntoStatementBox, ParseVisitor, Statement, StatementBox, Token};

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
        use crate::parse::TokenType;
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
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, mut _visitor: E) {}
    fn visit_child_expressions_mut<E: FnMut(&mut ExpressionBox)>(&mut self, _visitor: E) {}
    fn visit_child_statements_mut<S: FnMut(&mut StatementBox)>(&mut self, mut visitor: S) {
        for statement in self.body.iter_mut() {
            visitor(statement);
        }
    }
    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, mut visitor: S) {
        for statement in self.body.iter() {
            visitor(statement);
        }
    }
}
