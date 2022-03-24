use crate::analyze::Deref;

use super::{App, Constraint, Impl, Marker, Printer, Term, Type};
use colored::Colorize;
use hashbrown::{HashMap, HashSet};

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Unifier {
    pub(super) substitutions: HashMap<Marker, Term>,
    pub(super) unresolved: HashSet<Marker>,
}
impl Unifier {
    pub(super) fn apply_constraints(&mut self, mut constraints: Vec<Constraint>, printer: &mut Printer) {
        while let Some(mut pattern) = constraints.pop() {
            if let Err(TypeError(lhs, rhs)) = Unifier::unify_marker(&pattern.marker, &mut pattern.term, self, printer) {
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
            // simplify(&mut term, self);
            match Unifier::unify_marker(&marker, &mut term, self, printer) {
                Ok(result) => {
                    self.substitutions.insert(marker, term);
                    match result {
                        UnificationResult::Resolved => {
                            // We are resolved, so we are good to go
                        }
                        UnificationResult::Unresolved => {
                            // We got a result, but its still not resolved. Maybe in the future we
                            // resolve more
                        }
                        UnificationResult::NoChange => {
                            // There was nothing we can do here. Maybe in the future we resolve more
                        }
                    }
                }
                Err(_) => panic!(),
            }
        }
    }

    fn unify(
        lhs: &Term,
        rhs: &Term,
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

        // If the lhs is a marker, unify it to the right
        if let Term::Marker(marker) = lhs {
            return Self::unify_marker(marker, &mut rhs.clone(), unifier, printer);
        }

        // If the rhs is a marker, unify it to the left
        if let Term::Marker(marker) = rhs {
            return Self::unify_marker(marker, &mut lhs.clone(), unifier, printer);
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
                            Self::unify(field, rhs_fields.get(name).expect("eh"), unifier, printer)?;
                        }
                    }
                }
                App::Function(lhs_parameters, lhs_return_type) => {
                    if let Term::App(App::Function(rhs_parameters, rhs_return_type)) = rhs {
                        for (i, param) in rhs_parameters.iter().enumerate() {
                            Self::unify(&lhs_parameters[i], param, unifier, printer)?;
                        }
                        Self::unify(lhs_return_type, rhs_return_type, unifier, printer)?;
                    }
                }
            }
        }

        // Do we have an implementation to check?
        if let Term::Impl(imp) = rhs {
            match imp {
                Impl::Fields(rhs_fields) => match lhs {
                    Term::App(App::Object(lhs_fields)) => {
                        println!("I could just remap {} to be {}", printer.term(lhs), printer.term(rhs));
                        for (name, field) in rhs_fields.iter() {
                            Self::unify(lhs_fields.get(name).expect("hi"), field, unifier, printer)?;
                        }
                    }
                    _ => panic!("le panique"),
                },
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
        Self::simplify(term, unifier);

        // If a substitution is already available for this marker, we will unify the term with that
        if let Some(sub) = unifier.substitutions.get(marker) {
            // Is our sub an impl? If so, we may be able to merge
            if let Term::Impl(imp) = sub {
                if let Term::Impl(other_imp) = term {
                    return Self::merge_impl(marker, &mut imp.clone(), other_imp, unifier, printer);
                }
            }
            // Otherwise, just unfiy the term with this substitute
            return Self::unify(&sub.clone(), term, unifier, printer);
        }

        // If the term is a deref, we might be able to translate it

        if let Term::Deref(deref) = term {
            match deref {
                Deref::Call { target, arguments } => {
                    if let Term::App(App::Function(parameters, return_type)) = target.as_mut() {
                        (*parameters, *return_type) = App::checkout_function(parameters, return_type);
                        for (i, param) in parameters.iter().enumerate() {
                            Self::unify(&arguments[i], param, unifier, printer)?;
                        }
                        return Self::unify_marker(marker, return_type, unifier, printer);
                    }
                }
                Deref::MemberType { target } | Deref::Field { target, .. } => {
                    return Self::unify_marker(marker, target, unifier, printer);
                }
            }
        }

        // Check for occurance -- if there is any, then we get out of here
        if Self::occurs(marker, term, unifier) {
            return Ok(UnificationResult::NoChange);
        }

        println!(
            "{}    {} => {}",
            "SUB".bright_green(),
            printer.marker(marker),
            printer.term(term)
        );
        unifier.substitutions.insert(*marker, term.clone());
        if Self::resolved(term) {
            Ok(UnificationResult::Resolved)
        } else {
            unifier.unresolved.insert(*marker);
            Ok(UnificationResult::Unresolved)
        }
    }

    fn merge_impl(
        marker: &Marker,
        imp: &mut Impl,
        other_imp: &Impl,
        unifier: &mut Self,
        printer: &mut Printer,
    ) -> Result<UnificationResult, TypeError> {
        match imp {
            Impl::Fields(fields) => {
                if let Impl::Fields(other_fields) = other_imp {
                    fields.extend(other_fields.clone());
                }
            }
        };

        println!(
            "{}    {} => {}",
            "SUB".bright_green(),
            printer.marker(marker),
            printer.imp(imp)
        );
        unifier.substitutions.insert(*marker, Term::Impl(imp.clone()));
        if Self::resolved(&Term::Impl(imp.clone())) {
            Ok(UnificationResult::Resolved)
        } else {
            unifier.unresolved.insert(*marker);
            Ok(UnificationResult::Unresolved)
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
                App::Function(params, _) => params.iter().any(|param| Self::occurs(marker, param, unifier)),
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
                _ => todo!(),
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
                App::Function(parameters, _) => parameters.iter().all(Self::resolved),
            },
            Term::Deref(_) => false,
            Term::Impl(_) => false,
        }
    }

    fn simplify(term: &mut Term, unifier: &mut Self) {
        match term {
            Term::Type(_) => {}
            Term::Marker(marker) => {
                if let Some(sub) = unifier.substitutions.get(marker) {
                    *term = sub.clone();
                }
            }
            Term::App(app) => match app {
                App::Array(member_term) => Self::simplify(member_term, unifier),
                App::Object(fields) => fields.iter_mut().for_each(|(_, f)| Self::simplify(f, unifier)),
                App::Function(arguments, return_type) => {
                    Self::simplify(return_type, unifier);
                    arguments.iter_mut().for_each(|arg| Self::simplify(arg, unifier));
                }
            },
            Term::Deref(deref) => match deref {
                Deref::Field { target, .. } => Self::simplify(target, unifier),
                Deref::MemberType { target } => Self::simplify(target, unifier),
                Deref::Call { target, arguments } => {
                    Self::simplify(target, unifier);
                    arguments.iter_mut().for_each(|arg| Self::simplify(arg, unifier));
                }
            },
            Term::Impl(imp) => match imp {
                Impl::Fields(fields) => fields.iter_mut().for_each(|(_, field)| Self::simplify(field, unifier)),
            },
        }
    }
}

pub(super) enum UnificationResult {
    Resolved,
    Unresolved,
    NoChange,
}

pub(super) struct TypeError(Type, Type);
