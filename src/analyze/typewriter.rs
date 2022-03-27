use super::*;
use crate::parse::{Function, Identifier, Stmt};
use hashbrown::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Typewriter {
    pub substitutions: HashMap<Marker, Term>,
    pub collection: Vec<Constraint>,
}

// General
impl Typewriter {
    pub fn new() -> Self {
        Self {
            substitutions: HashMap::default(),
            collection: Vec::default(),
        }
    }

    pub fn write(&mut self, scope: &mut Scope, stmts: &[Stmt]) {
        let constraints = Constraints::build(scope, self, stmts);
        self.apply_constraints(constraints, scope);
        println!("\nFinal substitutions:");
        self.substitutions
            .iter()
            .for_each(|(marker, term)| println!("{}", Printer::substitution(marker, term)));
        println!("Local fields in scope: {:?}", scope.local_fields());
        println!("Fields in self: {:?}\n", scope.namespace_fields());
    }

    pub fn take_return_term(&mut self) -> Term {
        if let Some(term) = self.substitutions.remove(&Marker::RETURN) {
            term
        } else {
            Term::Type(Type::Undefined)
        }
    }

    pub fn find_term(&self, marker: Marker) -> Term {
        self.substitutions.get(&marker).cloned().unwrap_or(Term::Marker(marker))
    }
}

impl Default for Typewriter {
    fn default() -> Self {
        Self::new()
    }
}

// Unification
impl Typewriter {
    pub fn apply_constraints(&mut self, mut constraints: Vec<Constraint>, scope: &mut Scope) {
        while let Some(mut pattern) = constraints.pop() {
            let result = match &mut pattern {
                Constraint::Eq(marker, term) => self.unify_marker(marker, term, scope),
                Constraint::Trait(marker, trt) => self.apply_trait(marker, trt, scope),
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

    pub(super) fn unify_marker(
        &mut self,
        marker: &Marker,
        term: &mut Term,
        scope: &mut Scope,
    ) -> Result<(), TypeError> {
        // Ensure our term is as simple as possible
        self.normalize(term);

        println!("{}", Printer::marker_unification(marker, term));

        // If there is an impl, we should apply it
        if let Term::Generic(traits) = term {
            for trt in traits {
                self.apply_trait(marker, trt, scope)?;
            }
            return Ok(());
        }

        // If a substitution is already available for this marker, we will unify the term with that
        if let Some(sub) = &mut self.substitutions.get_mut(marker).cloned() {
            return self.unify_terms(sub, term, scope);
        }

        // If the term is a deref, we might be able to translate it
        // Todo: move all of these to normalize
        if let Term::Deref(deref) = term {
            match deref {
                Deref::Call {
                    target,
                    arguments,
                    uses_new,
                } => {}
                Deref::MemberType { target } => match target.as_mut() {
                    Term::App(App::Array(member_type)) => {
                        return self.unify_marker(marker, member_type.as_mut(), scope);
                    }
                    Term::Type(Type::Array { member_type }) => {
                        return self.unify_marker(marker, &mut Term::Type(member_type.as_ref().clone()), scope);
                    }
                    Term::Marker(_) => {}
                    _ => panic!("invalid array deref target"),
                },
                Deref::Field { target, field_name } => match target.as_mut() {
                    Term::App(App::Object(fields)) => {
                        return self.unify_marker(marker, fields.get_mut(field_name).expect("doh"), scope);
                    }
                    Term::Generic(traits) => {
                        for trt in traits {
                            match trt {
                                Trait::FieldOp(FieldOp::Read(_, field)) | Trait::FieldOp(FieldOp::Write(_, field)) => {
                                    self.unify_marker(marker, field, scope)?;
                                }
                            }
                        }
                        return Ok(());
                    }
                    Term::Marker(_) => {}
                    Term::Deref(Deref::Call { .. }) => {}
                    _ => panic!("invalid obj deref target"),
                },
            }
        }

        // Check for occurance -- if there is any, then we won't register this
        if !self.occurs(marker, term) {
            self.new_substitution(*marker, term.clone())?;
        }

        Ok(())
    }

    fn unify_terms(&mut self, lhs: &mut Term, rhs: &mut Term, scope: &mut Scope) -> Result<(), TypeError> {
        // If these terms are equal, there's nothing to do
        if lhs == rhs {
            return Ok(());
        }

        // Normalize all inputs
        self.normalize(lhs);
        self.normalize(rhs);

        println!("{}", Printer::term_unification(lhs, rhs));

        // If the lhs is a marker, unify it to the right
        if let Term::Marker(marker) = lhs {
            return self.unify_marker(marker, rhs, scope);
        }

        // If the rhs is a marker, unify it to the left
        if let Term::Marker(marker) = rhs {
            return self.unify_marker(marker, lhs, scope);
        }

        // Are these equivelent apps?
        if let Term::App(lhs_app) = lhs {
            match lhs_app {
                App::Array(lhs_member_type) => {
                    if let Term::App(App::Array(rhs_member_type)) = rhs {
                        self.unify_terms(lhs_member_type, rhs_member_type, scope)?;
                    }
                }
                App::Object(lhs_fields) => {
                    if let Term::App(App::Object(rhs_fields)) = rhs {
                        for (name, field) in lhs_fields {
                            self.unify_terms(field, rhs_fields.get_mut(name).expect("eh"), scope)?;
                        }
                    }
                }
                App::Function {
                    parameters: lhs_params,
                    return_type: lhs_return,
                    ..
                } => {
                    if let Term::App(App::Function {
                        parameters: rhs_params,
                        return_type: rhs_return,
                        ..
                    }) = rhs
                    {
                        for (i, (_, param)) in rhs_params.iter_mut().enumerate() {
                            self.unify_terms(&mut lhs_params[i].1, param, scope)?;
                        }
                        self.unify_terms(lhs_return, rhs_return, scope)?;
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

    fn apply_trait(&mut self, marker: &Marker, trt: &mut Trait, scope: &mut Scope) -> Result<(), TypeError> {
        if let Some(sub) = &mut self.substitutions.get_mut(marker).cloned() {
            match trt {
                Trait::FieldOp(field_op) => match sub {
                    Term::App(App::Object(fields)) => match field_op {
                        FieldOp::Read(name, term) => {
                            self.unify_terms(fields.get_mut(name).expect("missing field being read"), term, scope)
                        }
                        FieldOp::Write(name, term) => {
                            if let Some(field) = fields.get_mut(name) {
                                self.unify_terms(field, term, scope)
                            } else {
                                // add in the field then
                                fields.insert(name.clone(), term.as_ref().clone());
                                self.new_substitution(*marker, sub.clone())
                            }
                        }
                    },
                    Term::Generic(sub_traits) => {
                        for sub_op in sub_traits.iter_mut().map(|sub_trait| match sub_trait {
                            Trait::FieldOp(sub_op) => sub_op,
                        }) {
                            let matches = sub_op.name() == field_op.name();
                            if matches {
                                // Unify the terms
                                self.unify_terms(field_op.term_mut(), sub_op.term_mut(), scope)?;

                                // If our sub is currently a read, and the pattern is a write, we will upgarde
                                if let FieldOp::Read(_, _) = sub_op {
                                    if let FieldOp::Write(_, _) = field_op {
                                        *sub_op = field_op.clone();
                                    }
                                }
                            }
                        }
                        self.new_substitution(*marker, Term::Generic(sub_traits.clone()))
                    }
                    _ => Ok(()),
                },
            }
        } else {
            self.new_substitution(*marker, Term::Generic(vec![trt.clone()]))
        }
    }

    fn occurs(&self, marker: &Marker, term: &Term) -> bool {
        if let Term::Marker(term_marker) = term {
            // If the term just points to our marker, then its an occurance
            if term_marker == marker {
                return true;
            }

            // If the term exists in our substitutions, check if the sub occurs with our marker
            if let Some(sub) = self.substitutions.get(term_marker) {
                return self.occurs(marker, sub);
            }
        }

        // If the term is an app, it may have our marker within it
        if let Term::App(term_app) = term {
            return match term_app {
                App::Array(member_term) => self.occurs(marker, member_term),
                App::Object(fields) => fields.iter().any(|(_, field)| self.occurs(marker, field)),
                App::Function {
                    self_parameter,
                    parameters,
                    return_type,
                    ..
                } => {
                    self_parameter
                        .as_ref()
                        .map_or(false, |param| self.occurs(marker, param))
                        || self.occurs(marker, return_type)
                        || parameters.iter().any(|(_, param)| self.occurs(marker, param))
                }
            };
        }

        // If the term is a deref, it might be dereffing our marker
        if let Term::Deref(deref) = term {
            return match deref {
                Deref::Call { target, arguments, .. } => {
                    if self.occurs(marker, target) {
                        true
                    } else {
                        arguments.iter().any(|arg| self.occurs(marker, arg))
                    }
                }
                Deref::Field { target, .. } | Deref::MemberType { target } => self.occurs(marker, target),
            };
        }

        false
    }

    fn normalize(&self, term: &mut Term) {
        match term {
            Term::Type(_) => {}
            Term::Marker(marker) => {
                if let Some(sub) = self.substitutions.get(marker) {
                    *term = sub.clone();
                }
            }
            Term::App(app) => match app {
                App::Array(member_term) => self.normalize(member_term),
                App::Object(fields) => fields.iter_mut().for_each(|(_, f)| self.normalize(f)),
                App::Function {
                    self_parameter,
                    parameters,
                    return_type,
                    ..
                } => {
                    if let Some(self_parameter) = self_parameter {
                        self.normalize(self_parameter);
                    }
                    self.normalize(return_type);
                    parameters.iter_mut().for_each(|(_, param)| self.normalize(param));
                }
            },
            Term::Deref(deref) => match deref {
                Deref::Field { target, .. } => self.normalize(target),
                Deref::MemberType { target } => self.normalize(target),
                Deref::Call { target, arguments, .. } => {
                    self.normalize(target);
                    arguments.iter_mut().for_each(|arg| self.normalize(arg));
                    if let Term::App(App::Function {
                        parameters,
                        return_type,
                        ..
                    }) = target.as_mut()
                    {
                        // Create a temporary typewriter that will be used to evaluate this call
                        let mut call_writer = self.clone();
                        let mut call_scope = Scope::new();

                        // Unify the parameters with our arguments
                        for (i, arg) in arguments.iter_mut().enumerate() {
                            let (name, param) = &mut parameters[i];
                            call_writer.unify_terms(param, arg, &mut call_scope).unwrap(); // TODO THIS MUST BE THROWN
                        }

                        // Normalize the return type
                        call_writer.normalize(return_type);
                        *term = return_type.as_ref().clone();
                    }
                }
            },
            Term::Generic(traits) => traits.iter_mut().for_each(|trt| match trt {
                Trait::FieldOp(op) => self.normalize(op.term_mut()),
            }),
        }
    }

    fn new_substitution(&mut self, marker: Marker, term: Term) -> Result<(), TypeError> {
        println!("{}", Printer::substitution(&marker, &term));
        self.substitutions.insert(marker, term);
        let markers_needing_updates: Vec<Marker> = self
            .substitutions
            .iter()
            .filter(|(_, sub_term)| self.occurs(&marker, sub_term))
            .map(|(marker, _)| *marker)
            .collect();
        for marker in markers_needing_updates {
            println!("updating {}...", Printer::marker(&marker));
            let mut term = self.substitutions.remove(&marker).unwrap();
            self.normalize(&mut term);
            println!("it is {} now", Printer::term(&term));
            self.substitutions.insert(marker, term);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct TypeError(Type, Type);
