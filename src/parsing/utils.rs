use super::token::Token;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedToken(usize, Token),
    ExpectedToken(Token),
    InvalidLintLevel(usize, String),
    InvalidLint(usize, String),
    UnexpectedEnd,
}
