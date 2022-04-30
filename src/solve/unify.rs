use hashbrown::HashMap;

use crate::duck_error;

use super::*;

impl Solver {
    pub fn unify_tys(&mut self, lhs: &mut Ty, rhs: &mut Ty) -> Result<(), TypeError> {
        println!("{}", Printer::ty_unification(lhs, rhs, self));
        match (lhs, rhs) {
            (lhs, rhs) if lhs == rhs => Ok(()),
            (ty @ Ty::Uninitialized, other) | (other, ty @ Ty::Uninitialized) => {
                *ty = other.clone();
                Ok(())
            }
            (other, Ty::Var(var)) | (Ty::Var(var), other) => self.unify_var(var, other),
            (Ty::Any, _) | (_, Ty::Any) => Ok(()),
            (und @ Ty::Undefined, other) | (other, und @ Ty::Undefined) => {
                *other = Ty::Option(Box::new(other.clone()));
                *und = other.clone();
                Ok(())
            }
            (Ty::Array(lhs_member), Ty::Array(rhs_member)) => self.unify_tys(lhs_member, rhs_member),
            (adt @ Ty::Adt(_), Ty::Identity) | (Ty::Identity, adt @ Ty::Adt(_)) => {
                self.unify_tys(adt, &mut Ty::Adt(self.self_id()))
            }
            (Ty::Adt(lhs_adt), Ty::Adt(rhs_adt)) => {
                let rhs = self.adts.remove(rhs_adt).unwrap(); // yikes
                for (name, rhs_field) in rhs.fields.iter() {
                    self.write_adt(*lhs_adt, &crate::parse::Identifier::lazy(name), rhs_field.ty.clone())?;
                }
                self.adts.insert(*rhs_adt, rhs);
                Ok(())
            }
            (Ty::Func(lhs_func), Ty::Func(rhs_func)) => match (lhs_func, rhs_func) {
                (Func::Def(def), call @ Func::Call(_)) | (call @ Func::Call(_), Func::Def(def)) => {
                    #[cfg(test)]
                    let mut def = def.checkout(self);
                    println!(
                        "\n--- Evaluating call for checkout: {}... ---\n",
                        Printer::ty(&Ty::Func(Func::Def(def.clone())), self)
                    );
                    let bound_scope = def.binding.as_ref().map_or_else(|| self.self_id(), |v| v.self_scope());
                    let transmute_identity = bound_scope != self.self_id();

                    if call.parameters().len() > def.parameters.len() {
                        return duck_error!("extra arguments provided to call");
                    }
                    for (i, param) in def.parameters.iter_mut().enumerate() {
                        if let Some(arg) = call.parameters_mut().get_mut(i) {
                            if arg == &Ty::Identity && transmute_identity {
                                self.unify_tys(param, &mut Ty::Adt(self.self_id()))?;
                            } else {
                                self.unify_tys(param, arg)?;
                            }
                        } else if i < def.minimum_arguments {
                            return duck_error!("missing argument {i} in call");
                        };
                    }

                    if def.return_type.as_ref() == &Ty::Identity && transmute_identity {
                        *def.return_type = Ty::Adt(bound_scope);
                    }
                    self.unify_tys(call.return_type_mut(), &mut def.return_type)?;
                    *call = Func::Def(def.clone());

                    #[cfg(test)]
                    println!("\n--- Ending call... ---\n");
                    Ok(())
                }
                (Func::Def(lhs_def), Func::Def(rhs_def)) => {
                    self.unify_tys(&mut lhs_def.return_type, &mut rhs_def.return_type)?;
                    rhs_def
                        .parameters
                        .iter_mut()
                        .enumerate()
                        .try_for_each(|(i, rhs_param)| match lhs_def.parameters.get_mut(i) {
                            Some(lhs_param) => self.unify_tys(lhs_param, rhs_param),
                            None => duck_error!("Missing an argument"),
                        })
                }
                (Func::Call(_), Func::Call(_)) => Ok(()),
            },
            (lhs, rhs) => {
                if lhs != rhs {
                    #[cfg(test)]
                    println!("Error!");
                    duck_error!(
                        "Attempted to equate two incompatible types: {} and {}",
                        Printer::ty(lhs, self),
                        Printer::ty(rhs, self)
                    )
                } else {
                    Ok(())
                }
            }
        }
    }

    fn unify_var(&mut self, var: &Var, ty: &mut Ty) -> Result<(), TypeError> {
        if let Some(mut sub) = self.subs.get_mut(var).cloned() {
            self.unify_tys(&mut sub, ty)?;
            *ty = sub;
        }

        self.normalize(ty);
        if *ty != Ty::Var(*var) {
            self.sub(*var, ty.clone());
        }

        Ok(())
    }

    #[allow(unused)]
    fn occurs(&self, var: &Var, ty: &Ty) -> bool {
        match ty {
            Ty::Var(ty_var) => ty_var == var || self.subs.get(ty_var).map_or(false, |ty| self.occurs(var, ty)),
            Ty::Array(member_ty) => self.occurs(var, member_ty),
            Ty::Adt(adt) => self
                .get_adt(*adt)
                .fields
                .iter()
                .any(|(_, field)| self.occurs(var, &field.ty)),
            Ty::Func(func) => {
                self.occurs(var, func.return_type()) || func.parameters().iter().any(|v| self.occurs(var, v))
            }
            _ => false,
        }
    }

    pub fn normalize(&mut self, ty: &mut Ty) {
        match ty {
            Ty::Var(var) => {
                if let Some(sub) = self.subs.get(var) {
                    *ty = sub.clone();
                    self.normalize(ty)
                }
            }
            Ty::Array(member_ty) => self.normalize(member_ty),
            Ty::Adt(adt_id) => {
                // HACK: borrow checker shenanigans
                // 1. we're removing becuase we need mutable access on top of our existing one
                // 2. we allow for the None case in case of a cycle.
                //
                // both of these might be wrong and required revisiting. this code went through a
                // lot of versions to get Identity working.
                if let Some(mut adt) = self.adts.remove(adt_id) {
                    for (_, field) in adt.fields.iter_mut() {
                        let search = &Ty::Adt(*adt_id);
                        if field.ty.contains(search) {
                            field.ty.replace(search, Ty::Identity);
                        } else {
                            self.normalize(&mut field.ty);
                        }
                    }
                    self.adts.insert(*adt_id, adt);
                }
            }
            Ty::Func(func) => {
                func.parameters_mut().iter_mut().for_each(|v| self.normalize(v));
                self.normalize(func.return_type_mut())
            }
            _ => {}
        }
    }

    pub fn sub(&mut self, var: Var, ty: Ty) -> Ty {
        #[cfg(test)]
        println!("{}", Printer::substitution(&var, &ty, self));
        self.subs.insert(var, ty);
        self.subs.get(&var).unwrap().clone() // hot af clone
    }
}
