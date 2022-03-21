use super::{App, Constraints, Marker, Term};
use hashbrown::HashMap;
use itertools::Itertools;

#[derive(Debug, Default)]
pub struct Unifier {
    pub(super) collection: HashMap<Marker, Term>,
}
impl Unifier {
    pub(super) fn apply_constraints(&mut self, mut constraints: Constraints) {
        while let Some(pattern) = constraints.collection.pop() {
            Unifier::unify(&Term::Marker(pattern.marker), &pattern.term, self);
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
        let mut term = term.clone();
        // If a substitution is already available for this marker, we will unify the term with that
        if let Some(sub) = subs.collection.get(marker) {
            Self::unify(&sub.clone(), &term, subs); // todo
            return;
        }

        // If the term is a marker and we have a subsitution for it, we can use that
        if let Term::Marker(other_marker) = term {
            if let Some(sub) = subs.collection.get(&other_marker) {
                Self::unify(&Term::Marker(*marker), &sub.clone(), subs); // todo
                return;
            }
        }

        // If the term is an application, we need to first simplify it
        if let Term::App(app) = &mut term {
            match app {
                App::Array(member_term) => {
                    if let Term::Marker(marker) = member_term.as_mut() {
                        if let Some(sub) = subs.collection.get(marker) {
                            *member_term = Box::new(sub.clone());
                        }
                    }
                }
                App::Object(fields) => {
                    for (_, field) in fields {
                        if let Term::Marker(marker) = field {
                            if let Some(sub) = subs.collection.get(marker) {
                                *field = sub.clone();
                            }
                        }
                    }
                }
                App::Call(_, _) => todo!(),
                App::Inspect(name, field) => {
                    if let Term::Marker(field_marker) = field.as_mut() {
                        if let Some(sub) = subs.collection.get(field_marker) {
                            match sub {
                                Term::App(App::Object(fields)) => {
                                    // We know the true type!
                                    Self::unify_marker(marker, &mut fields.get(name).expect("foorah").clone(), subs);
                                    return;
                                }
                                _ => *field = Box::new(sub.clone()),
                            }
                        }
                    }
                }
            }
        }

        // Check for occurance -- if there is any, then we get out of here
        if Self::occurs(marker, &term, subs) {
            return;
        }

        // Nothing can be subbed -- insert this into our subs
        subs.collection.insert(*marker, term.clone());
    }

    fn occurs(marker: &Marker, term: &Term, subs: &mut Self) -> bool {
        if let Term::Marker(term_marker) = term {
            // If the term just points to our marker, then its an occurance
            if term_marker == marker {
                return true;
            }

            // If the term exists in our substitutions, check if the sub occurs with our marker
            if let Some(sub) = subs.collection.get_mut(term_marker) {
                return Self::occurs(marker, &sub.clone(), subs);
            }
        }

        // If the term is an app, it may have our marker within it
        if let Term::App(term_app) = term {
            match term_app {
                App::Array(member_term) => return Self::occurs(marker, member_term, subs),
                App::Object(fields) => return fields.iter().any(|(_, field)| Self::occurs(marker, field, subs)),
                App::Call(_, _) => todo!(),
                App::Inspect(_, inspected_term) => return Self::occurs(marker, inspected_term, subs),
            }
        }

        false
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
