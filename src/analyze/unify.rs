use crate::analyze::{Deref, Page};

use super::{App, Constraint, Impl, Marker, Printer, Term, Type};
use colored::Colorize;
use hashbrown::{HashMap, HashSet};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Unifier {
    pub(super) substitutions: HashMap<Marker, Term>,
    pub(super) unresolved: HashSet<Marker>,
}
impl Unifier {
    pub(super) fn apply_constraints(&mut self, mut constraints: Vec<Constraint>, printer: &mut Printer) {
        while let Some(mut pattern) = constraints.pop() {
            let result = match &mut pattern {
                Constraint::Eq(marker, term) => Unifier::unify_marker(marker, term, self, printer),
                Constraint::Impl(marker, imp) => Unifier::apply_impl(marker, imp, self, printer),
            };
            if let Err(TypeError(lhs, rhs)) = result {
                panic!(
                    "hit a type error: lhs: {} rhs: {}",
                    printer.tpe(&lhs),
                    printer.tpe(&rhs)
                );
            }
        }

        // Revist those who are not resolved
        let unresolved = self.unresolved.clone();
        for marker in unresolved.iter().copied() {
            // Take this substitution out and reprocess it
            let mut term = self.substitutions.remove(&marker).unwrap();
            Self::normalize(&mut term, self);
            self.substitutions.insert(marker, term);
            // match Unifier::unify(&Term::Marker(marker), &term, self, printer) {
            //     Ok(result) => {
            //         self.substitutions.insert(marker, term);
            //         match result {
            //             UnificationResult::Resolved => {
            //                 // We are resolved, so we are good to go
            //             }
            //             UnificationResult::Unresolved => {
            //                 // We got a result, but its still not resolved. Maybe in the future
            // we                 // resolve more
            //             }
            //             UnificationResult::NoChange => {
            //                 // There was nothing we can do here. Maybe in the future we resolve
            // more             }
            //         }
            //     }
            //     Err(_) => panic!(),
            // }
        }
    }

    fn unify(
        lhs: &mut Term,
        rhs: &mut Term,
        unifier: &mut Self,
        printer: &mut Printer,
    ) -> Result<UnificationResult, TypeError> {
        println!(
            "{}  {} ~ {}",
            "UNIFY".bright_yellow(),
            printer.term(lhs),
            printer.term(rhs)
        );

        // If these terms are equal, there's nothing to do
        if lhs == rhs {
            return Ok(UnificationResult::NoChange);
        }

        // Normalize all inputs
        Self::normalize(lhs, unifier);
        Self::normalize(rhs, unifier);

        // If the lhs is a marker, unify it to the right
        if let Term::Marker(marker) = lhs {
            return Self::unify_marker(marker, rhs, unifier, printer);
        }

        // If the rhs is a marker, unify it to the left
        if let Term::Marker(marker) = rhs {
            return Self::unify_marker(marker, lhs, unifier, printer);
        }

        // Are these equivelent apps?
        if let Term::App(lhs_app) = lhs {
            match lhs_app {
                App::Array(lhs_member_type) => {
                    if let Term::App(App::Array(rhs_member_type)) = rhs {
                        Self::unify(lhs_member_type, rhs_member_type, unifier, printer)?;
                    }
                }
                App::Object(lhs_fields) => {
                    if let Term::App(App::Object(rhs_fields)) = rhs {
                        for (name, field) in lhs_fields {
                            Self::unify(field, rhs_fields.get_mut(name).expect("eh"), unifier, printer)?;
                        }
                    }
                }
                App::Function(lhs_parameters, lhs_return_type, _) => {
                    if let Term::App(App::Function(rhs_parameters, rhs_return_type, _)) = rhs {
                        for (i, param) in rhs_parameters.iter_mut().enumerate() {
                            Self::unify(&mut lhs_parameters[i], param, unifier, printer)?;
                        }
                        Self::unify(lhs_return_type, rhs_return_type, unifier, printer)?;
                    }
                }
            }
        }

        // Are these clashing types?
        if let Term::Type(lhs_type) = lhs {
            if let Term::Type(rhs_type) = rhs {
                if lhs_type != rhs_type {
                    return Err(TypeError(lhs_type.clone(), rhs_type.clone()));
                }
            }
        }

        Ok(UnificationResult::NoChange)
    }

    fn unify_marker(
        marker: &Marker,
        term: &mut Term,
        unifier: &mut Self,
        printer: &mut Printer,
    ) -> Result<UnificationResult, TypeError> {
        println!(
            "{}  {} ~ {}",
            "UNIFY".bright_yellow(),
            printer.marker(marker),
            printer.term(term),
        );

        // Ensure our term is as simple as possible
        Self::normalize(term, unifier);

        // If there is an impl, we should apply it
        if let Term::Impl(imp) = term {
            return Self::apply_impl(marker, imp, unifier, printer);
        }

        // If a substitution is already available for this marker, we will unify the term with that
        if let Some(sub) = unifier.substitutions.get(marker) {
            return Self::unify(&mut sub.clone(), term, unifier, printer);
        }

        // If the term is a deref, we might be able to translate it
        if let Term::Deref(deref) = term {
            match deref {
                Deref::Call { target, arguments } => {
                    if let Term::App(App::Function(_, _, function)) = target.as_mut() {
                        let mut page = Page::default();
                        for (i, arg) in arguments.iter().enumerate() {
                            let param = &function.parameters[i];
                            page.scope.new_field(param.name(), param.name_expr());
                            let param_marker = page.scope.get_expr_marker(param.name_expr());
                            Unifier::unify_marker(&param_marker, &mut arg.clone(), &mut page.unifier, printer)?;
                        }
                        let (_, mut return_type) = App::process_function(function.clone(), &mut page, printer);
                        return Self::unify_marker(marker, &mut return_type, unifier, printer);
                    }
                }
                Deref::MemberType { target } => return Self::unify_marker(marker, target, unifier, printer),
                Deref::Field { target, field_name } => match target.as_mut() {
                    Term::App(App::Object(fields)) => {
                        return Self::unify_marker(marker, fields.get_mut(field_name).expect("doh"), unifier, printer);
                    }
                    Term::Impl(Impl::Fields(ops)) => {
                        let term = ops.get_mut(field_name).expect("rats");
                        return Self::unify_marker(marker, term, unifier, printer);
                    }
                    _ => {}
                },
            }
        }

        // Check for occurance -- if there is any, then we get out of here
        if Self::occurs(marker, term, unifier) {
            return Ok(UnificationResult::NoChange);
        }

        unifier.new_substitution(*marker, term.clone(), printer)
    }

    fn apply_impl(
        marker: &Marker,
        imp: &mut Impl,
        unifier: &mut Self,
        printer: &mut Printer,
    ) -> Result<UnificationResult, TypeError> {
        if let Some(sub) = &mut unifier.substitutions.get_mut(marker).cloned() {
            match imp {
                Impl::Fields(imp_fields) => match sub {
                    Term::Impl(Impl::Fields(fields)) => {
                        for (name, imp_field) in imp_fields {
                            if let Some(field) = fields.get_mut(name) {
                                Self::unify(field, imp_field, unifier, printer)?;
                            } else {
                                fields.insert(name.into(), imp_field.clone());
                            }
                        }
                        unifier.new_substitution(*marker, Term::Impl(imp.clone()), printer)
                    }
                    Term::App(App::Object(fields)) => {
                        for (name, term) in imp_fields.iter_mut() {
                            Self::unify(
                                fields.get_mut(name).expect("missing field being read"),
                                term,
                                unifier,
                                printer,
                            )?;
                        }
                        Ok(UnificationResult::NoChange)
                    }
                    // maybe deref?
                    _ => Ok(UnificationResult::NoChange),
                },
            }
        } else {
            unifier.new_substitution(*marker, Term::Impl(imp.clone()), printer)
        }
    }

    fn occurs(marker: &Marker, term: &Term, unifier: &Self) -> bool {
        if let Term::Marker(term_marker) = term {
            // If the term just points to our marker, then its an occurance
            if term_marker == marker {
                return true;
            }

            // If the term exists in our substitutions, check if the sub occurs with our marker
            if let Some(sub) = unifier.substitutions.get(term_marker) {
                return Self::occurs(marker, sub, unifier);
            }
        }

        // If the term is an app, it may have our marker within it
        if let Term::App(term_app) = term {
            return match term_app {
                App::Array(member_term) => Self::occurs(marker, member_term, unifier),
                App::Object(fields) => fields.iter().any(|(_, field)| Self::occurs(marker, field, unifier)),
                App::Function(params, return_type, _) => {
                    Self::occurs(marker, return_type, unifier)
                        || params.iter().any(|param| Self::occurs(marker, param, unifier))
                }
            };
        }

        // If the term is a deref, it might be dereffing our marker
        if let Term::Deref(deref) = term {
            return match deref {
                Deref::Call { target, arguments } => {
                    if Self::occurs(marker, target, unifier) {
                        true
                    } else {
                        arguments.iter().any(|arg| Self::occurs(marker, arg, unifier))
                    }
                }
                Deref::Field { target, .. } | Deref::MemberType { target } => Self::occurs(marker, target, unifier),
            };
        }

        false
    }

    fn resolved(term: &Term) -> bool {
        match term {
            Term::Type(_) => true,
            Term::Marker(_) => false,
            Term::App(app) => match app {
                App::Array(member_term) => Self::resolved(member_term),
                App::Object(fields) => fields.iter().all(|(_, field)| Self::resolved(field)),
                App::Function(parameters, return_type, _) => {
                    parameters.iter().all(Self::resolved) && Self::resolved(return_type)
                }
            },
            Term::Deref(_) => false,
            Term::Impl(_) => false,
        }
    }

    fn normalize(term: &mut Term, unifier: &mut Self) {
        match term {
            Term::Type(_) => {}
            Term::Marker(marker) => {
                if let Some(sub) = unifier.substitutions.get(marker) {
                    *term = sub.clone();
                }
            }
            Term::App(app) => match app {
                App::Array(member_term) => Self::normalize(member_term, unifier),
                App::Object(fields) => fields.iter_mut().for_each(|(_, f)| Self::normalize(f, unifier)),
                App::Function(arguments, return_type, _) => {
                    Self::normalize(return_type, unifier);
                    arguments.iter_mut().for_each(|arg| Self::normalize(arg, unifier));
                }
            },
            Term::Deref(deref) => match deref {
                Deref::Field { target, .. } => Self::normalize(target, unifier),
                Deref::MemberType { target } => Self::normalize(target, unifier),
                Deref::Call { target, arguments } => {
                    Self::normalize(target, unifier);
                    arguments.iter_mut().for_each(|arg| Self::normalize(arg, unifier));
                }
            },
            Term::Impl(imp) => match imp {
                Impl::Fields(fields) => fields.iter_mut().for_each(|(_, term)| Self::normalize(term, unifier)),
            },
        }
    }

    fn new_substitution(
        &mut self,
        marker: Marker,
        term: Term,
        printer: &mut Printer,
    ) -> Result<UnificationResult, TypeError> {
        println!(
            "{}    {} => {}",
            "SUB".bright_green(),
            printer.marker(&marker),
            printer.term(&term),
        );

        let result = if Self::resolved(&term) {
            Ok(UnificationResult::Resolved)
        } else {
            self.unresolved.insert(marker);
            Ok(UnificationResult::Unresolved)
        };
        self.substitutions.insert(marker, term);
        result
    }
}

pub(super) enum UnificationResult {
    Resolved,
    Unresolved,
    NoChange,
}

pub(super) struct TypeError(Type, Type);
