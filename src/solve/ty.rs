use hashbrown::HashMap;

use crate::{parse::Identifier, var};

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
    Adt(Adt),
    Func(Func),
    Option(Box<Ty>),
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

impl Def {
    pub fn checkout(&self, solver: &mut Solver) -> Def {
        fn checkout_ty(ty: &Ty, solver: &mut Solver, map: &mut HashMap<Var, Var>) -> Ty {
            match ty {
                Ty::Var(var) => {
                    if let Some(ty) = solver.subs.get(&var).cloned() {
                        checkout_ty(&ty, solver, map)
                    } else if let Some(mapping) = map.get(&var) {
                        Ty::Var(*mapping)
                    } else {
                        let new = Var::Generated(rand::random());
                        map.insert(*var, new);
                        Ty::Var(new)
                    }
                }
                Ty::Array(inner) => Ty::Array(Box::new(checkout_ty(inner, solver, map))),
                Ty::Adt(_) => ty.clone(), // todo: this will cause a bug and ill get so frusterated
                Ty::Func(func) => match func {
                    Func::Def(Def {
                        binding,
                        parameters,
                        minimum_arguments,
                        return_type,
                    }) => Ty::Func(Func::Def(Def {
                        binding: binding.clone(),
                        parameters: parameters.iter().map(|v| checkout_ty(v, solver, map)).collect(),
                        minimum_arguments: *minimum_arguments,
                        return_type: Box::new(checkout_ty(return_type, solver, map)),
                    })),
                    Func::Call(Call {
                        parameters,
                        return_type,
                    }) => Ty::Func(Func::Call(Call {
                        parameters: parameters.iter().map(|v| checkout_ty(v, solver, map)).collect(),
                        return_type: Box::new(checkout_ty(return_type, solver, map)),
                    })),
                },
                Ty::Option(inner) => Ty::Option(Box::new(checkout_ty(inner, solver, map))),
                _ => ty.clone(),
            }
        }

        let mut remap = HashMap::new();
        Def {
            binding: self.binding.clone(),
            parameters: self
                .parameters
                .iter()
                .map(|v| checkout_ty(v, solver, &mut remap))
                .collect(),
            minimum_arguments: self.minimum_arguments,
            return_type: Box::new(checkout_ty(&self.return_type, solver, &mut remap)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Call {
    pub parameters: Vec<Ty>,
    pub return_type: Box<Ty>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Binding {
    Method {
        local_scope: Var,
        self_scope: Var,
    },
    Constructor {
        local_scope: Var,
        self_scope: Var,
        inheritance: Option<Identifier>,
    },
}
impl Binding {
    pub fn self_scope(&self) -> &Var {
        match self {
            Binding::Method { self_scope, .. } | Binding::Constructor { self_scope, .. } => self_scope,
        }
    }
}
