use crate::duck_error;

use super::*;

impl Solver {
    pub fn unify_tys(&mut self, lhs: &mut Ty, rhs: &mut Ty) -> Result<(), TypeError> {
        if lhs == rhs {
            return Ok(());
        }
        // println!("{}", Printer::ty_unification(lhs, rhs, self));
        match (lhs, rhs) {
            (ty @ Ty::Uninitialized, other) | (other, ty @ Ty::Uninitialized) => {
                *ty = other.clone();
                Ok(())
            }
            (other, Ty::Var(var)) | (Ty::Var(var), other) => self.unify_var(var, other),
            (Ty::Array(lhs_member), Ty::Array(rhs_member)) => self.unify_tys(lhs_member, rhs_member),
            (Ty::Adt(lhs_adt), Ty::Adt(rhs_adt)) => {
                let rhs = self.adts.remove(rhs_adt).unwrap(); // yikes
                for (name, rhs_field) in rhs.fields.iter() {
                    self.write_adt(*lhs_adt, &crate::parse::Identifier::lazy(name), rhs_field.ty.clone())?;
                }
                self.adts.insert(*rhs_adt, rhs);
                Ok(())
            }
            (Ty::Func(lhs_func), Ty::Func(rhs_func)) => match (lhs_func, rhs_func) {
                (Func::Def(def), Func::Call(call)) | (Func::Call(call), Func::Def(def)) => {
                    #[cfg(test)]
                    println!("\n--- Calling function... ---\n",);
                    let mut solver = self.clone();
                    let mut def = def.clone();
                    for (i, param) in def.parameters.iter_mut().enumerate() {
                        if let Some(arg) = call.parameters.get_mut(i) {
                            solver.unify_tys(param, arg)?;
                        } else if i < def.minimum_arguments {
                            return duck_error!("missing argument {i} in call");
                        };
                    }
                    if call.parameters.len() > def.parameters.len() {
                        return duck_error!("extra arguments provided to call");
                    }
                    solver.normalize(&mut def.return_type);
                    // HACK: bit of a bodge here, but since the Adt's actual data is stored on the solver, not in these
                    // types, we need to do the following to transfer that data back to the real solver
                    if let &Ty::Adt(id) = def.return_type.as_ref() {
                        let adt = solver.get_adt(id);
                        self.adts.insert(id, adt.clone());
                    }

                    self.unify_tys(&mut call.return_type, &mut def.return_type)?;

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

        if !self.occurs(var, ty) {
            self.sub(*var, ty.clone());
        }
        Ok(())
    }

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
                // HACK: borrow checker issues for mutability
                let mut adt = self.adts.remove(adt_id).unwrap();
                for (_, field) in adt.fields.iter_mut() {
                    // HACK: because we're removing the adt, if it contains a reference to itself, it will not be
                    // present in the below normalization. We're safe to skip it since we're already normalizing it
                    // anyway.
                    if field.ty != Ty::Adt(*adt_id) {
                        self.normalize(&mut field.ty);
                    }
                }
                self.adts.insert(*adt_id, adt);
            }
            Ty::Func(func) => {
                func.parameters_mut().iter_mut().for_each(|v| self.normalize(v));
                self.normalize(func.return_type_mut())
            }
            _ => {}
        }
    }

    pub fn sub(&mut self, var: Var, mut ty: Ty) -> Ty {
        self.normalize(&mut ty); // todo: unsure if needed
        #[cfg(test)]
        println!("{}", Printer::substitution(&var, &ty, self));
        self.subs.insert(var, ty);
        self.subs.get(&var).unwrap().clone() // hot af clone
    }
}
