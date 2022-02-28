use super::{expression::ExpressionBox, token::Token};
use crate::utils::{FilePreviewUtil, Span};
use colored::Colorize;

/// The various errors that can be encountered when we parse Gml.
///
/// It is worth noting that duck is not presently interested with asserting the
/// validity of Gml. In other words, it's okay if duck runs successfully over
/// Gml that would not compile in GameMaker. The reverse is not true -- if
/// something is valid Gml, duck should be able to parse it, with very few
/// exceptions (only made for absolutley absurd syntax that would have a
/// significant impact on duck's codebase. )
///
/// As such, the only errors we support are out of neccesity -- errors that
/// prevent us from being able to safely (or meaningfully) continue through the
/// user's Gml.
#[derive(Debug, PartialEq, Clone)]
pub enum ParseError {
    /// A token was encountered that was not expected in the current context.
    UnexpectedToken(Span, Token),
    /// A token was required in the current context that was not found.
    ExpectedToken(Span, Token),
    /// An assignment was made to an invalid expression.
    ///
    /// ### Example
    /// ```gml
    /// !foo = 0; // `!foo` yields a value (boolean), and values cannot be assigned to.
    /// ```
    ///
    /// While this is asserting the validity of Gml, adding a catch to it often
    /// helps us detect our own issues in duck.
    InvalidAssignmentTarget(Span, ExpressionBox),
    /// A only contains an expression that can not be a statement on its own.
    ///
    /// Similar to the [ParseError::InvalidAssignmentTarget], this is asserting
    /// validity of gml, but is useful for detecting our own mistakes.
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
            ParseError::InvalidAssignmentTarget(_, _) => "Invalid assignment target".into(),
            ParseError::IncompleteStatement(_, _) => "Incomplete statement".into(),
            ParseError::UnexpectedEnd(_) => "Unexpected end".into(),
        }
    }
}
