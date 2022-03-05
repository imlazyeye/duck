use super::{expression::ExpressionBox, token::Token};
use crate::utils::{FilePreviewUtil, Span};
use colored::Colorize;

/// The various errors that can be encountered when we parse gml.
#[derive(Debug, PartialEq, Clone)]
pub enum ParseError {
    /// A token was encountered that was not expected in the current context.
    UnexpectedToken(Span, Token),
    /// A token was required in the current context that was not found.
    ExpectedToken(Span, Token),
    /// One of a collection of tokens were required in the current context that was not found.
    ExpectedTokens(Span, Vec<Token>),
    /// An assignment was made to an invalid expression.
    ///
    /// ### Example
    /// ```gml
    /// !foo = 0; // `!foo` yields a value (boolean), and values cannot be assigned to.
    /// ```
    InvalidAssignmentTarget(Span, ExpressionBox),
    /// A only contains an expression that can not be a statement on its own.
    IncompleteStatement(Span, ExpressionBox),
    /// The source of gml ended unexpectedly before a item could be successfully
    /// parsed.
    UnexpectedEnd(Span),
}
impl ParseError {
    /// Takes a [FilePreviewUtil] to create a display-ready report of the parse
    /// error.
    pub fn generate_report(&self, preview: &FilePreviewUtil) -> String {
        let path_message = preview.path_message();
        let snippet_message = preview.snippet_message();
        format!(
            "{}: {}\n{path_message}\n{snippet_message}\n",
            "parse error".bright_red().bold(),
            self.error_message().bright_white(),
        )
    }

    /// Returns the span collected in the error.
    pub fn span(&self) -> &Span {
        match self {
            ParseError::UnexpectedToken(span, _) => span,
            ParseError::ExpectedToken(span, _) => span,
            ParseError::ExpectedTokens(span, _) => span,
            ParseError::InvalidAssignmentTarget(span, _) => span,
            ParseError::IncompleteStatement(span, _) => span,
            ParseError::UnexpectedEnd(span) => span,
        }
    }

    /// Returns a short message describing the error.
    fn error_message(&self) -> String {
        match self {
            ParseError::UnexpectedToken(_, token) => format!("Unexpected token: {:?}", token),
            ParseError::ExpectedToken(_, token) => format!("Expected token: {:?}", token),
            ParseError::ExpectedTokens(_, tokens) => format!("Expected one of the following tokens: {:?}", tokens),
            ParseError::InvalidAssignmentTarget(_, _) => "Invalid assignment target".into(),
            ParseError::IncompleteStatement(_, _) => "Incomplete statement".into(),
            ParseError::UnexpectedEnd(_) => "Unexpected end".into(),
        }
    }
}
