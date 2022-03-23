use crate::analyze::Deref;

use super::{App, Constraint, Marker, Printer, Rule, Term, Type};
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

    fn unify(lhs: &Term, rhs: &Term, subs: &mut Self, printer: &mut Printer) -> Result<UnificationResult, TypeError> {
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
            return Self::unify_marker(marker, &mut rhs.clone(), subs, printer);
        }

        // If the rhs is a marker, unify it to the left
        if let Term::Marker(marker) = rhs {
            return Self::unify_marker(marker, &mut lhs.clone(), subs, printer);
        }

        // Handle an app...
        if let Term::App(lhs_app) = lhs {
            match lhs_app {
                App::Array(lhs_member_type) => {
                    if let Term::App(App::Array(rhs_member_type)) = rhs {
                        Self::unify(lhs_member_type, rhs_member_type, subs, printer)?;
                    }
                }
                App::Object(lhs_fields) => match rhs {
                    Term::App(App::Object(rhs_fields)) => {
                        for (name, field) in lhs_fields {
                            Self::unify(field, rhs_fields.get(name).expect("eh"), subs, printer)?;
                        }
                    }
                    Term::Rule(Rule::Field(name, term)) => {
                        let lhs_field = lhs_fields.get(name).expect("app did not fufill rule");
                        Self::unify(lhs_field, term, subs, printer)?;
                    }
                    _ => {}
                },
                App::Deref(deref) => match deref {
                    Deref::Call { target, arguments } => {
                        if let Term::Rule(Rule::Function(return_type, parameters)) = rhs {
                            for (i, param) in parameters.iter().enumerate() {
                                Self::unify(param, &arguments[i], subs, printer)?;
                            }
                            return Self::unify(target, rhs, subs, printer);
                        }
                    }
                    _ => todo!(),
                },
                _ => {}
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
        subs: &mut Self,
        printer: &mut Printer,
    ) -> Result<UnificationResult, TypeError> {
        println!(
            "{}  {} ~ {}",
            "UNIFY".bright_yellow(),
            printer.marker(marker),
            printer.term(term),
        );

        // If a substitution is already available for this marker, we will unify the term with that
        if let Some(sub) = subs.substitutions.get(marker) {
            return Self::unify(&sub.clone(), term, subs, printer);
        }

        // If the term is a marker and we have a subsitution for it, we can use that
        if let Term::Marker(other_marker) = term {
            if let Some(sub) = subs.substitutions.get(other_marker) {
                return Self::unify_marker(marker, &mut sub.clone(), subs, printer);
            }
        }

        // If the term is an app, see if it can be simplified
        if let Term::App(app) = term {
            match app {
                App::Array(member_term) => {
                    if let Term::Marker(member_marker) = member_term.as_ref() {
                        if let Some(sub) = subs.substitutions.get(member_marker) {
                            *member_term = Box::new(sub.clone());
                            return Self::unify_marker(marker, term, subs, printer);
                        }
                    }
                }
                App::Object(fields) => {
                    let mut any = false;
                    for (_, field) in fields {
                        if let Term::Marker(field_marker) = field {
                            if let Some(sub) = subs.substitutions.get(field_marker) {
                                any = true;
                                *field = sub.clone();
                            }
                        }
                    }
                    if any {
                        return Self::unify_marker(marker, term, subs, printer);
                    }
                }
                App::Deref(deref) => match deref {
                    Deref::Call { target, arguments } => {
                        let mut any = false;
                        for arg in arguments.iter_mut() {
                            if let Term::Marker(arg_marker) = arg {
                                if let Some(sub) = subs.substitutions.get(arg_marker) {
                                    any = true;
                                    *arg = sub.clone();
                                }
                            }
                        }
                        match target.as_ref() {
                            Term::Marker(call_marker) => {
                                if let Some(sub) = subs.substitutions.get(call_marker) {
                                    any = true;
                                    *target = Box::new(sub.clone());
                                }
                            }
                            Term::App(App::Function(parameters, page)) => {
                                let (parameters, page) = App::checkout_function(parameters, page);
                                for (i, (_, param)) in parameters.iter().enumerate() {
                                    Self::unify(param, &arguments[i], subs, printer)?;
                                }
                                let mut return_type = page.return_term();
                                *target = Box::new(Term::App(App::Function(parameters, page)));
                                return Self::unify_marker(marker, &mut return_type, subs, printer);
                            }
                            Term::Rule(Rule::Function(return_type, _)) => {
                                return Self::unify_marker(marker, &mut return_type.clone(), subs, printer);
                            }
                            _ => {}
                        }
                        if any {
                            return Self::unify_marker(marker, term, subs, printer);
                        }
                    }
                    _ => panic!(),
                },
                _ => {}
            }
        }

        // If the term is a rule, see if we can simplify it
        if let Term::Rule(rule) = term {
            match rule {
                Rule::Field(_, field) => {
                    if let Term::Marker(field_marker) = field.as_ref() {
                        if let Some(sub) = subs.substitutions.get(field_marker) {
                            *field = Box::new(sub.clone());
                            return Self::unify_marker(marker, term, subs, printer);
                        }
                    }
                }
                Rule::Function(return_type, parameters) => {
                    let mut any = false;
                    for param in parameters.iter_mut() {
                        if let Term::Marker(param_marker) = param {
                            if let Some(sub) = subs.substitutions.get(param_marker) {
                                any = true;
                                *param = sub.clone()
                            }
                        }
                    }
                    if let Term::Marker(return_marker) = return_type.as_ref() {
                        if let Some(sub) = subs.substitutions.get(return_marker) {
                            any = true;
                            *return_type = Box::new(sub.clone())
                        }
                    }
                    if any {
                        return Self::unify_marker(marker, term, subs, printer);
                    }
                }
            }
        }

        // Check for occurance -- if there is any, then we get out of here
        if Self::occurs(marker, term, subs) {
            return Ok(UnificationResult::NoChange);
        }

        println!(
            "{}    {} => {}",
            "SUB".bright_green(),
            printer.marker(&marker),
            printer.term(&term)
        );
        subs.substitutions.insert(*marker, term.clone());
        if Self::resolved(&term) {
            Ok(UnificationResult::Resolved)
        } else {
            subs.unresolved.insert(*marker);
            Ok(UnificationResult::Unresolved)
        }
    }

    fn occurs(marker: &Marker, term: &Term, subs: &Self) -> bool {
        if let Term::Marker(term_marker) = term {
            // If the term just points to our marker, then its an occurance
            if term_marker == marker {
                return true;
            }

            // If the term exists in our substitutions, check if the sub occurs with our marker
            if let Some(sub) = subs.substitutions.get(term_marker) {
                return Self::occurs(marker, sub, subs);
            }
        }

        // If the term is an app, it may have our marker within it
        if let Term::App(term_app) = term {
            return match term_app {
                App::Array(member_term) => Self::occurs(marker, member_term, subs),
                App::Object(fields) => fields.iter().any(|(_, field)| Self::occurs(marker, field, subs)),
                App::Deref(deref) => match deref {
                    Deref::Call { target, arguments } => {
                        if Self::occurs(marker, target, subs) {
                            true
                        } else {
                            arguments.iter().any(|arg| Self::occurs(marker, arg, subs))
                        }
                    }
                    _ => todo!(),
                },
                App::Function(params, _) => params.iter().any(|(_, param)| Self::occurs(marker, param, subs)),
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
                App::Function(parameters, _) => parameters.iter().all(|(_, param)| Self::resolved(param)),
                App::Deref(_) => false,
            },
            Term::Rule(_) => false,
        }
    }
}

pub(super) enum UnificationResult {
    Resolved,
    Unresolved,
    NoChange,
}

pub(super) struct TypeError(Type, Type);
