use std::ops::{Deref, DerefMut};

use super::expression::{Expression, ExpressionBox};

#[derive(Debug, PartialEq)]
pub enum Statement {
    MacroDeclaration(String, Option<String>, String),
    EnumDeclaration(String, Vec<ExpressionBox>),
    GlobalvarDeclaration(String),
    LocalVariableSeries(Vec<(String, Option<ExpressionBox>)>),
    TryCatch(StatementBox, ExpressionBox, StatementBox),
    For(StatementBox, StatementBox, StatementBox, StatementBox),
    With(ExpressionBox, StatementBox),
    Repeat(ExpressionBox, StatementBox),
    DoUntil(StatementBox, ExpressionBox),
    While(ExpressionBox, StatementBox),
    If(ExpressionBox, StatementBox, Option<StatementBox>),
    Switch(ExpressionBox, Vec<Case>, Option<Vec<StatementBox>>),
    Block(Vec<StatementBox>),
    Return(Option<ExpressionBox>),
    Break,
    Exit,
    Expression(ExpressionBox),
}

#[derive(Debug, PartialEq)]
pub struct StatementBox(Box<Statement>);
impl From<Statement> for StatementBox {
    fn from(stmt: Statement) -> Self {
        Self(Box::new(stmt))
    }
}
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
pub struct Case(pub ExpressionBox, pub Vec<StatementBox>); // kinda a block?
