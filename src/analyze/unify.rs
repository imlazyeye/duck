use crate::duck_error;

use super::*;

impl Solver {
    pub fn unify_tys(&mut self, lhs: &mut Ty, rhs: &mut Ty) -> Result<(), TypeError> {
        if lhs == rhs {
            return Ok(());
        }
        println!("{}", Printer::ty_unification(lhs, rhs));

        // General unification
        match (lhs, rhs) {
            (other, Ty::Var(var)) | (Ty::Var(var), other) => self.unify_var(var, other),
            (Ty::Array(lhs_member), Ty::Array(rhs_member)) => self.unify_tys(lhs_member, rhs_member),
            (Ty::Record(lhs_record), Ty::Record(rhs_record)) => rhs_record
                .fields
                .iter()
                .try_for_each(|(name, rhs_field)| lhs_record.apply_field(name, rhs_field.clone())?.commit(self)),
            (Ty::Func(lhs_func), Ty::Func(rhs_func)) => match (lhs_func, rhs_func) {
                (Func::Def(def), Func::Call(call)) | (Func::Call(call), Func::Def(def)) => {
                    println!("\n--- Calling function... ---\n",);
                    let mut solver = self.clone();
                    let mut def = def.clone();
                    for (i, param) in def.parameters.iter_mut().enumerate() {
                        let arg = if let Some(arg) = call.parameters.get_mut(i) {
                            arg
                        } else {
                            return duck_error!("Missing argument {i} in call.");
                        };
                        solver.unify_tys(param, arg)?;
                    }
                    solver.normalize(&mut def.return_type)?;
                    self.unify_tys(&mut call.return_type, &mut def.return_type)?;
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
                    println!("Error!");
                    duck_error!(
                        "Attempted to equate two incompatible types: {} and {}",
                        Printer::ty(lhs),
                        Printer::ty(rhs)
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
            self.new_substitution(*var, ty.clone())
        } else {
            Ok(())
        }
    }

    fn occurs(&self, var: &Var, ty: &Ty) -> bool {
        match ty {
            Ty::Var(ty_var) => ty_var == var || self.subs.get(ty_var).map_or(false, |ty| self.occurs(var, ty)),
            Ty::Array(member_ty) => self.occurs(var, member_ty),
            Ty::Record(record) => record.fields.iter().any(|(_, field)| self.occurs(var, &field.ty)),
            Ty::Func(func) => {
                self.occurs(var, func.return_type()) || func.parameters().iter().any(|v| self.occurs(var, v))
            }
            _ => false,
        }
    }

    pub fn normalize(&self, ty: &mut Ty) -> Result<(), TypeError> {
        match ty {
            Ty::Var(var) => {
                if let Some(sub) = self.subs.get(var) {
                    *ty = sub.clone();
                    self.normalize(ty)
                } else {
                    Ok(())
                }
            }
            Ty::Array(member_ty) => self.normalize(member_ty),
            Ty::Record(record) => record
                .fields
                .iter_mut()
                .try_for_each(|(_, field)| self.normalize(&mut field.ty)),
            Ty::Func(func) => {
                func.parameters_mut().iter_mut().try_for_each(|v| self.normalize(v))?;
                self.normalize(func.return_type_mut())
            }
            _ => Ok(()),
        }
    }

    /// ### Errors
    /// shut up
    pub fn new_substitution(&mut self, var: Var, ty: Ty) -> Result<(), TypeError> {
        println!("{}", Printer::substitution(&var, &ty));
        self.subs.insert(var, ty);
        Ok(())
    }
}
