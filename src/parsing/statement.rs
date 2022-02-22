use super::expression::ExpressionBox;
use crate::Span;

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    MacroDeclaration(String, Option<String>, String),
    EnumDeclaration(String, Vec<(String, Option<ExpressionBox>)>),
    GlobalvarDeclaration(String),
    LocalVariableSeries(Vec<(String, Option<ExpressionBox>)>),
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
    pub fn into_box(self, span: Span) -> StatementBox {
        StatementBox(Box::new(self), span)
    }
    pub fn lazy_box(self) -> StatementBox {
        StatementBox(Box::new(self), Span::default())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct StatementBox(pub Box<Statement>, pub Span);
impl StatementBox {
    pub fn statement(&self) -> &Statement {
        self.0.as_ref()
    }
    pub fn span(&self) -> Span {
        self.1
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Case(pub ExpressionBox, pub Vec<StatementBox>); // kinda a block?
