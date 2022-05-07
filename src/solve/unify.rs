use super::*;
use crate::duck_error;

impl<'s> Session<'s> {
    pub fn unify(lhs: &mut Ty, rhs: &mut Ty) -> Result<Substitution, TypeError> {
        // if let Some(lhs) = lhs.as_shallow_normalized(self) {
        //     return self.unify(&lhs, rhs);
        // } else if let Some(rhs) = rhs.as_shallow_normalized(self) {
        //     return self.unify(lhs, &rhs);
        // };

        println!("{}", Printer::ty_unification(lhs, rhs));

        match (lhs, rhs) {
            (lhs, rhs) if lhs == rhs => Ok(Substitution::None),
            (other, Ty::Var(var)) | (Ty::Var(var), other) => Ok(Substitution::Single(*var, other.clone())),
            (Ty::Any, _) | (_, Ty::Any) => Ok(Substitution::None),
            (und @ Ty::Undefined, other) | (other, und @ Ty::Undefined) => {
                todo!()
            }
            (Ty::Array(lhs_member), Ty::Array(rhs_member)) => Self::unify(lhs_member, rhs_member),
            (adt @ Ty::Adt(_), Ty::Identity) | (Ty::Identity, adt @ Ty::Adt(_)) => {
                unimplemented!();
            }
            (Ty::Adt(lhs_adt), Ty::Adt(rhs_adt)) => {
                let mut sub = Substitution::None;
                for (name, rhs_field) in rhs_adt.fields.iter_mut() {
                    if let FieldValue::Initialized(rhs_ty) = &mut rhs_field.value {
                        let new_sub = lhs_adt.read(name, rhs_ty)?.commit()?;
                        sub = sub.combo(new_sub);
                    }
                }
                Ok(sub)
            }
            (Ty::Func(lhs_func), Ty::Func(rhs_func)) => match (lhs_func, rhs_func) {
                (Func::Def(def), call @ Func::Call(_)) | (call @ Func::Call(_), Func::Def(def)) => {
                    let mut sub = Substitution::None;
                    // let def = def.checkout(self);
                    println!(
                        "\n--- Evaluating call for checkout: {}... ---\n",
                        Printer::ty(&Ty::Func(Func::Def(def.clone())))
                    );
                    if call.parameters().len() > def.parameters.len() {
                        return duck_error!("extra arguments provided to call");
                    }
                    for (i, param) in def.parameters.iter_mut().enumerate() {
                        if let Some(arg) = call.parameters_mut().get_mut(i) {
                            sub = sub.combo(Self::unify(param, arg)?);
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
    pub fn normalize(&mut self, sess: &Session) {
        if let Some(ty) = self.as_deep_normalized(sess) {
            *self = ty
        }
    }

    pub fn as_shallow_normalized(&self, sess: &Session) -> Option<Ty> {
        match self {
            Ty::Var(var) => sess.subs.get(var).cloned(),
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

impl<'sess> Session<'sess> {
    pub fn sub(&mut self, var: Var, mut ty: Ty) -> Result<(), TypeError> {
        #[cfg(test)]
        println!("{}", Printer::substitution(&var, &ty));
        let mut previous_sub = self.subs.remove(&var);
        if let Some(previous_sub) = &mut previous_sub {
            Session::unify(previous_sub, &mut ty)?.commit(self)?;
        }
        self.subs.insert(var, ty);
        Ok(())
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

    pub fn commit(self, sess: &mut Session) -> Result<(), TypeError> {
        match self {
            Substitution::Single(var, ty) => sess.sub(var, ty),
            Substitution::Multi(subs) => subs.into_iter().try_for_each(|(var, ty)| sess.sub(var, ty)),
            Substitution::None => Ok(()),
        }
    }
}
// impl Drop for Substitution {
//     fn drop(&mut self) {
//         if !std::thread::panicking() {
//             panic!("Failed to commit a substitution request!");
//         }
//     }
// }
