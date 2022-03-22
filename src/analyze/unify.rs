use super::{App, Constraints, Marker, Term};
use hashbrown::HashMap;
use itertools::Itertools;

#[derive(Debug, Default, Clone)]
pub struct Unifier {
    pub(super) collection: HashMap<Marker, Term>,
}
impl Unifier {
    pub(super) fn apply_constraints(&mut self, mut constraints: Constraints) {
        while let Some(mut pattern) = constraints.collection.pop() {
            Self::simplify_term(&mut pattern.term, self);
            Unifier::unify_marker(&pattern.marker, &mut pattern.term, self);
        }
    }

    fn unify(lhs: &Term, rhs: &Term, subs: &mut Self) {
        // If these terms are equal, there's nothing to do
        if lhs == rhs {
            return;
        }

        // If the lhs is a marker, unify it to the right
        if let Term::Marker(marker) = lhs {
            Self::unify_marker(marker, &mut rhs.clone(), subs);
            return;
        }

        // If the rhs is a marker, unify it to the left
        if let Term::Marker(marker) = rhs {
            Self::unify_marker(marker, &mut lhs.clone(), subs);
            return;
        }

        // If these are both applications, we can unify them together
        if let Term::App(lhs_app) = lhs {
            if let Term::App(rhs_app) = rhs {
                match lhs_app {
                    App::Array(lhs_member_type) => match rhs_app {
                        App::Array(rhs_member_type) => Self::unify(lhs_member_type, rhs_member_type, subs),
                        _ => panic!(),
                    },
                    App::Object(lhs_fields) => match rhs_app {
                        App::Object(rhs_fields) => {
                            for (name, term) in lhs_fields {
                                Self::unify(term, rhs_fields.get(name).expect("eh"), subs)
                            }
                        }
                        _ => panic!("panik"),
                    },
                    App::Call(call_target, arguments) => todo!(),
                    App::Inspect(name, inspected_term) => match rhs_app {
                        App::Object(fields) => Self::unify(inspected_term, fields.get(name).expect("ehh"), subs),
                        _ => panic!(),
                    },
                }
            }
        }
    }

    fn unify_marker(marker: &Marker, term: &mut Term, subs: &mut Self) {
        // If a substitution is already available for this marker, we will unify the term with that
        if let Some(sub) = subs.collection.get(marker) {
            Self::unify(&sub.clone(), term, subs); // todo
            return;
        }

        // If the term is a marker and we have a subsitution for it, we can use that
        if let Term::Marker(other_marker) = term {
            if let Some(sub) = subs.collection.get(other_marker) {
                Self::unify(&Term::Marker(*marker), &sub.clone(), subs); // todo
                return;
            }
        }

        // Check for occurance -- if there is any, then we get out of here
        if Self::occurs(marker, term, subs) {
            return;
        }

        // Time to register a new sub. Let's apply it to all previous subs
        subs.collection.insert(*marker, term.clone());
        let oh_no = subs.clone();
        for (_, other_term) in subs.collection.iter_mut() {
            Self::simplify_term(other_term, &oh_no);
        }
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
                App::Inspect(_, inspected_term) => Self::occurs(marker, inspected_term, subs),
            };
        }

        false
    }

    fn simplify_term(term: &mut Term, subs: &Self) {
        match term {
            Term::Marker(inner_marker) => {
                if let Some(new_term) = subs.collection.get(inner_marker) {
                    *term = new_term.clone()
                }
            }
            Term::App(app) => match app {
                App::Array(member_term) => {
                    Self::simplify_term(member_term.as_mut(), subs);
                }
                App::Object(fields) => {
                    for (_, field) in fields {
                        Self::simplify_term(field, subs);
                    }
                }
                App::Call(call_target, arguments) => {
                    Self::simplify_term(call_target, subs);
                    for arg in arguments {
                        Self::simplify_term(arg, subs);
                    }
                }
                App::Inspect(name, field) => {
                    Self::simplify_term(field.as_mut(), subs);
                    if let Term::App(App::Object(fields)) = field.as_ref() {
                        // We know the true type!
                        *term = fields.get(name).expect("foorah").clone();
                    }
                }
            },
            _ => {}
        }
    }
}

impl std::fmt::Display for Unifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(
            &self
                .collection
                .iter()
                .map(|(marker, term)| format!("{marker} => {term}"))
                .join("\n"),
        )
    }
}
