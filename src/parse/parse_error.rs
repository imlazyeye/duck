use super::{ExpressionBox, Span, Token, TokenType};
use crate::FileId;
use codespan_reporting::diagnostic::{Diagnostic, Label};

/// A [ParseError] coupled with information about where the error originated.
#[derive(Debug, PartialEq, Clone)]
pub struct ParseErrorReport {
    parse_error: ParseError,
    span: Span,
    file_id: FileId,
}
impl ParseErrorReport {
    /// Creates a new parse error report.
    pub fn new(parse_error: ParseError, span: Span, file_id: FileId) -> Self {
        Self {
            parse_error,
            span,
            file_id,
        }
    }

    /// Converts the parse error report into a diagnostic.
    pub fn diagnostic(&self) -> Diagnostic<FileId> {
        Diagnostic::error()
            .with_message(self.parse_error.error_message())
            .with_labels(vec![Label::primary(self.file_id, self.span.start()..self.span.end())])
    }
}
/// The various errors that can be encountered when we parse gml.
#[derive(Debug, PartialEq, Clone)]
pub enum ParseError {
    /// A token was encountered that was not expected in the current context.
    UnexpectedToken(Token),
    /// A token was required in the current context that was not found.
    ExpectedToken(TokenType),
    /// One of a collection of tokens were required in the current context that was not found.
    ExpectedTokens(Vec<TokenType>),
    /// An assignment was made to an invalid expression.
    ///
    /// ### Example
    /// ```gml
    /// !foo = 0; // `!foo` yields a value (boolean), and values cannot be assigned to.
    /// ```
    InvalidAssignmentTarget(ExpressionBox),
    /// A only contains an expression that can not be a statement on its own.
    IncompleteStatement(ExpressionBox),
    /// todo
    InvalidLintLevel(Token),
    /// The source of gml ended unexpectedly before a item could be successfully
    /// parsed.
    UnexpectedEnd,
}
impl ParseError {
    /// Returns a short message describing the error.
    fn error_message(&self) -> String {
        match self {
            ParseError::UnexpectedToken(token) => format!("Unexpected token: {:?}", token),
            ParseError::ExpectedToken(token) => format!("Expected token: {:?}", token),
            ParseError::ExpectedTokens(tokens) => format!("Expected one of the following tokens: {:?}", tokens),
            ParseError::InvalidAssignmentTarget(_) => "Invalid assignment target".into(),
            ParseError::IncompleteStatement(_) => "Incomplete statement".into(),
            ParseError::UnexpectedEnd => "Unexpected end".into(),
            ParseError::InvalidLintLevel(_) => "Invalid lint level".into(),
        }
    }
}
