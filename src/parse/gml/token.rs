#![allow(missing_docs)]

use std::fmt::Display;

use crate::parse::Span;

use super::{AssignmentOp, EqualityOp, EvaluationOp, Literal, LogicalOp, PostfixOp, UnaryOp};

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

    /// Returns a [EvaluationOp] corresponding to this Token, if possible.
    pub fn as_evaluation_op(&self) -> Option<EvaluationOp> {
        match self.token_type {
            TokenType::Plus => Some(EvaluationOp::Plus(*self)),
            TokenType::Minus => Some(EvaluationOp::Minus(*self)),
            TokenType::Slash => Some(EvaluationOp::Slash(*self)),
            TokenType::Star => Some(EvaluationOp::Star(*self)),
            TokenType::Div => Some(EvaluationOp::Div(*self)),
            TokenType::Mod | TokenType::Percent => Some(EvaluationOp::Modulo(*self)),
            TokenType::Ampersand => Some(EvaluationOp::And(*self)),
            TokenType::Pipe => Some(EvaluationOp::Or(*self)),
            TokenType::Caret => Some(EvaluationOp::Xor(*self)),
            TokenType::BitShiftLeft => Some(EvaluationOp::BitShiftLeft(*self)),
            TokenType::BitShiftRight => Some(EvaluationOp::BitShiftRight(*self)),
            _ => None,
        }
    }

    /// Returns a [EqualityOp] corresponding to this Token, if possible.
    pub fn as_equality_op(&self) -> Option<EqualityOp> {
        match self.token_type {
            TokenType::Equal | TokenType::DoubleEqual | TokenType::ColonEqual => Some(EqualityOp::Equal(*self)),
            TokenType::BangEqual | TokenType::LessThanGreaterThan => Some(EqualityOp::NotEqual(*self)),
            TokenType::GreaterThan => Some(EqualityOp::GreaterThan(*self)),
            TokenType::GreaterThanOrEqual => Some(EqualityOp::GreaterThanOrEqual(*self)),
            TokenType::LessThan => Some(EqualityOp::LessThan(*self)),
            TokenType::LessThanOrEqual => Some(EqualityOp::LessThanOrEqual(*self)),
            _ => None,
        }
    }

    /// Returns a [AssignmentOp] corresponding to this Token, if possible.
    pub fn as_assignment_op(&self) -> Option<AssignmentOp> {
        match self.token_type {
            TokenType::Equal => Some(AssignmentOp::Identity(*self)),
            TokenType::PlusEqual => Some(AssignmentOp::PlusEqual(*self)),
            TokenType::MinusEqual => Some(AssignmentOp::MinusEqual(*self)),
            TokenType::StarEqual => Some(AssignmentOp::StarEqual(*self)),
            TokenType::SlashEqual => Some(AssignmentOp::SlashEqual(*self)),
            TokenType::PipeEqual => Some(AssignmentOp::OrEqual(*self)),
            TokenType::AmpersandEqual => Some(AssignmentOp::AndEqual(*self)),
            TokenType::CaretEquals => Some(AssignmentOp::XorEqual(*self)),
            TokenType::DoubleHookEquals => Some(AssignmentOp::NullCoalecenceEqual(*self)),
            TokenType::PercentEqual => Some(AssignmentOp::ModEqual(*self)),
            _ => None,
        }
    }

    /// Returns a [UnaryOp] corresponding to this Token, if possible.
    pub fn as_unary_op(&self) -> Option<UnaryOp> {
        match self.token_type {
            TokenType::DoublePlus => Some(UnaryOp::Increment(*self)),
            TokenType::DoubleMinus => Some(UnaryOp::Decrement(*self)),
            TokenType::Bang | TokenType::Not => Some(UnaryOp::Not(*self)),
            TokenType::Plus => Some(UnaryOp::Positive(*self)),
            TokenType::Minus => Some(UnaryOp::Negative(*self)),
            TokenType::Tilde => Some(UnaryOp::BitwiseNot(*self)),
            _ => None,
        }
    }

    /// Returns a [PostfixOp] corresponding to this Token, if possible.
    pub fn as_postfix_op(&self) -> Option<PostfixOp> {
        match self.token_type {
            TokenType::DoublePlus => Some(PostfixOp::Increment(*self)),
            TokenType::DoubleMinus => Some(PostfixOp::Decrement(*self)),
            _ => None,
        }
    }

    /// Returns a [LogicalOp] corresponding to this Token, if possible.
    pub fn as_logical_op(&self) -> Option<LogicalOp> {
        match self.token_type {
            TokenType::And | TokenType::DoubleAmpersand => Some(LogicalOp::And(*self)),
            TokenType::Or | TokenType::DoublePipe => Some(LogicalOp::Or(*self)),
            TokenType::Xor => Some(LogicalOp::Xor(*self)),
            _ => None,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(&self.token_type.to_string())
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
    Caret,
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
    Hook,
    DoubleHook,
    DoubleHookEquals,
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
    CaretEquals,
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
    LessThanGreaterThan,
    Macro(&'static str, Option<&'static str>, &'static str),
    Comment(&'static str),
    Identifier(&'static str),
    Real(f64),
    StringLiteral(&'static str),
    Tag(&'static str, Option<&'static str>),
    Hex(&'static str),
    MiscConstant(&'static str),
    Invalid(&'static str),
    Eof,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TokenType::Switch => "switch",
            TokenType::Case => "case",
            TokenType::Break => "break",
            TokenType::Return => "return",
            TokenType::Colon => ":",
            TokenType::Dot => ".",
            TokenType::Enum => "enum",
            TokenType::LeftBrace => "{",
            TokenType::RightBrace => "}",
            TokenType::LeftParenthesis => "(",
            TokenType::RightParenthesis => ")",
            TokenType::LeftSquareBracket => "[",
            TokenType::RightSquareBracket => "]",
            TokenType::Default => "default",
            TokenType::Comma => ",",
            TokenType::Ampersand => "&",
            TokenType::And => "and",
            TokenType::DoubleAmpersand => "&&",
            TokenType::Or => "or",
            TokenType::DoublePipe => "||",
            TokenType::Xor => "xor",
            TokenType::Caret => "^",
            TokenType::Equal => "=",
            TokenType::DoubleEqual => "==",
            TokenType::BangEqual => "!=",
            TokenType::Function => "function",
            TokenType::Constructor => "constructor",
            TokenType::Exit => "exit",
            TokenType::New => "new",
            TokenType::Global => "global",
            TokenType::Globalvar => "globalvar",
            TokenType::SelfKeyword => "self",
            TokenType::Mod => "mod",
            TokenType::Percent => "%",
            TokenType::PercentEqual => "%=",
            TokenType::Div => "div",
            TokenType::Slash => "/",
            TokenType::Star => "*",
            TokenType::Try => "try",
            TokenType::Catch => "catch",
            TokenType::With => "with",
            TokenType::True => "true",
            TokenType::False => "false",
            TokenType::Plus => "+",
            TokenType::Minus => "-",
            TokenType::Bang => "!",
            TokenType::Hook => "?",
            TokenType::DoubleHook => "??",
            TokenType::DoubleHookEquals => "??=",
            TokenType::GreaterThan => ">",
            TokenType::GreaterThanOrEqual => ">=",
            TokenType::LessThan => "<",
            TokenType::LessThanOrEqual => "<=",
            TokenType::Pipe => "|",
            TokenType::SemiColon => ";",
            TokenType::Hash => "#",
            TokenType::If => "if",
            TokenType::Else => "else",
            TokenType::While => "while",
            TokenType::For => "for",
            TokenType::Do => "do",
            TokenType::Until => "until",
            TokenType::Repeat => "repeat",
            TokenType::Var => "var",
            TokenType::PlusEqual => "+=",
            TokenType::MinusEqual => "-=",
            TokenType::StarEqual => "*=",
            TokenType::SlashEqual => "/=",
            TokenType::DoublePlus => "++",
            TokenType::DoubleMinus => "--",
            TokenType::DollarSign => "$",
            TokenType::PipeEqual => "|=",
            TokenType::AmpersandEqual => "&=",
            TokenType::CaretEquals => "^=",
            TokenType::Tilde => "~",
            TokenType::BitShiftLeft => "<<",
            TokenType::BitShiftRight => ">>",
            TokenType::AtSign => "@",
            TokenType::Continue => "continue",
            TokenType::Static => "static",
            TokenType::Then => "then",
            TokenType::Finally => "finally",
            TokenType::Undefined => "undefined",
            TokenType::Noone => "noone",
            TokenType::Not => "not",
            TokenType::Other => "other",
            TokenType::Delete => "delete",
            TokenType::Begin => "begin",
            TokenType::End => "end",
            TokenType::Throw => "throw",
            TokenType::ColonEqual => ":=",
            TokenType::LessThanGreaterThan => "<>",
            TokenType::Macro(name, config, body) => {
                return if let Some(config) = config {
                    f.pad(&format!("#macro {config}:{name} {body}"))
                } else {
                    f.pad(&format!("#macro {name} {body}"))
                };
            }
            TokenType::Comment(body) => return f.pad(&format!("// {body}")),
            TokenType::Identifier(iden) => iden,
            TokenType::Real(r) => return f.pad(&r.to_string()),
            TokenType::StringLiteral(s) => return f.pad(&format!("\"{s}\"")),
            TokenType::Tag(label, param) => {
                if let Some(param) = param {
                    return f.pad(&format!("// #[{label}({param})]"));
                } else {
                    return f.pad(&format!("// #[{label}]"));
                }
            }
            TokenType::Hex(hex) => hex,
            TokenType::MiscConstant(con) => con,
            TokenType::Invalid(_) => "INVALID_TOKEN",
            TokenType::Eof => "",
        };
        f.pad(s)
    }
}
