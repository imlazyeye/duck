use std::ops::{Deref, DerefMut};

use super::statement::StatementBox;

#[derive(Debug, PartialEq)]
pub enum Expression {
    FunctionDeclaration(Function),
    Logical(ExpressionBox, LogicalOperator, ExpressionBox),
    Equality(ExpressionBox, EqualityOperator, ExpressionBox),
    Evaluation(ExpressionBox, EvaluationOperator, ExpressionBox),
    Ternary(ExpressionBox, ExpressionBox, ExpressionBox),
    Assignment(ExpressionBox, AssignmentOperator, ExpressionBox),
    Unary(UnaryOperator, ExpressionBox),
    Postfix(ExpressionBox, PostfixOperator),
    ArrayLiteral(Vec<ExpressionBox>),
    StructLiteral(Vec<(String, ExpressionBox)>),
    DSAccess(ExpressionBox, DSAccess),
    DotAccess(AccessScope, ExpressionBox),
    Call(ExpressionBox, Vec<ExpressionBox>, bool),
    Grouping(ExpressionBox),
    Literal(Literal),
    Identifier(String),
}

#[derive(Debug, PartialEq)]
pub struct ExpressionBox(pub Box<Expression>);
impl From<Expression> for ExpressionBox {
    fn from(exp: Expression) -> Self {
        Self(Box::new(exp))
    }
}
impl Deref for ExpressionBox {
    type Target = Expression;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for ExpressionBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Not,
    Negative,
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
}

#[derive(Debug, PartialEq)]
pub enum DSAccess {
    Array(ExpressionBox),
    Map(ExpressionBox),
    Grid(ExpressionBox, ExpressionBox),
    List(ExpressionBox),
    Struct(ExpressionBox),
}

#[derive(Debug, PartialEq)]
pub enum Function {
    Anonymous(Vec<Parameter>, Option<Constructor>, StatementBox),
    Named(String, Vec<Parameter>, Option<Constructor>, StatementBox),
}

#[derive(Debug, PartialEq)]
pub struct Constructor(pub Option<ExpressionBox>);

#[derive(Debug, PartialEq)]
pub struct Parameter(pub String, pub Option<ExpressionBox>);

#[derive(Debug, PartialEq)]
pub enum AccessScope {
    Global,
    /// This is Self. I can't use Self.
    Current,
    Other(ExpressionBox),
}
