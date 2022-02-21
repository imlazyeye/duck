use super::statement::StatementBox;
use crate::{Position, Span};

#[derive(Debug, PartialEq)]
pub enum Expression {
    FunctionDeclaration(
        Option<String>,
        Vec<Parameter>,
        Option<Constructor>,
        StatementBox,
        bool,
    ),
    Logical(ExpressionBox, LogicalOperator, ExpressionBox),
    Equality(ExpressionBox, EqualityOperator, ExpressionBox),
    Evaluation(ExpressionBox, EvaluationOperator, ExpressionBox),
    NullCoalecence(ExpressionBox, ExpressionBox),
    Ternary(ExpressionBox, ExpressionBox, ExpressionBox),
    Assignment(ExpressionBox, AssignmentOperator, ExpressionBox),
    Unary(UnaryOperator, ExpressionBox),
    Postfix(ExpressionBox, PostfixOperator),
    Access(ExpressionBox, AccessScope),
    Call(ExpressionBox, Vec<ExpressionBox>, bool),
    Grouping(ExpressionBox),
    Literal(Literal),
    Identifier(String),
}
impl Expression {
    pub fn into_box(self, span: Span) -> ExpressionBox {
        ExpressionBox(Box::new(self), span)
    }
    pub fn lazy_box(self) -> ExpressionBox {
        ExpressionBox(Box::new(self), Span::default())
    }
}

#[derive(Debug, PartialEq)]
pub struct ExpressionBox(pub Box<Expression>, pub Span);
impl ExpressionBox {
    pub fn expression(&self) -> &Expression {
        self.0.as_ref()
    }
    pub fn span(&self) -> Span {
        self.1
    }
}

#[derive(Debug, PartialEq)]
pub enum EvaluationOperator {
    Plus,
    Minus,
    Slash,
    Star,
    Div,
    Modulo,
    And,
    Or,
    Xor,
    BitShiftLeft,
    BitShiftRight,
}

#[derive(Debug, PartialEq)]
pub enum EqualityOperator {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

#[derive(Debug, PartialEq)]
pub enum LogicalOperator {
    And,
    Or,
    Xor,
}

#[derive(Debug, PartialEq)]
#[allow(clippy::enum_variant_names)]
pub enum AssignmentOperator {
    Equal,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    XorEqual,
    OrEqual,
    AndEqual,
    NullCoalecenceEqual,
    ModEqual,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Increment,
    Decrement,
    Not,
    Negative,
    BitwiseNot,
}

#[derive(Debug, PartialEq)]
pub enum PostfixOperator {
    Increment,
    Decrement,
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    True,
    False,
    String(String),
    Real(f64),
    Hex(String),
    Array(Vec<ExpressionBox>),
    Struct(Vec<(String, ExpressionBox)>),
}

#[derive(Debug, PartialEq)]
pub enum AccessScope {
    Global,
    Current,
    Dot(ExpressionBox),
    Array(ExpressionBox, Option<ExpressionBox>, bool),
    Map(ExpressionBox),
    Grid(ExpressionBox, ExpressionBox),
    List(ExpressionBox),
    Struct(ExpressionBox),
}

#[derive(Debug, PartialEq)]
pub struct Constructor(pub Option<ExpressionBox>);

#[derive(Debug, PartialEq)]
pub struct Parameter(pub String, pub Option<ExpressionBox>);
