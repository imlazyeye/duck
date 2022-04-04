use super::*;
use crate::parse::Identifier;

#[derive(Debug, PartialEq, Clone)]
pub enum App {
    Array(Box<Term>),
    Record(Record),
    Function(Function),
    Call(Call),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub binding: Option<Marker>,
    pub inheritance: Option<Identifier>,
    pub parameters: Vec<Term>,
    pub return_type: Box<Term>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Call {
    pub parameters: Vec<Term>,
    pub return_type: Box<Term>,
}
