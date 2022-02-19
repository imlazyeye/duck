use super::{expression::ExpressionBox, token::Token};

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedToken(usize, Token),
    ExpectedToken(usize, Token),
    InvalidLintLevel(usize, String),
    InvalidAssignmentTarget(usize, ExpressionBox),
    InvalidNewTarget(usize, ExpressionBox),
    UnexpectedEnd,
}
