#![allow(missing_docs)]

use std::fmt::Display;

use crate::parse::Span;

use super::{AssignmentOp, EqualityOp, EvaluationOp, Literal, LogicalOp, PostfixOp, UnaryOp};

/// A combination of a TokenType and the Span it originates from.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Token {
    pub token_type: TokenKind,
    pub span: Span,
}
impl Token {
    /// Creates a new token.
    pub fn new(token_type: TokenKind, span: Span) -> Self {
        Self { token_type, span }
    }

    /// Creates a new token with a default span.
    #[cfg(test)]
    pub fn lazy(token_type: TokenKind) -> Self {
        Self::new(token_type, Span::default())
    }
}

impl Token {
    /// Returns a [Literal] corresponding to this Token, if possible.
    pub fn to_literal(&self) -> Option<Literal> {
        match self.token_type {
            TokenKind::True => Some(Literal::True),
            TokenKind::False => Some(Literal::False),
            TokenKind::Undefined => Some(Literal::Undefined),
            TokenKind::Noone => Some(Literal::Noone),
            TokenKind::StringLiteral(lexeme) => Some(Literal::String(lexeme.to_string())),
            TokenKind::Real(value) => Some(Literal::Real(value)),
            TokenKind::Hex(lexeme) => Some(Literal::Hex(lexeme.to_string())),
            TokenKind::MiscConstant(lexeme) => Some(Literal::Misc(lexeme.to_string())),
            _ => None,
        }
    }

    /// Returns a [EvaluationOp] corresponding to this Token, if possible.
    pub fn as_evaluation_op(&self) -> Option<EvaluationOp> {
        match self.token_type {
            TokenKind::Plus => Some(EvaluationOp::Plus(*self)),
            TokenKind::Minus => Some(EvaluationOp::Minus(*self)),
            TokenKind::Slash => Some(EvaluationOp::Slash(*self)),
            TokenKind::Star => Some(EvaluationOp::Star(*self)),
            TokenKind::Div => Some(EvaluationOp::Div(*self)),
            TokenKind::Mod | TokenKind::Percent => Some(EvaluationOp::Modulo(*self)),
            TokenKind::Ampersand => Some(EvaluationOp::And(*self)),
            TokenKind::Pipe => Some(EvaluationOp::Or(*self)),
            TokenKind::Caret => Some(EvaluationOp::Xor(*self)),
            TokenKind::BitShiftLeft => Some(EvaluationOp::BitShiftLeft(*self)),
            TokenKind::BitShiftRight => Some(EvaluationOp::BitShiftRight(*self)),
            _ => None,
        }
    }

    /// Returns a [EqualityOp] corresponding to this Token, if possible.
    pub fn as_equality_op(&self) -> Option<EqualityOp> {
        match self.token_type {
            TokenKind::Equal | TokenKind::DoubleEqual | TokenKind::ColonEqual => Some(EqualityOp::Equal(*self)),
            TokenKind::BangEqual | TokenKind::LessThanGreaterThan => Some(EqualityOp::NotEqual(*self)),
            TokenKind::GreaterThan => Some(EqualityOp::GreaterThan(*self)),
            TokenKind::GreaterThanOrEqual => Some(EqualityOp::GreaterThanOrEqual(*self)),
            TokenKind::LessThan => Some(EqualityOp::LessThan(*self)),
            TokenKind::LessThanOrEqual => Some(EqualityOp::LessThanOrEqual(*self)),
            _ => None,
        }
    }

    /// Returns a [AssignmentOp] corresponding to this Token, if possible.
    pub fn as_assignment_op(&self) -> Option<AssignmentOp> {
        match self.token_type {
            TokenKind::Equal => Some(AssignmentOp::Identity(*self)),
            TokenKind::PlusEqual => Some(AssignmentOp::PlusEqual(*self)),
            TokenKind::MinusEqual => Some(AssignmentOp::MinusEqual(*self)),
            TokenKind::StarEqual => Some(AssignmentOp::StarEqual(*self)),
            TokenKind::SlashEqual => Some(AssignmentOp::SlashEqual(*self)),
            TokenKind::PipeEqual => Some(AssignmentOp::OrEqual(*self)),
            TokenKind::AmpersandEqual => Some(AssignmentOp::AndEqual(*self)),
            TokenKind::CaretEquals => Some(AssignmentOp::XorEqual(*self)),
            TokenKind::DoubleHookEquals => Some(AssignmentOp::NullCoalecenceEqual(*self)),
            TokenKind::PercentEqual => Some(AssignmentOp::ModEqual(*self)),
            _ => None,
        }
    }

    /// Returns a [UnaryOp] corresponding to this Token, if possible.
    pub fn as_unary_op(&self) -> Option<UnaryOp> {
        match self.token_type {
            TokenKind::DoublePlus => Some(UnaryOp::Increment(*self)),
            TokenKind::DoubleMinus => Some(UnaryOp::Decrement(*self)),
            TokenKind::Bang | TokenKind::Not => Some(UnaryOp::Not(*self)),
            TokenKind::Plus => Some(UnaryOp::Positive(*self)),
            TokenKind::Minus => Some(UnaryOp::Negative(*self)),
            TokenKind::Tilde => Some(UnaryOp::BitwiseNot(*self)),
            _ => None,
        }
    }

    /// Returns a [PostfixOp] corresponding to this Token, if possible.
    pub fn as_postfix_op(&self) -> Option<PostfixOp> {
        match self.token_type {
            TokenKind::DoublePlus => Some(PostfixOp::Increment(*self)),
            TokenKind::DoubleMinus => Some(PostfixOp::Decrement(*self)),
            _ => None,
        }
    }

    /// Returns a [LogicalOp] corresponding to this Token, if possible.
    pub fn as_logical_op(&self) -> Option<LogicalOp> {
        match self.token_type {
            TokenKind::And | TokenKind::DoubleAmpersand => Some(LogicalOp::And(*self)),
            TokenKind::Or | TokenKind::DoublePipe => Some(LogicalOp::Or(*self)),
            TokenKind::Xor => Some(LogicalOp::Xor(*self)),
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
pub enum TokenKind {
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

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TokenKind::Switch => "switch",
            TokenKind::Case => "case",
            TokenKind::Break => "break",
            TokenKind::Return => "return",
            TokenKind::Colon => ":",
            TokenKind::Dot => ".",
            TokenKind::Enum => "enum",
            TokenKind::LeftBrace => "{",
            TokenKind::RightBrace => "}",
            TokenKind::LeftParenthesis => "(",
            TokenKind::RightParenthesis => ")",
            TokenKind::LeftSquareBracket => "[",
            TokenKind::RightSquareBracket => "]",
            TokenKind::Default => "default",
            TokenKind::Comma => ",",
            TokenKind::Ampersand => "&",
            TokenKind::And => "and",
            TokenKind::DoubleAmpersand => "&&",
            TokenKind::Or => "or",
            TokenKind::DoublePipe => "||",
            TokenKind::Xor => "xor",
            TokenKind::Caret => "^",
            TokenKind::Equal => "=",
            TokenKind::DoubleEqual => "==",
            TokenKind::BangEqual => "!=",
            TokenKind::Function => "function",
            TokenKind::Constructor => "constructor",
            TokenKind::Exit => "exit",
            TokenKind::New => "new",
            TokenKind::Global => "global",
            TokenKind::Globalvar => "globalvar",
            TokenKind::SelfKeyword => "self",
            TokenKind::Mod => "mod",
            TokenKind::Percent => "%",
            TokenKind::PercentEqual => "%=",
            TokenKind::Div => "div",
            TokenKind::Slash => "/",
            TokenKind::Star => "*",
            TokenKind::Try => "try",
            TokenKind::Catch => "catch",
            TokenKind::With => "with",
            TokenKind::True => "true",
            TokenKind::False => "false",
            TokenKind::Plus => "+",
            TokenKind::Minus => "-",
            TokenKind::Bang => "!",
            TokenKind::Hook => "?",
            TokenKind::DoubleHook => "??",
            TokenKind::DoubleHookEquals => "??=",
            TokenKind::GreaterThan => ">",
            TokenKind::GreaterThanOrEqual => ">=",
            TokenKind::LessThan => "<",
            TokenKind::LessThanOrEqual => "<=",
            TokenKind::Pipe => "|",
            TokenKind::SemiColon => ";",
            TokenKind::Hash => "#",
            TokenKind::If => "if",
            TokenKind::Else => "else",
            TokenKind::While => "while",
            TokenKind::For => "for",
            TokenKind::Do => "do",
            TokenKind::Until => "until",
            TokenKind::Repeat => "repeat",
            TokenKind::Var => "var",
            TokenKind::PlusEqual => "+=",
            TokenKind::MinusEqual => "-=",
            TokenKind::StarEqual => "*=",
            TokenKind::SlashEqual => "/=",
            TokenKind::DoublePlus => "++",
            TokenKind::DoubleMinus => "--",
            TokenKind::DollarSign => "$",
            TokenKind::PipeEqual => "|=",
            TokenKind::AmpersandEqual => "&=",
            TokenKind::CaretEquals => "^=",
            TokenKind::Tilde => "~",
            TokenKind::BitShiftLeft => "<<",
            TokenKind::BitShiftRight => ">>",
            TokenKind::AtSign => "@",
            TokenKind::Continue => "continue",
            TokenKind::Static => "static",
            TokenKind::Then => "then",
            TokenKind::Finally => "finally",
            TokenKind::Undefined => "undefined",
            TokenKind::Noone => "noone",
            TokenKind::Not => "not",
            TokenKind::Other => "other",
            TokenKind::Delete => "delete",
            TokenKind::Begin => "begin",
            TokenKind::End => "end",
            TokenKind::Throw => "throw",
            TokenKind::ColonEqual => ":=",
            TokenKind::LessThanGreaterThan => "<>",
            TokenKind::Macro(name, config, body) => {
                return if let Some(config) = config {
                    f.pad(&format!("#macro {config}:{name} {body}"))
                } else {
                    f.pad(&format!("#macro {name} {body}"))
                };
            }
            TokenKind::Comment(body) => return f.pad(&format!("// {body}")),
            TokenKind::Identifier(iden) => iden,
            TokenKind::Real(r) => return f.pad(&r.to_string()),
            TokenKind::StringLiteral(s) => return f.pad(&format!("\"{s}\"")),
            TokenKind::Tag(label, param) => {
                if let Some(param) = param {
                    return f.pad(&format!("// #[{label}({param})]"));
                } else {
                    return f.pad(&format!("// #[{label}]"));
                }
            }
            TokenKind::Hex(hex) => hex,
            TokenKind::MiscConstant(con) => con,
            TokenKind::Invalid(_) => "INVALID_TOKEN",
            TokenKind::Eof => "",
        };
        f.pad(s)
    }
}
