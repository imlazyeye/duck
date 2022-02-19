use std::ops::{Deref, DerefMut};

#[derive(Debug, PartialEq)]
pub enum Expression {
    Evaluation(ExpressionBox, EvaluationOperator, ExpressionBox),
    Ternary(ExpressionBox, ExpressionBox, ExpressionBox),
    Assignment(ExpressionBox, AssignmentOperator, ExpressionBox),
    Unary(UnaryOperator, ExpressionBox),
    Postfix(ExpressionBox, PostfixOperator),
    Call(ExpressionBox, Vec<ExpressionBox>),
    ArrayLiteral(Vec<ExpressionBox>),
    StructLiteral(Vec<(String, ExpressionBox)>),
    DSAccess(ExpressionBox, DsAccess),
    DotAccess(ExpressionBox, ExpressionBox),
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
    Mod,
    Equals,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    And,
    Or,
}

#[derive(Debug, PartialEq)]
#[allow(clippy::enum_variant_names)]
pub enum AssignmentOperator {
    Equals,
    PlusEquals,
    MinusEquals,
    StarEquals,
    SlashEquals,
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
pub enum DsAccess {
    Array(ExpressionBox),
    Map(ExpressionBox),
    Grid(ExpressionBox, ExpressionBox),
    List(ExpressionBox),
}
