use std::{
    iter::{Enumerate, Peekable},
    vec::IntoIter,
};

use super::{lexer::Lexer, token::Token, utils::ParseError};

pub(super) struct TokenPilot<'a> {
    lexer: Peekable<Enumerate<Lexer<'a>>>,
    cursor: usize,
}
impl<'a> TokenPilot<'a> {
    pub fn new(source_code: &'a str) -> Self {
        let lexer = Lexer::new(source_code).enumerate().peekable();
        Self { lexer, cursor: 0 }
    }

    /// Get the gml tokens's cursor.
    pub(super) fn cursor(&self) -> usize {
        self.cursor
    }

    /// Returns the type of the next Token, or returns an error if there is none.
    pub fn peek(&mut self) -> Result<&Token, ParseError> {
        if let Some((_, (_, token))) = self.lexer.peek() {
            Ok(token)
        } else {
            Err(ParseError::UnexpectedEnd)
        }
    }

    /// Returns the type of the next Token if there is one. Used for situations where
    /// no tokens remaining would be valid.
    pub fn soft_peek(&mut self) -> Option<&Token> {
        if let Some((_, (_, token))) = self.lexer.peek() {
            Some(token)
        } else {
            None
        }
    }

    /// Consumes and returns the next token if it is the given type.
    pub fn match_take(&mut self, token: Token) -> Option<Token> {
        if self.peek() == Ok(&token) {
            Some(self.take().unwrap())
        } else {
            None
        }
    }

    /// Returns the next Token, returning an error if there is none, or if it is
    /// not of the required type.
    pub fn require(&mut self, token: Token) -> Result<Token, ParseError> {
        let found_token = self.take()?;
        if found_token == token {
            Ok(found_token)
        } else {
            Err(ParseError::ExpectedToken(token))
        }
    }

    /// Returns the inner field of the next Token, requiring it to be an Identifier.
    pub fn require_identifier(&mut self) -> Result<String, ParseError> {
        let next = self.take()?;
        if let Token::Identifier(v) = next {
            Ok(v)
        } else {
            Err(ParseError::UnexpectedToken(self.cursor, next))
        }
    }

    /// Returns the inner field of the next Token if it is an Identifier.
    pub fn match_take_identifier(&mut self) -> Result<Option<String>, ParseError> {
        if matches!(self.peek()?, Token::Identifier(_)) {
            Ok(Some(self.require_identifier()?))
        } else {
            Ok(None)
        }
    }

    /// Returns the next Token, returning an error if there is none.
    pub fn take(&mut self) -> Result<Token, ParseError> {
        if let Some((position, (_, token))) = self.lexer.next() {
            self.cursor = position;
            Ok(token)
        } else {
            Err(ParseError::UnexpectedEnd)
        }
    }

    /// Takes until it takes a token matching one passed in.
    /// Om nom nom.
    pub fn take_through(&mut self, ending_tokens: &[Token]) -> Result<Token, ParseError> {
        loop {
            match self.peek()? {
                token if ending_tokens.contains(token) => break self.take(),
                _ => {
                    self.take()?;
                }
            }
        }
    }

    /// Takes until it peeks a token matching one passed in.
    /// Om nom nom.
    pub fn take_until(&mut self, ending_tokens: &[Token]) -> Result<&Token, ParseError> {
        loop {
            match self.peek()? {
                token if ending_tokens.contains(token) => break self.peek(),
                _ => {
                    self.take()?;
                }
            }
        }
    }
}
