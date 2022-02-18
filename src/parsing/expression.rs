use std::ops::{Deref, DerefMut};

use super::Token;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Binary(ExpressionBox, Operator, ExpressionBox),
    Unary(Operator, ExpressionBox),
    Literal(Literal),
    Grouping(ExpressionBox),
    Identifier(String),
    Assign(ExpressionBox, ExpressionBox),
    Call(ExpressionBox, Vec<ExpressionBox>),
    Access(ExpressionBox, String),
}
impl Expression {
    pub fn to_box(self) -> ExpressionBox {
        ExpressionBox::new(self)
    }
}

#[derive(Debug, PartialEq)]
pub struct ExpressionBox(Box<Expression>);
impl ExpressionBox {
    pub fn new(expression: Expression) -> Self {
        Self(Box::new(expression))
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
pub enum Operator {
    Plus,
    Minus,
    Slash,
    Star,
    Bang,
    Interrobang,
    Div,
    Mod,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Equals,
    And,
    Or,
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    True,
    False,
    String(String),
    Real(f64),
}
