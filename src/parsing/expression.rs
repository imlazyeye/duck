use super::statement::StatementBox;
use crate::Span;

#[derive(Debug, PartialEq, Clone)]
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
    Access(Scope, ExpressionBox),
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

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionBox(pub Box<Expression>, pub Span);
impl ExpressionBox {
    pub fn expression(&self) -> &Expression {
        self.0.as_ref()
    }
    pub fn span(&self) -> Span {
        self.1
    }
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub enum EqualityOperator {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LogicalOperator {
    And,
    Or,
    Xor,
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    Increment,
    Decrement,
    Not,
    Negative,
    BitwiseNot,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PostfixOperator {
    Increment,
    Decrement,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    True,
    False,
    String(String),
    Real(f64),
    Hex(String),
    Array(Vec<ExpressionBox>),
    Struct(Vec<(String, ExpressionBox)>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Scope {
    Global,
    Current,
    Dot(ExpressionBox),
    Array(ExpressionBox, Option<ExpressionBox>, bool),
    Map(ExpressionBox),
    Grid(ExpressionBox, ExpressionBox),
    List(ExpressionBox),
    Struct(ExpressionBox),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Constructor(pub Option<ExpressionBox>);

#[derive(Debug, PartialEq, Clone)]
pub struct Parameter(pub String, pub Option<ExpressionBox>);
