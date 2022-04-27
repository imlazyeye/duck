use crate::parse::Identifier;

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Ty {
    Uninitialized,
    Any,
    Identity,
    Undefined,
    Noone,
    Bool,
    Real,
    Str,
    Var(Var),
    Array(Box<Ty>),
    Adt(AdtId),
    Func(Func),
}

impl Ty {
    pub fn contains(&self, other: &Ty) -> bool {
        match self {
            Ty::Array(inner) => inner.contains(other),
            Ty::Func(func) => func.parameters().iter().any(|v| v.contains(other)) || func.return_type().contains(other),
            _ => self == other,
        }
    }

    pub fn replace(&mut self, search: &Ty, replace: Ty) {
        match self {
            Ty::Array(inner) => {
                if inner.as_ref() == search {
                    *inner.as_mut() = replace
                }
            }
            Ty::Func(func) => {
                func.parameters_mut().iter_mut().for_each(|v| {
                    if v == search {
                        *v = replace.clone()
                    }
                });
                if func.return_type() == search {
                    *func.return_type_mut() = replace
                }
            }
            _ => {
                if self == search {
                    *self = replace;
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Func {
    Def(Def),
    Call(Call),
}
impl Func {
    pub fn parameters(&self) -> &[Ty] {
        match self {
            Func::Def(inner) => &inner.parameters,
            Func::Call(inner) => &inner.parameters,
        }
    }
    pub fn parameters_mut(&mut self) -> &mut [Ty] {
        match self {
            Func::Def(inner) => &mut inner.parameters,
            Func::Call(inner) => &mut inner.parameters,
        }
    }
    pub fn return_type(&self) -> &Ty {
        match self {
            Func::Def(inner) => &inner.return_type,
            Func::Call(inner) => &inner.return_type,
        }
    }
    pub fn return_type_mut(&mut self) -> &mut Ty {
        match self {
            Func::Def(inner) => &mut inner.return_type,
            Func::Call(inner) => &mut inner.return_type,
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct Def {
    pub binding: Option<Binding>,
    pub parameters: Vec<Ty>,
    pub minimum_arguments: usize,
    pub return_type: Box<Ty>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Call {
    pub parameters: Vec<Ty>,
    pub return_type: Box<Ty>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Binding {
    Method {
        local_scope: AdtId,
        self_scope: AdtId,
    },
    Constructor {
        local_scope: AdtId,
        self_scope: AdtId,
        inheritance: Option<Identifier>,
    },
}
impl Binding {
    pub fn self_scope(&self) -> AdtId {
        match self {
            Binding::Method { self_scope, .. } | Binding::Constructor { self_scope, .. } => *self_scope,
        }
    }
}
