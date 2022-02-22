use colored::Colorize;

use crate::Span;

use super::{expression::ExpressionBox, token::Token};

#[derive(Debug, PartialEq, Clone)]
pub enum ParseError {
    UnexpectedToken(Span, Token),
    ExpectedToken(Span, Token),
    InvalidLintLevel(Span, String),
    InvalidAssignmentTarget(Span, ExpressionBox),
    InvalidNewTarget(Span, ExpressionBox),
    IncompleteStatement(Span, ExpressionBox),
    UnexpectedEnd(Span),
}
impl ParseError {
    pub fn span(&self) -> &Span {
        match self {
            ParseError::UnexpectedToken(span, _) => span,
            ParseError::ExpectedToken(span, _) => span,
            ParseError::InvalidLintLevel(span, _) => span,
            ParseError::InvalidAssignmentTarget(span, _) => span,
            ParseError::InvalidNewTarget(span, _) => span,
            ParseError::IncompleteStatement(span, _) => span,
            ParseError::UnexpectedEnd(span) => span,
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
        // let path_message = self.span().path_message();
        // let snippet_message = self.span().snippet_message();
        f.pad(&format!(
            "{}: {}",
            "parse error".bright_red().bold(),
            self.error_message().bright_white(),
        ))
    }
}
