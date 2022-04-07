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
    pub binding: Option<Binding>,
    pub local_marker: Marker,
    pub parameters: Vec<Term>,
    pub return_type: Box<Term>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Binding {
    Method(Marker),
    Constructor(Marker, Option<Identifier>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Call {
    pub parameters: Vec<Term>,
    pub target: Box<Term>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum RecordOp {
    Read,
    Write,
}
