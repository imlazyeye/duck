use hashbrown::HashMap;

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
    Adt(Adt),
    Func(Func),
    Option(Box<Ty>),
}

impl Ty {
    pub const UNINITIALIZED: Ty = Ty::Uninitialized;
    pub const ANY: Ty = Ty::Any;
    pub const IDENTITY: Ty = Ty::Identity;
    pub const UNDEFINED: Ty = Ty::Undefined;
    pub const NOONE: Ty = Ty::Noone;
    pub const BOOL: Ty = Ty::Bool;
    pub const REAL: Ty = Ty::Real;
    pub const STR: Ty = Ty::Str;

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

    pub fn adt(&self) -> &Adt {
        match self {
            Ty::Adt(adt) => adt,
            _ => panic!("Failed to find an adt on {}", Printer::ty(self)),
        }
    }

    pub fn adt_mut(&mut self) -> &mut Adt {
        match self {
            Ty::Adt(adt) => adt,
            _ => panic!("Failed to find an adt on {}", Printer::ty(self)),
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
    pub fn checkout(&self) -> Def {
        fn checkout_ty(ty: &Ty, map: &mut HashMap<Var, Var>) -> Ty {
            match ty {
                Ty::Var(var) => {
                    if let Some(mapping) = map.get(var) {
                        Ty::Var(*mapping)
                    } else {
                        let new = Var::Generated(rand::random());
                        map.insert(*var, new);
                        Ty::Var(new)
                    }
                }
                Ty::Array(inner) => Ty::Array(Box::new(checkout_ty(inner, map))),
                Ty::Adt(adt) => Ty::Adt(Adt {
                    fields: adt
                        .fields
                        .iter()
                        .map(|(n, v)| {
                            (
                                n.clone(),
                                Field {
                                    ty: checkout_ty(&v.ty, map),
                                    constant: v.constant,
                                    resolved: v.resolved,
                                },
                            )
                        })
                        .collect(),

                    state: adt.state,
                    bounties: adt.bounties.clone(),
                }),
                Ty::Func(func) => match func {
                    Func::Def(Def {
                        binding,
                        parameters,
                        minimum_arguments,
                        return_type,
                    }) => Ty::Func(Func::Def(Def {
                        binding: binding.clone(),
                        parameters: parameters.iter().map(|v| checkout_ty(v, map)).collect(),
                        minimum_arguments: *minimum_arguments,
                        return_type: Box::new(checkout_ty(return_type, map)),
                    })),
                    Func::Call(Call {
                        parameters,
                        return_type,
                    }) => Ty::Func(Func::Call(Call {
                        parameters: parameters.iter().map(|v| checkout_ty(v, map)).collect(),
                        return_type: Box::new(checkout_ty(return_type, map)),
                    })),
                },
                Ty::Option(inner) => Ty::Option(Box::new(checkout_ty(inner, map))),
                _ => ty.clone(),
            }
        }

        let mut remap = HashMap::new();
        Def {
            binding: self.binding.clone(),
            parameters: self.parameters.iter().map(|v| checkout_ty(v, &mut remap)).collect(),
            minimum_arguments: self.minimum_arguments,
            return_type: Box::new(checkout_ty(&self.return_type, &mut remap)),
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
