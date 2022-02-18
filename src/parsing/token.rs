use super::expression::{Literal, Operator};

#[derive(Debug, PartialEq, Clone)]
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
    AndKeyword,
    AndSymbol,
    OrKeyword,
    OrSymbol,
    Equals,
    DoubleEquals,
    Macro,
    Function,
    Constructor,
    Exit,
    New,
    Global,
    Globalvar,
    ModKeyword,
    ModSymbol,
    Div,
    Slash,
    Star,
    Try,
    With,
    True,
    False,
    Plus,
    Minus,
    Bang,
    Interrobang,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    BitwiseAnd,
    BitwiseOr,
    Comment(String),
    Identifier(String),
    Real(f64),
    StringLiteral(String),
    LintTag(String),
    Eof,
}
impl Token {
    pub fn to_literal(&self) -> Option<Literal> {
        match self {
            Token::True => Some(Literal::True),
            Token::False => Some(Literal::False),
            Token::StringLiteral(lexeme) => Some(Literal::String(lexeme.clone())),
            Token::Real(value) => Some(Literal::Real(*value)),
            _ => None,
        }
    }

    pub fn to_operator(&self) -> Option<Operator> {
        match self {
            Token::Plus => Some(Operator::Plus),
            Token::Minus => Some(Operator::Minus),
            Token::Slash => Some(Operator::Slash),
            Token::Star => Some(Operator::Star),
            Token::Bang => Some(Operator::Bang),
            Token::Interrobang => Some(Operator::Interrobang),
            Token::Div => Some(Operator::Div),
            Token::ModKeyword | Token::ModSymbol => Some(Operator::Mod),
            Token::GreaterThan => Some(Operator::GreaterThan),
            Token::GreaterThanOrEqual => Some(Operator::GreaterThanOrEqual),
            Token::LessThan => Some(Operator::LessThan),
            Token::LessThanOrEqual => Some(Operator::LessThanOrEqual),
            //Token::Equals => Some(Operator::Equals),
            Token::DoubleEquals => Some(Operator::Equals),
            Token::AndKeyword | Token::AndSymbol => Some(Operator::And),
            Token::OrKeyword | Token::OrSymbol => Some(Operator::Or),
            _ => None,
        }
    }

    pub fn as_identifier(self) -> Option<String> {
        match self {
            Token::Identifier(lexeme) => Some(lexeme),
            _ => None,
        }
    }
}
