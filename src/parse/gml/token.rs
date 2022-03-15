#![allow(missing_docs)]

use crate::parse::Span;

use super::{
    AssignmentOperator, EqualityOperator, EvaluationOp, Literal, LogicalOperator, PostfixOperator, UnaryOperator,
};

/// A combination of a TokenType and the Span it originates from.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Token {
    pub token_type: TokenType,
    pub span: Span,
}
impl Token {
    /// Creates a new token.
    pub fn new(token_type: TokenType, span: Span) -> Self {
        Self { token_type, span }
    }

    /// Creates a new token with a default span.
    #[cfg(test)]
    pub fn lazy(token_type: TokenType) -> Self {
        Self::new(token_type, Span::default())
    }
}

impl Token {
    /// Returns a [Literal] corresponding to this Token, if possible.
    pub fn to_literal(&self) -> Option<Literal> {
        match self.token_type {
            TokenType::True => Some(Literal::True),
            TokenType::False => Some(Literal::False),
            TokenType::Undefined => Some(Literal::Undefined),
            TokenType::Noone => Some(Literal::Noone),
            TokenType::StringLiteral(lexeme) => Some(Literal::String(lexeme.to_string())),
            TokenType::Real(value) => Some(Literal::Real(value)),
            TokenType::Hex(lexeme) => Some(Literal::Hex(lexeme.to_string())),
            TokenType::MiscConstant(lexeme) => Some(Literal::Misc(lexeme.to_string())),
            _ => None,
        }
    }

    /// Returns a [EvaluationOperator] corresponding to this Token, if possible.
    pub fn as_evaluation_operator(&self) -> Option<EvaluationOp> {
        match self.token_type {
            TokenType::Plus => Some(EvaluationOp::Plus(*self)),
            TokenType::Minus => Some(EvaluationOp::Minus(*self)),
            TokenType::Slash => Some(EvaluationOp::Slash(*self)),
            TokenType::Star => Some(EvaluationOp::Star(*self)),
            TokenType::Div => Some(EvaluationOp::Div(*self)),
            TokenType::Mod | TokenType::Percent => Some(EvaluationOp::Modulo(*self)),
            TokenType::Ampersand => Some(EvaluationOp::And(*self)),
            TokenType::Pipe => Some(EvaluationOp::Or(*self)),
            TokenType::Circumflex => Some(EvaluationOp::Xor(*self)),
            TokenType::BitShiftLeft => Some(EvaluationOp::BitShiftLeft(*self)),
            TokenType::BitShiftRight => Some(EvaluationOp::BitShiftRight(*self)),
            _ => None,
        }
    }

    /// Returns a [EqualityOperator] corresponding to this Token, if possible.
    pub fn as_equality_operator(&self) -> Option<EqualityOperator> {
        match self.token_type {
            TokenType::Equal | TokenType::DoubleEqual | TokenType::ColonEqual => Some(EqualityOperator::Equal(*self)),
            TokenType::BangEqual | TokenType::GreaterThanLessThan => Some(EqualityOperator::NotEqual(*self)),
            TokenType::GreaterThan => Some(EqualityOperator::GreaterThan(*self)),
            TokenType::GreaterThanOrEqual => Some(EqualityOperator::GreaterThanOrEqual(*self)),
            TokenType::LessThan => Some(EqualityOperator::LessThan(*self)),
            TokenType::LessThanOrEqual => Some(EqualityOperator::LessThanOrEqual(*self)),
            _ => None,
        }
    }

    /// Returns a [AssignmentOperator] corresponding to this Token, if possible.
    pub fn as_assignment_operator(&self) -> Option<AssignmentOperator> {
        match self.token_type {
            TokenType::Equal => Some(AssignmentOperator::Equal(*self)),
            TokenType::PlusEqual => Some(AssignmentOperator::PlusEqual(*self)),
            TokenType::MinusEqual => Some(AssignmentOperator::MinusEqual(*self)),
            TokenType::StarEqual => Some(AssignmentOperator::StarEqual(*self)),
            TokenType::SlashEqual => Some(AssignmentOperator::SlashEqual(*self)),
            TokenType::PipeEqual => Some(AssignmentOperator::OrEqual(*self)),
            TokenType::AmpersandEqual => Some(AssignmentOperator::AndEqual(*self)),
            TokenType::CirumflexEqual => Some(AssignmentOperator::XorEqual(*self)),
            TokenType::DoubleInterrobangEquals => Some(AssignmentOperator::NullCoalecenceEqual(*self)),
            TokenType::PercentEqual => Some(AssignmentOperator::ModEqual(*self)),
            _ => None,
        }
    }

    /// Returns a [UnaryOperator] corresponding to this Token, if possible.
    pub fn as_unary_operator(&self) -> Option<UnaryOperator> {
        match self.token_type {
            TokenType::DoublePlus => Some(UnaryOperator::Increment(*self)),
            TokenType::DoubleMinus => Some(UnaryOperator::Decrement(*self)),
            TokenType::Bang | TokenType::Not => Some(UnaryOperator::Not(*self)),
            TokenType::Plus => Some(UnaryOperator::Positive(*self)),
            TokenType::Minus => Some(UnaryOperator::Negative(*self)),
            TokenType::Tilde => Some(UnaryOperator::BitwiseNot(*self)),
            _ => None,
        }
    }

    /// Returns a [PostfixOperator] corresponding to this Token, if possible.
    pub fn as_postfix_operator(&self) -> Option<PostfixOperator> {
        match self.token_type {
            TokenType::DoublePlus => Some(PostfixOperator::Increment(*self)),
            TokenType::DoubleMinus => Some(PostfixOperator::Decrement(*self)),
            _ => None,
        }
    }

    /// Returns a [LogicalOperator] corresponding to this Token, if possible.
    pub fn as_logical_operator(&self) -> Option<LogicalOperator> {
        match self.token_type {
            TokenType::And | TokenType::DoubleAmpersand => Some(LogicalOperator::And(*self)),
            TokenType::Or | TokenType::DoublePipe => Some(LogicalOperator::Or(*self)),
            TokenType::Xor => Some(LogicalOperator::Xor(*self)),
            _ => None,
        }
    }
}

/// An individual token of gml.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    Switch,
    Case,
    Break,
    Return,
    Colon,
    Dot,
    Enum,
    LeftBrace,
    RightBrace,
    LeftParenthesis,
    RightParenthesis,
    LeftSquareBracket,
    RightSquareBracket,
    Default,
    Comma,
    Ampersand,
    And,
    DoubleAmpersand,
    Or,
    DoublePipe,
    Xor,
    Circumflex,
    Equal,
    DoubleEqual,
    BangEqual,
    Function,
    Constructor,
    Exit,
    New,
    Global,
    Globalvar,
    SelfKeyword,
    Mod,
    Percent,
    PercentEqual,
    Div,
    Slash,
    Star,
    Try,
    Catch,
    With,
    True,
    False,
    Plus,
    Minus,
    Bang,
    Interrobang,
    DoubleInterrobang,
    DoubleInterrobangEquals,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Pipe,
    SemiColon,
    Hash,
    If,
    Else,
    While,
    For,
    Do,
    Until,
    Repeat,
    Var,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    DoublePlus,
    DoubleMinus,
    DollarSign,
    PipeEqual,
    AmpersandEqual,
    CirumflexEqual,
    Tilde,
    BitShiftLeft,
    BitShiftRight,
    AtSign,
    Continue,
    Static,
    Then,
    Finally,
    Undefined,
    Noone,
    Not,
    Other,
    Delete,
    Begin,
    End,
    Throw,
    ColonEqual,
    GreaterThanLessThan,
    Macro(&'static str, Option<&'static str>, &'static str),
    Comment(&'static str),
    Identifier(&'static str),
    Real(f64),
    StringLiteral(&'static str),
    LintTag(&'static str, &'static str),
    Hex(&'static str),
    MiscConstant(&'static str),
    Invalid(&'static str),
    Eof,
}
