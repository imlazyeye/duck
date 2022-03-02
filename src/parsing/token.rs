#![allow(missing_docs)]

use super::{
    AssignmentOperator, EqualityOperator, EvaluationOperator, Literal, LogicalOperator, PostfixOperator, UnaryOperator,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
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
    Macro(&'static str, Option<&'static str>, &'static str),
    #[deprecated(
        note = "Comment parsing get's very tricky, very fast, and until we have a bigger need (and better solution so that it doesn't hurt our overall parsing), the [Lexer] will just discard these."
    )]
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
impl Token {
    /// Returns a [Literal] corresponding to this Token, if possible.
    pub fn to_literal(&self) -> Option<Literal> {
        match self {
            Token::True => Some(Literal::True),
            Token::False => Some(Literal::False),
            Token::Undefined => Some(Literal::Undefined),
            Token::Noone => Some(Literal::Noone),
            Token::StringLiteral(lexeme) => Some(Literal::String(lexeme.to_string())),
            Token::Real(value) => Some(Literal::Real(*value)),
            Token::Hex(lexeme) => Some(Literal::Hex(lexeme.to_string())),
            Token::MiscConstant(lexeme) => Some(Literal::Misc(lexeme.to_string())),
            _ => None,
        }
    }

    /// Returns a [EvaluationOperator] corresponding to this Token, if possible.
    pub fn as_evaluation_operator(&self) -> Option<EvaluationOperator> {
        match self {
            Token::Plus => Some(EvaluationOperator::Plus(*self)),
            Token::Minus => Some(EvaluationOperator::Minus(*self)),
            Token::Slash => Some(EvaluationOperator::Slash(*self)),
            Token::Star => Some(EvaluationOperator::Star(*self)),
            Token::Div => Some(EvaluationOperator::Div(*self)),
            Token::Mod | Token::Percent => Some(EvaluationOperator::Modulo(*self)),
            Token::Ampersand => Some(EvaluationOperator::And(*self)),
            Token::Pipe => Some(EvaluationOperator::Or(*self)),
            Token::Circumflex => Some(EvaluationOperator::Xor(*self)),
            Token::BitShiftLeft => Some(EvaluationOperator::BitShiftLeft(*self)),
            Token::BitShiftRight => Some(EvaluationOperator::BitShiftRight(*self)),
            _ => None,
        }
    }

    /// Returns a [EqualityOperator] corresponding to this Token, if possible.
    pub fn as_equality_operator(&self) -> Option<EqualityOperator> {
        match self {
            Token::Equal | Token::DoubleEqual => Some(EqualityOperator::Equal(*self)),
            Token::BangEqual => Some(EqualityOperator::NotEqual(*self)),
            Token::GreaterThan => Some(EqualityOperator::GreaterThan(*self)),
            Token::GreaterThanOrEqual => Some(EqualityOperator::GreaterThanOrEqual(*self)),
            Token::LessThan => Some(EqualityOperator::LessThan(*self)),
            Token::LessThanOrEqual => Some(EqualityOperator::LessThanOrEqual(*self)),
            _ => None,
        }
    }

    /// Returns a [AssignmentOperator] corresponding to this Token, if possible.
    pub fn as_assignment_operator(&self) -> Option<AssignmentOperator> {
        match self {
            Token::Equal => Some(AssignmentOperator::Equal(*self)),
            Token::PlusEqual => Some(AssignmentOperator::PlusEqual(*self)),
            Token::MinusEqual => Some(AssignmentOperator::MinusEqual(*self)),
            Token::StarEqual => Some(AssignmentOperator::StarEqual(*self)),
            Token::SlashEqual => Some(AssignmentOperator::SlashEqual(*self)),
            Token::PipeEqual => Some(AssignmentOperator::OrEqual(*self)),
            Token::AmpersandEqual => Some(AssignmentOperator::AndEqual(*self)),
            Token::CirumflexEqual => Some(AssignmentOperator::XorEqual(*self)),
            Token::DoubleInterrobangEquals => Some(AssignmentOperator::NullCoalecenceEqual(*self)),
            Token::PercentEqual => Some(AssignmentOperator::ModEqual(*self)),
            _ => None,
        }
    }

    /// Returns a [UnaryOperator] corresponding to this Token, if possible.
    pub fn as_unary_operator(&self) -> Option<UnaryOperator> {
        match self {
            Token::DoublePlus => Some(UnaryOperator::Increment(*self)),
            Token::DoubleMinus => Some(UnaryOperator::Decrement(*self)),
            Token::Bang | Token::Not => Some(UnaryOperator::Not(*self)),
            Token::Plus => Some(UnaryOperator::Positive(*self)),
            Token::Minus => Some(UnaryOperator::Negative(*self)),
            Token::Tilde => Some(UnaryOperator::BitwiseNot(*self)),
            _ => None,
        }
    }

    /// Returns a [PostfixOperator] corresponding to this Token, if possible.
    pub fn as_postfix_operator(&self) -> Option<PostfixOperator> {
        match self {
            Token::DoublePlus => Some(PostfixOperator::Increment(*self)),
            Token::DoubleMinus => Some(PostfixOperator::Decrement(*self)),
            _ => None,
        }
    }

    /// Returns a [LogicalOperator] corresponding to this Token, if possible.
    pub fn as_logical_operator(&self) -> Option<LogicalOperator> {
        match self {
            Token::And | Token::DoubleAmpersand => Some(LogicalOperator::And(*self)),
            Token::Or | Token::DoublePipe => Some(LogicalOperator::Or(*self)),
            Token::Xor => Some(LogicalOperator::Xor(*self)),
            _ => None,
        }
    }

    /// Returns the String in this Token if it is an identifier.
    pub fn as_identifier(&self) -> Option<&str> {
        match self {
            Token::Identifier(lexeme) => Some(lexeme),
            _ => None,
        }
    }
}
