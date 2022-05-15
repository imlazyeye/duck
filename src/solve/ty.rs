use super::*;
use hashbrown::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Ty {
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
    pub fn contains(&self, other: &Ty, subs: &Subs) -> bool {
        match self {
            ty if ty == other => true,
            Ty::Var(var) => subs.get(var).map_or(false, |v| v.contains(other, subs)),
            Ty::Array(inner) => inner.contains(other, subs),
            Ty::Func(func) => {
                func.parameters().iter().any(|v| v.contains(other, subs)) || func.return_type().contains(other, subs)
            }
            Ty::Adt(adt) => adt
                .fields
                .iter()
                .any(|(_, v)| v.value.ty().map_or(false, |v| v.contains(other, subs))),
            Ty::Option(inner) => inner.contains(other, subs),
            _ => false,
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

    pub fn sanatize(&mut self, adt_id: AdtId) {
        match self {
            Ty::Array(inner) => inner.sanatize(adt_id),
            Ty::Adt(adt) => {
                if adt.id == adt_id {
                    println!("sanitizing!");
                    *self = Ty::Identity;
                } else {
                    adt.fields.iter_mut().for_each(|(_, v)| {
                        if let Some(v) = v.value.ty_mut() {
                            v.sanatize(adt_id)
                        }
                    })
                }
            }
            Ty::Func(func) => {
                func.parameters_mut().iter_mut().for_each(|v| v.sanatize(adt_id));
                func.return_type_mut().sanatize(adt_id)
            }
            Ty::Option(inner) => inner.sanatize(adt_id),
            _ => {}
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
            let ty = ty.clone();
            match ty {
                Ty::Var(var) => {
                    if let Some(mapping) = map.get(&var) {
                        Ty::Var(*mapping)
                    } else {
                        let new = Var::Generated(rand::random());
                        map.insert(var, new);
                        Ty::Var(new)
                    }
                }
                Ty::Array(inner) => Ty::Array(Box::new(checkout_ty(&inner, map))),
                Ty::Adt(adt) => Ty::Adt(Adt {
                    id: AdtId::new(),
                    fields: adt
                        .fields
                        .iter()
                        .map(|(n, v)| {
                            (
                                n.clone(),
                                Field {
                                    value: v.value.ty().map_or(FieldValue::Uninitialized, |v| {
                                        FieldValue::Initialized(checkout_ty(v, map))
                                    }),
                                    constant: v.constant,
                                    resolved: v.resolved,
                                },
                            )
                        })
                        .collect(),

                    state: adt.state,
                    bounties: adt.bounties,
                }),
                Ty::Func(func) => match func {
                    Func::Def(Def {
                        binding,
                        parameters,
                        minimum_arguments,
                        return_type,
                    }) => Ty::Func(Func::Def(Def {
                        binding,
                        parameters: parameters.iter().map(|v| checkout_ty(v, map)).collect(),
                        minimum_arguments,
                        return_type: Box::new(checkout_ty(&return_type, map)),
                    })),
                    Func::Call(Call {
                        parameters,
                        return_type,
                    }) => Ty::Func(Func::Call(Call {
                        parameters: parameters.iter().map(|v| checkout_ty(v, map)).collect(),
                        return_type: Box::new(checkout_ty(&return_type, map)),
                    })),
                },
                Ty::Option(inner) => Ty::Option(Box::new(checkout_ty(&inner, map))),
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
pub struct Binding {
    pub local_var: Var,
    pub identity_var: Var,
}
