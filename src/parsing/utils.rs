use colored::Colorize;

use crate::Position;

use super::{expression::ExpressionBox, token::Token};

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedToken(Position, Token),
    ExpectedToken(Position, Token),
    InvalidLintLevel(Position, String),
    InvalidAssignmentTarget(Position, ExpressionBox),
    InvalidNewTarget(Position, ExpressionBox),
    IncompleteStatement(Position, ExpressionBox),
    UnexpectedEnd(Position),
}
impl ParseError {
    pub fn position(&self) -> &Position {
        match self {
            ParseError::UnexpectedToken(position, _) => position,
            ParseError::ExpectedToken(position, _) => position,
            ParseError::InvalidLintLevel(position, _) => position,
            ParseError::InvalidAssignmentTarget(position, _) => position,
            ParseError::InvalidNewTarget(position, _) => position,
            ParseError::IncompleteStatement(position, _) => position,
            ParseError::UnexpectedEnd(position) => position,
        }
    }

    fn error_message(&self) -> String {
        match self {
            ParseError::UnexpectedToken(_, token) => format!("Unexpected token: {:?}", token),
            ParseError::ExpectedToken(_, token) => format!("Expected token: {:?}", token),
            ParseError::InvalidLintLevel(_, level) => format!("Invalid lint level: {}", level),
            ParseError::InvalidAssignmentTarget(_, _) => "Invalid assignment target".into(),
            ParseError::InvalidNewTarget(_, _) => "Invalid use of `new`".into(),
            ParseError::IncompleteStatement(_, _) => "Incomplete statement".into(),
            ParseError::UnexpectedEnd(_) => "Unexpected end".into(),
        }
    }
}
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path_message = self.position().path_message();
        let snippet_message = self.position().snippet_message();
        f.pad(&format!(
            "{}: {}\n{path_message}\n{snippet_message}\n",
            "parse error".bright_red().bold(),
            self.error_message().bright_white(),
        ))
    }
}
