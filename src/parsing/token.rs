use super::expression::{
    AssignmentOperator, EqualityOperator, EvaluationOperator, Literal, LogicalOperator,
    PostfixOperator, UnaryOperator,
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
    ModKeyword,
    ModSymbol,
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
    BitShiftLeft,
    BitShiftRight,
    Macro(String, Option<String>, String),
    Comment(String),
    Identifier(String),
    Real(f64),
    StringLiteral(String),
    LintTag(String),
    Hex(String),
    Eof,
}
impl Token {
    pub fn to_literal(&self) -> Option<Literal> {
        match self {
            Token::True => Some(Literal::True),
            Token::False => Some(Literal::False),
            Token::StringLiteral(lexeme) => Some(Literal::String(lexeme.clone())),
            Token::Real(value) => Some(Literal::Real(*value)),
            Token::Hex(lexeme) => Some(Literal::Hex(lexeme.clone())),
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
            Token::ModKeyword | Token::ModSymbol => Some(EvaluationOperator::Modulo),
            Token::Ampersand => Some(EvaluationOperator::And),
            Token::Pipe => Some(EvaluationOperator::Or),
            Token::Circumflex => Some(EvaluationOperator::Xor),
            Token::BitShiftLeft => Some(EvaluationOperator::BitShiftLeft),
            Token::BitShiftRight => Some(EvaluationOperator::BitShiftRight),
            _ => None,
        }
    }

    pub fn as_equality_operator(&self) -> Option<EqualityOperator> {
        match self {
            Token::Equal | Token::DoubleEqual => Some(EqualityOperator::Equal),
            Token::BangEqual => Some(EqualityOperator::NotEqual),
            Token::GreaterThan => Some(EqualityOperator::GreaterThan),
            Token::GreaterThanOrEqual => Some(EqualityOperator::GreaterThanOrEqual),
            Token::LessThan => Some(EqualityOperator::LessThan),
            Token::LessThanOrEqual => Some(EqualityOperator::LessThanOrEqual),
            _ => None,
        }
    }

    pub fn as_assignment_operator(&self) -> Option<AssignmentOperator> {
        match self {
            Token::Equal => Some(AssignmentOperator::Equal),
            Token::PlusEqual => Some(AssignmentOperator::PlusEqual),
            Token::MinusEqual => Some(AssignmentOperator::MinusEqual),
            Token::StarEqual => Some(AssignmentOperator::StarEqual),
            Token::SlashEqual => Some(AssignmentOperator::SlashEqual),
            Token::PipeEqual => Some(AssignmentOperator::OrEqual),
            Token::AmpersandEqual => Some(AssignmentOperator::AndEqual),
            Token::CirumflexEqual => Some(AssignmentOperator::XorEqual),
            Token::DoubleInterrobangEquals => Some(AssignmentOperator::NullCoalecenceEqual),
            _ => None,
        }
    }

    pub fn as_unary_operator(&self) -> Option<UnaryOperator> {
        match self {
            Token::DoublePlus => Some(UnaryOperator::Increment),
            Token::DoubleMinus => Some(UnaryOperator::Decrement),
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

    pub fn as_logical_operator(&self) -> Option<LogicalOperator> {
        match self {
            Token::And | Token::DoubleAmpersand => Some(LogicalOperator::And),
            Token::Or | Token::DoublePipe => Some(LogicalOperator::Or),
            Token::Xor => Some(LogicalOperator::Xor),
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
