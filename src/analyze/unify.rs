use super::*;

pub(super) struct Unification;
impl Unification {
    pub fn apply_constraints(mut constraints: Vec<Constraint>, typewriter: &mut Typewriter) {
        while let Some(mut pattern) = constraints.pop() {
            let result = match &mut pattern {
                Constraint::Eq(marker, term) => Unification::unify_marker(marker, term, typewriter),
                Constraint::Impl(marker, imp) => Unification::apply_impl(marker, imp, typewriter),
            };
            if let Err(TypeError(lhs, rhs)) = result {
                panic!(
                    "hit a type error: lhs: {} rhs: {}",
                    Printer::tpe(&lhs),
                    Printer::tpe(&rhs)
                );
            }
        }
    }

    fn unify_marker(marker: &Marker, term: &mut Term, typewriter: &mut Typewriter) -> Result<(), TypeError> {
        // Ensure our term is as simple as possible
        Self::normalize(term, typewriter);

        println!("{}", Printer::marker_unification(marker, term));

        // If there is an impl, we should apply it
        if let Term::Impl(imp) = term {
            return Self::apply_impl(marker, imp, typewriter);
        }

        // If a substitution is already available for this marker, we will unify the term with that
        if let Some(sub) = typewriter.substitutions.get(marker) {
            return Self::unify_terms(&mut sub.clone(), term, typewriter);
        }

        // If the term is a deref, we might be able to translate it
        if let Term::Deref(deref) = term {
            match deref {
                Deref::Call { target, arguments } => {
                    if let Term::App(App::Function(_, _, function)) = target.as_mut() {
                        let mut new_writer = Typewriter::new(typewriter.scope.clone());
                        for (i, arg) in arguments.iter().enumerate() {
                            let param = &function.parameters[i];
                            new_writer.scope.new_field(param.name(), param.name_expr());
                            let param_marker = new_writer.scope.get_expr_marker(param.name_expr());
                            Unification::unify_marker(&param_marker, &mut arg.clone(), &mut new_writer)?;
                        }
                        let (_, mut return_type) = App::process_function(function.clone(), &mut new_writer);
                        return Self::unify_marker(marker, &mut return_type, typewriter);
                    }
                }
                Deref::MemberType { target } => match target.as_mut() {
                    Term::App(App::Array(member_type)) => {
                        return Self::unify_marker(marker, member_type.as_mut(), typewriter);
                    }
                    Term::Type(Type::Array { member_type }) => {
                        return Self::unify_marker(marker, &mut Term::Type(member_type.as_ref().clone()), typewriter);
                    }
                    Term::Marker(_) => {}
                    _ => panic!("invalid array deref target"),
                },
                Deref::Field { target, field_name } => match target.as_mut() {
                    Term::App(App::Object(fields)) => {
                        return Self::unify_marker(marker, fields.get_mut(field_name).expect("doh"), typewriter);
                    }
                    Term::Impl(Impl::Fields(ops)) => {
                        let term = ops.get_mut(field_name).expect("rats");
                        return Self::unify_marker(marker, term, typewriter);
                    }
                    Term::Marker(_) => {}
                    _ => panic!("invalid obj deref target"),
                },
            }
        }

        // Check for occurance -- if there is any, then we get out of here
        if Self::occurs(marker, term, typewriter) {
            return Ok(());
        }

        Self::new_substitution(*marker, term.clone(), typewriter)
    }

    fn unify_terms(lhs: &mut Term, rhs: &mut Term, typewriter: &mut Typewriter) -> Result<(), TypeError> {
        // If these terms are equal, there's nothing to do
        if lhs == rhs {
            return Ok(());
        }

        // Normalize all inputs
        Self::normalize(lhs, typewriter);
        Self::normalize(rhs, typewriter);

        println!("{}", Printer::term_unification(lhs, rhs));

        // If the lhs is a marker, unify it to the right
        if let Term::Marker(marker) = lhs {
            return Self::unify_marker(marker, rhs, typewriter);
        }

        // If the rhs is a marker, unify it to the left
        if let Term::Marker(marker) = rhs {
            return Self::unify_marker(marker, lhs, typewriter);
        }

        // Are these equivelent apps?
        if let Term::App(lhs_app) = lhs {
            match lhs_app {
                App::Array(lhs_member_type) => {
                    if let Term::App(App::Array(rhs_member_type)) = rhs {
                        Self::unify_terms(lhs_member_type, rhs_member_type, typewriter)?;
                    }
                }
                App::Object(lhs_fields) => {
                    if let Term::App(App::Object(rhs_fields)) = rhs {
                        for (name, field) in lhs_fields {
                            Self::unify_terms(field, rhs_fields.get_mut(name).expect("eh"), typewriter)?;
                        }
                    }
                }
                App::Function(lhs_parameters, lhs_return_type, _) => {
                    if let Term::App(App::Function(rhs_parameters, rhs_return_type, _)) = rhs {
                        for (i, param) in rhs_parameters.iter_mut().enumerate() {
                            Self::unify_terms(&mut lhs_parameters[i], param, typewriter)?;
                        }
                        Self::unify_terms(lhs_return_type, rhs_return_type, typewriter)?;
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

        Ok(())
    }

    fn apply_impl(marker: &Marker, imp: &mut Impl, typewriter: &mut Typewriter) -> Result<(), TypeError> {
        if let Some(sub) = &mut typewriter.substitutions.get_mut(marker).cloned() {
            match imp {
                Impl::Fields(imp_fields) => match sub {
                    Term::Impl(Impl::Fields(fields)) => {
                        for (name, imp_field) in imp_fields {
                            if let Some(field) = fields.get_mut(name) {
                                Self::unify_terms(field, imp_field, typewriter)?;
                            } else {
                                fields.insert(name.into(), imp_field.clone());
                            }
                        }
                        Self::new_substitution(*marker, Term::Impl(imp.clone()), typewriter)
                    }
                    Term::App(App::Object(fields)) => {
                        for (name, term) in imp_fields.iter_mut() {
                            Self::unify_terms(
                                fields.get_mut(name).expect("missing field being read"),
                                term,
                                typewriter,
                            )?;
                        }
                        Ok(())
                    }
                    // maybe deref?
                    _ => Ok(()),
                },
            }
        } else {
            Self::new_substitution(*marker, Term::Impl(imp.clone()), typewriter)
        }
    }

    fn occurs(marker: &Marker, term: &Term, typewriter: &Typewriter) -> bool {
        if let Term::Marker(term_marker) = term {
            // If the term just points to our marker, then its an occurance
            if term_marker == marker {
                return true;
            }

            // If the term exists in our substitutions, check if the sub occurs with our marker
            if let Some(sub) = typewriter.substitutions.get(term_marker) {
                return Self::occurs(marker, sub, typewriter);
            }
        }

        // If the term is an app, it may have our marker within it
        if let Term::App(term_app) = term {
            return match term_app {
                App::Array(member_term) => Self::occurs(marker, member_term, typewriter),
                App::Object(fields) => fields.iter().any(|(_, field)| Self::occurs(marker, field, typewriter)),
                App::Function(params, return_type, _) => {
                    Self::occurs(marker, return_type, typewriter)
                        || params.iter().any(|param| Self::occurs(marker, param, typewriter))
                }
            };
        }

        // If the term is a deref, it might be dereffing our marker
        if let Term::Deref(deref) = term {
            return match deref {
                Deref::Call { target, arguments } => {
                    if Self::occurs(marker, target, typewriter) {
                        true
                    } else {
                        arguments.iter().any(|arg| Self::occurs(marker, arg, typewriter))
                    }
                }
                Deref::Field { target, .. } | Deref::MemberType { target } => Self::occurs(marker, target, typewriter),
            };
        }

        false
    }

    fn normalize(term: &mut Term, typewriter: &Typewriter) {
        match term {
            Term::Type(_) => {}
            Term::Marker(marker) => {
                if let Some(sub) = typewriter.substitutions.get(marker) {
                    *term = sub.clone();
                }
            }
            Term::App(app) => match app {
                App::Array(member_term) => Self::normalize(member_term, typewriter),
                App::Object(fields) => fields.iter_mut().for_each(|(_, f)| Self::normalize(f, typewriter)),
                App::Function(arguments, return_type, _) => {
                    Self::normalize(return_type, typewriter);
                    arguments.iter_mut().for_each(|arg| Self::normalize(arg, typewriter));
                }
            },
            Term::Deref(deref) => match deref {
                Deref::Field { target, .. } => Self::normalize(target, typewriter),
                Deref::MemberType { target } => Self::normalize(target, typewriter),
                Deref::Call { target, arguments } => {
                    Self::normalize(target, typewriter);
                    arguments.iter_mut().for_each(|arg| Self::normalize(arg, typewriter));
                }
            },
            Term::Impl(imp) => match imp {
                Impl::Fields(fields) => fields
                    .iter_mut()
                    .for_each(|(_, term)| Self::normalize(term, typewriter)),
            },
        }
    }

    fn new_substitution(marker: Marker, term: Term, typewriter: &mut Typewriter) -> Result<(), TypeError> {
        println!("{}", Printer::substitution(&marker, &term));
        typewriter.substitutions.insert(marker, term);
        let markers_needing_updates: Vec<Marker> = typewriter
            .substitutions
            .iter()
            .filter(|(_, sub_term)| Unification::occurs(&marker, sub_term, typewriter))
            .map(|(marker, _)| *marker)
            .collect();
        for marker in markers_needing_updates {
            let mut term = typewriter.substitutions.remove(&marker).unwrap();
            Self::normalize(&mut term, typewriter);
            typewriter.substitutions.insert(marker, term);
        }
        Ok(())
    }
}

pub struct TypeError(Type, Type);
