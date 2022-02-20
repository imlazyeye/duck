use std::iter::{Peekable};

use crate::Position;

use super::{lexer::Lexer, token::Token, utils::ParseError};

pub(super) struct TokenPilot<'a> {
    lexer: Peekable<Lexer<'a>>,
    cursor: usize,
}
impl<'a> TokenPilot<'a> {
    pub fn new(source_code: &'a str) -> Self {
        let lexer = Lexer::new(source_code).peekable();
        Self { lexer, cursor: 0 }
    }

    /// Get the gml tokens's cursor.
    pub(super) fn cursor(&self) -> usize {
        self.cursor
    }

    /// Returns the type of the next Token, or returns an error if there is none.
    pub fn peek(&mut self) -> Result<&Token, ParseError> {
        if let Some((_, token)) = self.lexer.peek() {
            Ok(token)
        } else {
            Err(ParseError::UnexpectedEnd(Position::default())) // todo: bad, give the pilot its own error type
        }
    }

    /// Returns the type of the next Token if there is one. Used for situations where
    /// no tokens remaining would be valid.
    pub fn soft_peek(&mut self) -> Option<&Token> {
        if let Some((_, token)) = self.lexer.peek() {
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

    /// Continously eats next token if it is the given type.
    pub fn match_take_repeating(&mut self, token: Token) {
        loop {
            if self.peek() != Ok(&token) {
                break;
            } else {
                self.take().unwrap();
            }
        }
    }

    /// Returns the next Token, returning an error if there is none, or if it is
    /// not of the required type.
    pub fn require(&mut self, token: Token) -> Result<Token, ParseError> {
        let found_token = self.take()?;
        if found_token == token {
            Ok(found_token)
        } else {
            Err(ParseError::ExpectedToken(Position::default(), token)) // same issue here
        }
    }

    /// Returns the inner field of the next Token, requiring it to be an Identifier.
    pub fn require_identifier(&mut self) -> Result<String, ParseError> {
        let next = self.take()?;
        if let Token::Identifier(v) = next {
            Ok(v)
        } else {
            Err(ParseError::UnexpectedToken(Position::default(), next)) // and here (oh no)
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
        if let Some((position, token)) = self.lexer.next() {
            self.cursor = position;
            Ok(token)
        } else {
            Err(ParseError::UnexpectedEnd(Position::default())) // okay maybe this was a mistake
        }
    }
}
