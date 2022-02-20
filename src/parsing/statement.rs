use std::ops::{Deref, DerefMut};

use super::expression::ExpressionBox;

#[derive(Debug, PartialEq)]
pub enum Statement {
    MacroDeclaration(String, Option<String>, String),
    EnumDeclaration(String, Vec<ExpressionBox>),
    GlobalvarDeclaration(String),
    LocalVariableSeries(Vec<ExpressionBox>),
    TryCatch(StatementBox, ExpressionBox, StatementBox),
    For(StatementBox, ExpressionBox, StatementBox, StatementBox),
    With(ExpressionBox, StatementBox),
    Repeat(ExpressionBox, StatementBox),
    DoUntil(StatementBox, ExpressionBox),
    While(ExpressionBox, StatementBox),
    If(ExpressionBox, StatementBox, Option<StatementBox>),
    Switch(ExpressionBox, Vec<Case>, Option<Vec<StatementBox>>),
    Block(Vec<StatementBox>),
    Return(Option<ExpressionBox>),
    Break,
    Continue,
    Exit,
    Expression(ExpressionBox),
}
impl Statement {
    pub fn expressions(&self) -> &[ExpressionBox] {
        match self {
            Statement::MacroDeclaration(_, _, _) => &[],
            Statement::EnumDeclaration(_, members) => members.as_slice(),
            Statement::GlobalvarDeclaration(_) => &[],
            Statement::LocalVariableSeries(_) => todo!(),
            Statement::TryCatch(_, _, _) => todo!(),
            Statement::For(_, _, _, _) => todo!(),
            Statement::With(_, _) => todo!(),
            Statement::Repeat(_, _) => todo!(),
            Statement::DoUntil(_, _) => todo!(),
            Statement::While(_, _) => todo!(),
            Statement::If(_, _, _) => todo!(),
            Statement::Switch(_, _, _) => todo!(),
            Statement::Block(_) => todo!(),
            Statement::Return(_) => todo!(),
            Statement::Break => todo!(),
            Statement::Continue => todo!(),
            Statement::Exit => todo!(),
            Statement::Expression(_) => todo!(),
        }
    }
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