use super::{App, Constraint, Marker, Printer, Rule, Term};
use colored::Colorize;
use hashbrown::HashMap;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Unifier {
    pub(super) collection: HashMap<Marker, Term>,
}
impl Unifier {
    pub(super) fn apply_constraints(&mut self, mut constraints: Vec<Constraint>, printer: &mut Printer) {
        while let Some(mut pattern) = constraints.pop() {
            Unifier::unify_marker(&pattern.marker, &mut pattern.term, self, printer);
        }
    }

    fn unify(lhs: &Term, rhs: &Term, subs: &mut Self, printer: &mut Printer) {
        println!(
            "{}  {} ~ {}",
            "UNIFY".bright_yellow(),
            printer.term(lhs),
            printer.term(rhs)
        );
        // If these terms are equal, there's nothing to do
        if lhs == rhs {
            return;
        }

        // If the lhs is a marker, unify it to the right
        if let Term::Marker(marker) = lhs {
            Self::unify_marker(marker, &mut rhs.clone(), subs, printer);
            return;
        }

        // If the rhs is a marker, unify it to the left
        if let Term::Marker(marker) = rhs {
            Self::unify_marker(marker, &mut lhs.clone(), subs, printer);
            return;
        }

        // Handle an app...
        if let Term::App(lhs_app) = lhs {
            match lhs_app {
                App::Array(lhs_member_type) => {
                    if let Term::App(App::Array(rhs_member_type)) = rhs {
                        Self::unify(lhs_member_type, rhs_member_type, subs, printer)
                    }
                }
                App::Object(lhs_fields) => match rhs {
                    Term::App(App::Object(rhs_fields)) => {
                        for (name, field) in lhs_fields {
                            Self::unify(field, rhs_fields.get(name).expect("eh"), subs, printer)
                        }
                    }
                    // If the rhs is a rule, we can apply that rule to us
                    Term::Rule(rule) => {
                        if let App::Object(lhs_fields) = lhs_app {
                            match rule {
                                Rule::Field(name, term) => {
                                    let lhs_field = lhs_fields.get(name).expect("app did not fufill rule");
                                    Self::unify(lhs_field, term, subs, printer);
                                }
                                _ => panic!(),
                            }
                        }
                    }
                    _ => {}
                },
                App::Call(lhs_target, lhs_args) => match rhs {
                    Term::App(App::Call(rhs_target, rhs_args)) => {
                        if lhs_target == rhs_target {
                            for (i, arg) in lhs_args.iter().enumerate() {
                                Self::unify(arg, &rhs_args[i], subs, printer)
                            }
                        }
                    }
                    _ => {
                        Self::unify(
                            lhs_target,
                            &Term::Rule(Rule::Function(Box::new(rhs.clone()), lhs_args.clone())),
                            subs,
                            printer,
                        );
                        // Self::unify(lhs, &rhs.clone(), subs, printer);
                    }
                },
                App::Function(_, _) => {}
            }
        }
    }

    fn unify_marker(marker: &Marker, term: &mut Term, subs: &mut Self, printer: &mut Printer) {
        println!(
            "{}  {} ~ {}",
            "UNIFY".bright_yellow(),
            printer.marker(marker),
            printer.term(term),
        );
        // If a substitution is already available for this marker, we will unify the term with that
        if let Some(sub) = subs.collection.get(marker) {
            Self::unify(&sub.clone(), term, subs, printer); // todo
            return;
        }

        // If the term is a marker and we have a subsitution for it, we can use that
        if let Term::Marker(other_marker) = term {
            if let Some(sub) = subs.collection.get(other_marker) {
                Self::unify(&Term::Marker(*marker), &sub.clone(), subs, printer); // todo
                return;
            }
        }

        // If the term is an app, see if it can be simplified
        if let Term::App(app) = term {
            match app {
                App::Array(member_term) => {
                    if let Term::Marker(member_marker) = member_term.as_ref() {
                        if let Some(sub) = subs.collection.get(member_marker) {
                            *member_term = Box::new(sub.clone());
                            Self::unify_marker(marker, term, subs, printer);
                            return;
                        }
                    }
                }
                App::Object(fields) => {
                    let mut any = false;
                    for (_, field) in fields {
                        if let Term::Marker(field_marker) = field {
                            if let Some(sub) = subs.collection.get(field_marker) {
                                any = true;
                                *field = sub.clone();
                            }
                        }
                    }
                    if any {
                        Self::unify_marker(marker, term, subs, printer);
                        return;
                    }
                }
                App::Call(call_target, arguments) => {
                    let mut any = false;
                    for arg in arguments.iter_mut() {
                        if let Term::Marker(arg_marker) = arg {
                            if let Some(sub) = subs.collection.get(arg_marker) {
                                any = true;
                                *arg = sub.clone();
                            }
                        }
                    }
                    match call_target.as_ref() {
                        Term::Marker(call_marker) => {
                            if let Some(sub) = subs.collection.get(call_marker) {
                                any = true;
                                *call_target = Box::new(sub.clone());
                            }
                        }
                        Term::App(App::Function(parameters, page)) => {
                            let (parameters, page) = App::checkout_function(parameters, page);
                            for (i, (_, param)) in parameters.iter().enumerate() {
                                Self::unify(param, &arguments[i], subs, printer);
                            }
                            Self::unify_marker(marker, &mut page.return_term(), subs, printer);
                            *call_target = Box::new(Term::App(App::Function(parameters, page)));
                            return;
                        }
                        Term::Rule(Rule::Function(return_type, parameters)) => {
                            for (i, param) in parameters.iter().enumerate() {
                                Self::unify(param, &arguments[i], subs, printer);
                            }
                            Self::unify_marker(marker, &mut return_type.as_ref().clone(), subs, printer);
                            return;
                        }
                        _ => {}
                    }
                    if any {
                        Self::unify_marker(marker, term, subs, printer);
                        return;
                    }
                }
                _ => {}
            }
        }

        // Check for occurance -- if there is any, then we get out of here
        if Self::occurs(marker, term, subs) {
            return;
        }

        // Time to register a new sub
        println!(
            "{}    {} => {}",
            "SUB".bright_green(),
            printer.marker(marker),
            printer.term(term)
        );
        subs.collection.insert(*marker, term.clone());
    }

    fn occurs(marker: &Marker, term: &Term, subs: &Self) -> bool {
        if let Term::Marker(term_marker) = term {
            // If the term just points to our marker, then its an occurance
            if term_marker == marker {
                return true;
            }

            // If the term exists in our substitutions, check if the sub occurs with our marker
            if let Some(sub) = subs.collection.get(term_marker) {
                return Self::occurs(marker, sub, subs);
            }
        }

        // If the term is an app, it may have our marker within it
        if let Term::App(term_app) = term {
            return match term_app {
                App::Array(member_term) => Self::occurs(marker, member_term, subs),
                App::Object(fields) => fields.iter().any(|(_, field)| Self::occurs(marker, field, subs)),
                App::Call(call_target, arguments) => {
                    if Self::occurs(marker, call_target, subs) {
                        true
                    } else {
                        arguments.iter().any(|arg| Self::occurs(marker, arg, subs))
                    }
                }
                App::Function(params, _) => params.iter().any(|(_, param)| Self::occurs(marker, param, subs)),
            };
        }

        false
    }
}
