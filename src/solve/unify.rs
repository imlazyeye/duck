use super::*;
use crate::duck_error;

impl<'s> Session<'s> {
    pub fn unify_ty_ty(&mut self, lhs: &Ty, rhs: &Ty) -> Result<(), TypeError> {
        if let Some(lhs) = lhs.as_shallow_normalized(self) {
            return self.unify_ty_ty(&lhs, rhs);
        } else if let Some(rhs) = rhs.as_shallow_normalized(self) {
            return self.unify_ty_ty(lhs, &rhs);
        };

        println!("{}", Printer::ty_unification(lhs, rhs));

        match (lhs, rhs) {
            (lhs, rhs) if lhs == rhs => Ok(()),
            (other, Ty::Var(var)) | (Ty::Var(var), other) => self.unify_var_ty(var, other),
            (Ty::Any, _) | (_, Ty::Any) => Ok(()),
            (und @ Ty::Undefined, other) | (other, und @ Ty::Undefined) => {
                todo!()
            }
            (Ty::Array(lhs_member), Ty::Array(rhs_member)) => self.unify_ty_ty(lhs_member, rhs_member),
            (adt @ Ty::Adt(_), Ty::Identity) | (Ty::Identity, adt @ Ty::Adt(_)) => {
                unimplemented!();
            }
            (Ty::Adt(lhs_adt), Ty::Adt(rhs_adt)) => {
                for (name, rhs_field) in rhs_adt.fields.iter() {
                    if let FieldValue::Initialized(rhs_ty) = &rhs_field.value {
                        match lhs_adt.ty(name) {
                            Some(lhs_ty) => self.unify_ty_ty(lhs_ty, rhs_ty)?,
                            None => return duck_error!("cannot find a value for `{name}`"),
                        }
                    }
                }
                Ok(())
            }
            (Ty::Func(lhs_func), Ty::Func(rhs_func)) => match (lhs_func, rhs_func) {
                (Func::Def(def), call @ Func::Call(_)) | (call @ Func::Call(_), Func::Def(def)) => {
                    #[cfg(test)]
                    let def = def.checkout(self);
                    println!(
                        "\n--- Evaluating call for checkout: {}... ---\n",
                        Printer::ty(&Ty::Func(Func::Def(def.clone())))
                    );
                    let bound_scope = def
                        .binding
                        .as_ref()
                        .map_or_else(|| self.identity_var(), |v| &v.identity_var);
                    let transmute_identity = bound_scope != self.identity_var();

                    if call.parameters().len() > def.parameters.len() {
                        return duck_error!("extra arguments provided to call");
                    }
                    for (i, param) in def.parameters.iter().enumerate() {
                        if let Some(arg) = call.parameters().get(i) {
                            if arg == &Ty::Identity && transmute_identity {
                                todo!()
                                // self.unify_tys(param, &mut Ty::Adt(self.self_id()))?;
                            } else {
                                self.unify_ty_ty(param, arg)?;
                            }
                        } else if i < def.minimum_arguments {
                            return duck_error!("missing argument {i} in call");
                        };
                    }

                    if def.return_type.as_ref() == &Ty::Identity && transmute_identity {
                        todo!()
                        // *def.return_type = Ty::Adt(bound_scope);
                    }
                    self.unify_ty_ty(call.return_type(), &def.return_type)?;
                    // *call = Func::Def(def.clone());

                    #[cfg(test)]
                    println!("\n--- Ending call... ---\n");
                    Ok(())
                }
                (Func::Def(lhs_def), Func::Def(rhs_def)) => {
                    self.unify_ty_ty(&lhs_def.return_type, &rhs_def.return_type)?;
                    rhs_def.parameters.iter().enumerate().try_for_each(|(i, rhs_param)| {
                        match lhs_def.parameters.get(i) {
                            Some(lhs_param) => self.unify_ty_ty(lhs_param, rhs_param),
                            None => duck_error!("Missing an argument"),
                        }
                    })
                }
                (Func::Call(_), Func::Call(_)) => Ok(()),
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

    pub fn unify_var_ty(&mut self, var: &Var, ty: &Ty) -> Result<(), TypeError> {
        assert!(
            ty.as_shallow_normalized(self).is_none(),
            "Var {} was bound to {}",
            Printer::var(var),
            Printer::ty(ty)
        );

        // occurs check?
        self.sub(*var, ty.clone());
        Ok(())
    }
}

impl Ty {
    pub fn normalized(self, sess: &Session) -> Self {
        if let Some(ty) = self.as_deep_normalized(sess) {
            ty
        } else {
            self
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
    pub fn sub(&mut self, var: Var, ty: Ty) {
        #[cfg(test)]
        println!("{}", Printer::substitution(&var, &ty));
        self.subs.insert(var, ty);
    }
}

// impl Solver {
//     pub fn unify_tys(&mut self, lhs: &mut Ty, rhs: &mut Ty) -> Result<(), TypeError> {
//         println!("{}", Printer::ty_unification(lhs, rhs));
//         match (lhs, rhs) {
//             (lhs, rhs) if lhs == rhs => Ok(()),
//             (ty @ Ty::Uninitialized, other) | (other, ty @ Ty::Uninitialized) => {
//                 *ty = other.clone();
//                 Ok(())
//             }
//             (other, Ty::Var(var)) | (Ty::Var(var), other) => self.unify_var(var, other),
//             (Ty::Any, _) | (_, Ty::Any) => Ok(()),
//             (und @ Ty::Undefined, other) | (other, und @ Ty::Undefined) => {
//                 *other = Ty::Option(Box::new(other.clone()));
//                 *und = other.clone();
//                 Ok(())
//             }
//             (Ty::Array(lhs_member), Ty::Array(rhs_member)) => self.unify_tys(lhs_member,
// rhs_member),             (adt @ Ty::Adt(_), Ty::Identity) | (Ty::Identity, adt @ Ty::Adt(_)) => {
//                 todo!();
//                 // self.unify_tys(adt, &mut Ty::Adt(self.self_id()))
//             }
//             (Ty::Adt(lhs_adt), Ty::Adt(rhs_adt)) => {
//                 for (name, rhs_field) in rhs_adt.fields.iter() {
//                     lhs_adt.write(name, rhs_field.ty.clone())?.commit(self)?;
//                 }
//                 Ok(())
//             }
//             (Ty::Func(lhs_func), Ty::Func(rhs_func)) => match (lhs_func, rhs_func) {
//                 (Func::Def(def), call @ Func::Call(_)) | (call @ Func::Call(_), Func::Def(def))
// => {                     #[cfg(test)]
//                     let mut def = def.checkout(self);
//                     println!(
//                         "\n--- Evaluating call for checkout: {}... ---\n",
//                         Printer::ty(&Ty::Func(Func::Def(def.clone())))
//                     );
//                     let bound_scope = def.binding.as_ref().map_or_else(|| self.self_id(), |v|
// v.self_scope());                     let transmute_identity = bound_scope != self.self_id();

//                     if call.parameters().len() > def.parameters.len() {
//                         return duck_error!("extra arguments provided to call");
//                     }
//                     for (i, param) in def.parameters.iter_mut().enumerate() {
//                         if let Some(arg) = call.parameters_mut().get_mut(i) {
//                             if arg == &Ty::Identity && transmute_identity {
//                                 todo!()
//                                 // self.unify_tys(param, &mut Ty::Adt(self.self_id()))?;
//                             } else {
//                                 self.unify_tys(param, arg)?;
//                             }
//                         } else if i < def.minimum_arguments {
//                             return duck_error!("missing argument {i} in call");
//                         };
//                     }

//                     if def.return_type.as_ref() == &Ty::Identity && transmute_identity {
//                         todo!()
//                         // *def.return_type = Ty::Adt(bound_scope);
//                     }
//                     self.unify_tys(call.return_type_mut(), &mut def.return_type)?;
//                     *call = Func::Def(def.clone());

//                     #[cfg(test)]
//                     println!("\n--- Ending call... ---\n");
//                     Ok(())
//                 }
//                 (Func::Def(lhs_def), Func::Def(rhs_def)) => {
//                     self.unify_tys(&mut lhs_def.return_type, &mut rhs_def.return_type)?;
//                     rhs_def
//                         .parameters
//                         .iter_mut()
//                         .enumerate()
//                         .try_for_each(|(i, rhs_param)| match lhs_def.parameters.get_mut(i) {
//                             Some(lhs_param) => self.unify_tys(lhs_param, rhs_param),
//                             None => duck_error!("Missing an argument"),
//                         })
//                 }
//                 (Func::Call(_), Func::Call(_)) => Ok(()),
//             },
//             (lhs, rhs) => {
//                 if lhs != rhs {
//                     #[cfg(test)]
//                     println!("Error!");
//                     duck_error!(
//                         "Attempted to equate two incompatible types: {} and {}",
//                         Printer::ty(lhs),
//                         Printer::ty(rhs)
//                     )
//                 } else {
//                     Ok(())
//                 }
//             }
//         }
//     }

//     fn unify_var(&mut self, var: &Var, ty: &mut Ty) -> Result<(), TypeError> {
//         if let Some(mut sub) = self.subs.get_mut(var).cloned() {
//             self.unify_tys(&mut sub, ty)?;
//             *ty = sub;
//         }

//         self.normalize(ty);
//         if *ty != Ty::Var(*var) {
//             self.sub(*var, ty.clone());
//         }

//         Ok(())
//     }

//     #[allow(unused)]
//     fn occurs(&self, var: &Var, ty: &Ty) -> bool {
//         match ty {
//             Ty::Var(ty_var) => ty_var == var || self.subs.get(ty_var).map_or(false, |ty|
// self.occurs(var, ty)),             Ty::Array(member_ty) => self.occurs(var, member_ty),
//             Ty::Adt(adt) => adt.fields.iter().any(|(_, field)| self.occurs(var, &field.ty)),
//             Ty::Func(func) => {
//                 self.occurs(var, func.return_type()) || func.parameters().iter().any(|v|
// self.occurs(var, v))             }
//             _ => false,
//         }
//     }

//     pub fn normalize(&mut self, ty: &mut Ty) {
//         match ty {
//             Ty::Var(var) => {
//                 if let Some(sub) = self.subs.get(var) {
//                     *ty = sub.clone();
//                     self.normalize(ty)
//                 }
//             }
//             Ty::Array(member_ty) => self.normalize(member_ty),
//             Ty::Adt(adt) => {
//                 for (_, field) in adt.fields.iter_mut() {
//                     todo!();
//                     // if field.ty.contains(ty) {
//                     //     field.ty.replace(ty, Ty::Identity);
//                     // } else {
//                     //    self.normalize(&mut field.ty);
//                     // }
//                 }
//             }
//             Ty::Func(func) => {
//                 func.parameters_mut().iter_mut().for_each(|v| self.normalize(v));
//                 self.normalize(func.return_type_mut())
//             }
//             _ => {}
//         }
//     }

//     pub fn sub(&mut self, var: Var, ty: Ty) -> Ty {
//         #[cfg(test)]
//         println!("{}", Printer::substitution(&var, &ty));
//         self.subs.insert(var, ty);
//         self.subs.get(&var).unwrap().clone() // hot af clone
//     }
// }
