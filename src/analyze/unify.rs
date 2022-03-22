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
            "{}: {} ? {}",
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
            match rhs {
                // If the rhs is also an app, we can compare their inner terms
                Term::App(rhs_app) => match lhs_app {
                    App::Array(lhs_member_type) => match rhs_app {
                        App::Array(rhs_member_type) => Self::unify(lhs_member_type, rhs_member_type, subs, printer),
                        _ => panic!(),
                    },
                    App::Object(lhs_fields) => match rhs_app {
                        App::Object(rhs_fields) => {
                            for (name, term) in lhs_fields {
                                Self::unify(term, rhs_fields.get(name).expect("eh"), subs, printer)
                            }
                        }
                        _ => panic!("panik"),
                    },
                    _ => panic!("no"),
                },

                // If the rhs is a rule, we can apply that rule to us
                Term::Rule(rule) => {
                    if let App::Object(lhs_fields) = lhs_app {
                        match rule {
                            Rule::Field(name, term) => {
                                let lhs_field = lhs_fields.get(name).expect("app did not fufill rule");
                                Self::unify(lhs_field, term, subs, printer);
                            }
                        }
                    }
                }

                _ => {}
            }
        }
    }

    fn unify_marker(marker: &Marker, term: &mut Term, subs: &mut Self, printer: &mut Printer) {
        println!(
            "{}: {} ? {}",
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
                    if let Term::Marker(call_marker) = call_target.as_ref() {
                        if let Some(sub) = subs.collection.get(call_marker) {
                            any = true;
                            if let Term::App(App::Function(parameters, page)) = sub {
                                let (parameters, page) = App::checkout_function(parameters, page);
                                for (i, (_, param)) in parameters.iter().enumerate() {
                                    Self::unify(param, &arguments[i], subs, printer);
                                }
                                Self::unify_marker(marker, &mut page.return_term(), subs, printer);
                                *call_target = Box::new(Term::App(App::Function(parameters, page)));
                                return;
                            } else {
                                *call_target = Box::new(sub.clone());
                            }
                        }
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
            "{}:   {} => {}",
            "SUB".bright_blue(),
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
