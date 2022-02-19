use super::expression::{
    AssignmentOperator, EvaluationOperator, Literal, PostfixOperator, UnaryOperator,
};

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
    PlusEquals,
    MinusEquals,
    StarEquals,
    SlashEquals,
    DoublePlus,
    DoubleMinus,
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

    pub fn as_evaluation_operator(&self) -> Option<EvaluationOperator> {
        match self {
            Token::Plus => Some(EvaluationOperator::Plus),
            Token::Minus => Some(EvaluationOperator::Minus),
            Token::Slash => Some(EvaluationOperator::Slash),
            Token::Star => Some(EvaluationOperator::Star),
            Token::Div => Some(EvaluationOperator::Div),
            Token::Equals | Token::DoubleEquals => Some(EvaluationOperator::Equals),
            Token::ModKeyword | Token::ModSymbol => Some(EvaluationOperator::Mod),
            Token::GreaterThan => Some(EvaluationOperator::GreaterThan),
            Token::GreaterThanOrEqual => Some(EvaluationOperator::GreaterThanOrEqual),
            Token::LessThan => Some(EvaluationOperator::LessThan),
            Token::LessThanOrEqual => Some(EvaluationOperator::LessThanOrEqual),
            Token::AndKeyword | Token::AndSymbol => Some(EvaluationOperator::And),
            Token::OrKeyword | Token::OrSymbol => Some(EvaluationOperator::Or),
            _ => None,
        }
    }

    pub fn as_assignment_operator(&self) -> Option<AssignmentOperator> {
        match self {
            Token::Equals => Some(AssignmentOperator::Equals),
            Token::PlusEquals => Some(AssignmentOperator::PlusEquals),
            Token::MinusEquals => Some(AssignmentOperator::MinusEquals),
            Token::StarEquals => Some(AssignmentOperator::StarEquals),
            Token::SlashEquals => Some(AssignmentOperator::SlashEquals),
            _ => None,
        }
    }

    pub fn as_unary_operator(&self) -> Option<UnaryOperator> {
        match self {
            Token::Bang => Some(UnaryOperator::Not),
            Token::Minus => Some(UnaryOperator::Negative),
            _ => None,
        }
    }

    pub fn as_postfix_operator(&self) -> Option<PostfixOperator> {
        match self {
            Token::DoublePlus => Some(PostfixOperator::Increment),
            Token::DoubleMinus => Some(PostfixOperator::Decrement),
            _ => None,
        }
    }

    pub fn as_identifier(&self) -> Option<&str> {
        match self {
            Token::Identifier(lexeme) => Some(lexeme),
            _ => None,
        }
    }
}
