use crate::duck_error;

use super::*;

impl Solver {
    pub fn unify_var(&mut self, var: &Var, ty: &mut Ty) -> Result<(), TypeError> {
        println!("{}", Printer::var_unification(var, ty));
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

    pub fn unify_tys(&mut self, lhs: &mut Ty, rhs: &mut Ty) -> Result<(), TypeError> {
        if lhs == rhs {
            return Ok(());
        }
        println!("{}", Printer::ty_unification(lhs, rhs));
        match (lhs, rhs) {
            (other, Ty::Var(var)) | (Ty::Var(var), other) => self.unify_var(var, other),
            (Ty::Array(lhs_member), Ty::Array(rhs_member)) => self.unify_tys(lhs_member, rhs_member),
            (Ty::Record(lhs_record), Ty::Record(rhs_record)) => rhs_record
                .fields
                .iter()
                .try_for_each(|(name, rhs_field)| lhs_record.apply_field(name, rhs_field.clone())?.apply(self)),
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

    fn occurs(&self, var: &Var, ty: &Ty) -> bool {
        match ty {
            Ty::Var(ty_var) => ty_var == var || self.subs.get(ty_var).map_or(false, |ty| self.occurs(var, ty)),
            Ty::Array(member_ty) => self.occurs(var, member_ty),
            Ty::Record(record) => record.fields.iter().any(|(_, field)| self.occurs(var, &field.ty)),
            Ty::Function(Function {
                parameters,
                return_type,
                ..
            }) => self.occurs(var, return_type) || parameters.iter().any(|v| self.occurs(var, v)),
            Ty::Call(Call { target, parameters }) => {
                self.occurs(var, target) || parameters.iter().any(|v| self.occurs(var, v))
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
            Ty::Function(super::Function {
                parameters,
                return_type,
                ..
            }) => {
                parameters.iter_mut().try_for_each(|v| self.normalize(v))?;
                self.normalize(return_type)
            }
            Ty::Call(super::Call {
                target,
                parameters: arguments,
            }) => {
                arguments.iter_mut().try_for_each(|v| self.normalize(v))?;
                self.normalize(target)?;
                match target.as_mut() {
                    Ty::Var(_) => Ok(()),
                    Ty::Function(function) => {
                        println!(
                            "\n--- Calling {}... ---\n",
                            Printer::ty(&Ty::Function(function.clone()))
                        );
                        let mut temp_writer = self.clone();
                        let mut temp_parameters = function.parameters.clone();
                        let mut temp_return = function.return_type.clone();
                        for (i, param) in temp_parameters.iter_mut().enumerate() {
                            let arg = if let Some(arg) = arguments.get_mut(i) {
                                arg
                            } else {
                                return duck_error!("Missing argument {i} in call.");
                            };
                            temp_writer.unify_tys(param, arg)?;
                        }
                        temp_writer.normalize(&mut temp_return)?;
                        *ty = *temp_return;
                        println!("\n--- Ending call... ---\n");
                        Ok(())
                    }
                    t => duck_error!("Invalid call target: {}", Printer::ty(t)),
                }
            }
            _ => Ok(()),
        }
    }

    /// ### Errors
    /// shut up
    pub fn new_substitution(&mut self, var: Var, ty: Ty) -> Result<(), TypeError> {
        // self.normalize_ty(&mut ty);
        println!("{}", Printer::substitution(&var, &ty));
        self.subs.insert(var, ty);
        // let vars_needing_updates: Vec<Var> = self
        //     .substitutions
        //     .iter()
        //     .filter(|(_, sub_ty)| self.occurs(&var, sub_ty))
        //     .map(|(var, _)| *var)
        //     .collect();
        // for var in vars_needing_updates {
        //     let mut ty = self.substitutions.remove(&var).unwrap();
        //     // self.normalize_ty(&mut ty);
        //     self.substitutions.insert(var, ty);
        // }
        Ok(())
    }
}
