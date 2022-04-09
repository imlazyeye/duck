use crate::parse::Identifier;

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Ty {
    Null,
    Any,
    Undefined,
    Noone,
    Bool,
    Real,
    Str,
    Var(Var),
    Array(Box<Ty>),
    Record(Record),
    Function(Function),
    Call(Call),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub binding: Option<Binding>,
    pub local_var: Var,
    pub parameters: Vec<Ty>,
    pub return_type: Box<Ty>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Binding {
    Method(Var),
    Constructor(Var, Option<Identifier>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Call {
    pub parameters: Vec<Ty>,
    pub target: Box<Ty>,
}
