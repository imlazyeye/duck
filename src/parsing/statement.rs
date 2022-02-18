use std::ops::{Deref, DerefMut};

use super::{expression::ExpressionBox, Token};

#[derive(Debug, PartialEq)]
pub enum Statement {
    Expression(ExpressionBox),
    Block(Vec<StatementBox>),
    If(ExpressionBox, StatementBox, Option<ExpressionBox>),
    While(ExpressionBox, StatementBox),
    FunctionDeclaration(String, Vec<FunctionParameter>, bool, StatementBox),
    LocalVariableDeclaration(String, Option<ExpressionBox>),
    Return(Option<ExpressionBox>),
}

#[derive(Debug, PartialEq)]
pub struct StatementBox(Box<Statement>);
impl Deref for StatementBox {
    type Target = Statement;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for StatementBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, PartialEq)]
pub struct FunctionParameter {
    pub name: Token,
    pub default_value: Option<ExpressionBox>,
}
