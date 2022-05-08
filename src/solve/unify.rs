use super::*;
use crate::duck_error;

pub struct Unification;
impl Unification {
    pub fn var_var(lhs: Var, rhs: Var, session: &mut Session) -> Result<(), TypeError> {
        session
            .checkout(&rhs, |rhs_ty| Unification::unify(&mut Ty::Var(lhs), rhs_ty))?
            .commit(session.subs)
    }

    pub fn var_ty(var: Var, ty: &mut Ty, session: &mut Session) -> Result<(), TypeError> {
        ty.normalize(session);
        session
            .checkout(&var, |var_ty| Unification::unify(ty, var_ty))?
            .commit(session.subs)
    }

    pub fn unify(lhs: &mut Ty, rhs: &mut Ty) -> Result<Substitution, TypeError> {
        println!("{}", Printer::ty_unification(lhs, rhs));
        match (lhs, rhs) {
            (lhs, rhs) if lhs == rhs => Ok(Substitution::None),
            (Ty::Var(var), other) | (other, Ty::Var(var)) => Ok(Substitution::Single(*var, other.clone())),
            (Ty::Any, _) | (_, Ty::Any) => Ok(Substitution::None),
            (und @ Ty::Undefined, other) | (other, und @ Ty::Undefined) => {
                *other = Ty::Option(Box::new(other.clone()));
                *und = other.clone();
                Ok(Substitution::None)
            }
            (Ty::Array(lhs_member), Ty::Array(rhs_member)) => Self::unify(lhs_member, rhs_member),
            (Ty::Adt(lhs_adt), Ty::Adt(rhs_adt)) => {
                let mut sub = Substitution::None;
                for (name, rhs_field) in rhs_adt.fields.iter_mut() {
                    if let FieldValue::Initialized(rhs_ty) = &mut rhs_field.value {
                        sub = sub.combo(lhs_adt.read(name, rhs_ty.clone())?);
                    }
                }
                Ok(sub)
            }
            (Ty::Func(lhs_func), Ty::Func(rhs_func)) => match (lhs_func, rhs_func) {
                (Func::Def(def), call @ Func::Call(_)) | (call @ Func::Call(_), Func::Def(def)) => {
                    let mut sub = Substitution::None;
                    let mut def = def.checkout();
                    println!(
                        "\n--- Evaluating call for checkout: {}... ---\n",
                        Printer::ty(&Ty::Func(Func::Def(def.clone())))
                    );
                    if call.parameters().len() > def.parameters.len() {
                        return duck_error!("extra arguments provided to call");
                    }
                    for (i, param) in def.parameters.iter_mut().enumerate() {
                        if let Some(arg) = call.parameters_mut().get_mut(i) {
                            sub = sub.combo(Self::unify(arg, param)?);
                        } else if i < def.minimum_arguments {
                            return duck_error!("missing argument {i} in call");
                        };
                    }
                    sub = sub.combo(Self::unify(call.return_type_mut(), &mut def.return_type)?);
                    println!("\n--- Ending call... ---\n");
                    Ok(sub)
                }
                _ => unreachable!(),
            },
            (lhs, rhs) => {
                duck_error!(
                    "Attempted to equate two incompatible types: {} and {}",
                    Printer::ty(lhs),
                    Printer::ty(rhs)
                )
            }
        }
    }
}

impl Ty {
    pub fn normalize(&mut self, sess: &Session) -> &mut Self {
        if let Some(ty) = self.as_deep_normalized(sess) {
            *self = ty
        }
        self
    }

    pub fn as_shallow_normalized(&self, sess: &Session) -> Option<Ty> {
        match self {
            Ty::Var(var) => sess.subs.get(var).filter(|v| v != &&Ty::Var(*var)).cloned(),
            _ => None,
        }
    }

    pub fn as_deep_normalized(&self, sess: &Session) -> Option<Ty> {
        match self {
            Ty::Var(_) => self.as_shallow_normalized(sess).map(|ty| {
                if let Some(dty) = ty.as_deep_normalized(sess) {
                    dty
                } else {
                    ty
                }
            }),
            Ty::Array(inner) => inner.as_deep_normalized(sess).map(|v| Ty::Array(Box::new(v))),
            Ty::Adt(adt) => {
                let mut adt = adt.clone();
                let mut any = false;
                adt.fields.iter_mut().for_each(|(_, field)| {
                    if let Some(ty) = field.value.ty().and_then(|v| v.as_deep_normalized(sess)) {
                        field.value = FieldValue::Initialized(ty);
                        any = true;
                    }
                });
                if any { Some(Ty::Adt(adt)) } else { None }
            }
            Ty::Func(func) => {
                let mut func = func.clone();
                let mut any = false;
                func.parameters_mut().iter_mut().for_each(|param| {
                    if let Some(ty) = param.as_deep_normalized(sess) {
                        *param = ty;
                        any = true;
                    }
                });
                if let Some(ty) = func.return_type().as_deep_normalized(sess) {
                    *func.return_type_mut() = ty;
                    any = true;
                }
                if any { Some(Ty::Func(func)) } else { None }
            }
            Ty::Option(inner) => inner.as_deep_normalized(sess).map(|v| Ty::Option(Box::new(v))),
            _ => None,
        }
    }
}

#[must_use]
pub enum Substitution {
    Single(Var, Ty),
    Multi(Vec<(Var, Ty)>),
    None,
}
impl Substitution {
    pub fn combo(self, other: Self) -> Self {
        let mut vec = match other {
            Substitution::Single(var, ty) => vec![(var, ty)],
            Substitution::Multi(vec) => vec,
            Substitution::None => vec![],
        };
        match self {
            Substitution::Single(var, ty) => vec.push((var, ty)),
            Substitution::Multi(mut other) => vec.append(&mut other),
            Substitution::None => {}
        }
        Substitution::Multi(vec)
    }

    pub fn commit(self, subs: &mut Subs) -> Result<(), TypeError> {
        match self {
            Substitution::Single(var, ty) => subs.register(var, ty),
            Substitution::Multi(multi) => multi.into_iter().try_for_each(|(var, ty)| subs.register(var, ty)),
            Substitution::None => Ok(()),
        }
    }
}
