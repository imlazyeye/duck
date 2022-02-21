use super::expression::ExpressionBox;
use crate::Position;

#[derive(Debug, PartialEq)]
pub enum Statement {
    MacroDeclaration(String, Option<String>, String),
    EnumDeclaration(String, Vec<(String, Option<ExpressionBox>)>),
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
    pub fn into_box(self, position: Position) -> StatementBox {
        StatementBox(Box::new(self), position)
    }
    pub fn lazy_box(self) -> StatementBox {
        StatementBox(Box::new(self), Position::default())
    }
}

#[derive(Debug, PartialEq)]
pub struct StatementBox(pub Box<Statement>, pub Position);
impl StatementBox {
    pub fn statement(&self) -> &Statement {
        self.0.as_ref()
    }
}

#[derive(Debug, PartialEq)]
pub struct Case(pub ExpressionBox, pub Vec<StatementBox>); // kinda a block?
