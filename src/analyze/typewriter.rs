use std::sync::Arc;

use super::*;
use crate::{
    parse::{Identifier, Stmt},
    FileId,
};
use codespan_reporting::diagnostic::Diagnostic;
use hashbrown::HashMap;
use parking_lot::Mutex;

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

    pub fn return_term(&mut self) -> Term {
        if let Some(term) = self.substitutions.get(&Marker::RETURN) {
            term.clone()
        } else {
            Term::Type(Type::Undefined)
        }
    }

    pub fn find_term(&self, marker: Marker) -> Term {
        self.substitutions.get(&marker).cloned().unwrap_or(Term::Marker(marker))
    }
}

// Unification
impl Typewriter {
    pub fn apply_constraints(&mut self, mut constraints: Vec<Constraint>, scope: &mut Scope) {
        while let Some(mut pattern) = constraints.pop() {
            let result = match &mut pattern {
                Constraint::Eq(marker, term) => self.unify_marker(marker, term, scope),
                Constraint::Trait(marker, trt) => self.apply_impl(marker, trt, scope),
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
        if let Term::Trait(trt) = term {
            return self.apply_impl(marker, trt, scope);
        }

        // If a substitution is already available for this marker, we will unify the term with that
        if let Some(sub) = &mut self.substitutions.get_mut(marker).cloned() {
            return self.unify_terms(sub, term, scope);
        }

        // If the term is a deref, we might be able to translate it
        if let Term::Deref(deref) = term {
            match deref {
                Deref::Call {
                    target,
                    arguments,
                    uses_new,
                } => {
                    if let Term::App(App::Function {
                        self_parameter,
                        parameters,
                        return_type,
                        body,
                    }) = target.as_mut()
                    {
                        println!("\n--- Calling function... ---\n");
                        // Create a temporary scope and typewriter to execute our work in
                        let mut call_scope = Scope::new();
                        let mut call_writer = Typewriter::new();

                        // Unify the parameters with our arguments
                        for (i, arg) in arguments.iter_mut().enumerate() {
                            let (name, param) = &mut parameters[i];
                            let inject_marker = call_scope.inject_to_local(name.clone(), arg.clone());
                            call_writer.new_substitution(inject_marker, arg.clone())?;
                        }

                        // Run the function
                        call_writer.write(&mut call_scope, body);
                        println!("\n--- Ending call... ---\n");

                        // If we used new, we override the return type to be the self parameter
                        if *uses_new {
                            *return_type = Box::new(Term::App(App::Object(
                                call_scope
                                    .namespace_fields()
                                    .into_iter()
                                    .map(|name| {
                                        (
                                            name.clone(),
                                            call_scope
                                                .lookup_term(&Identifier::lazy(name), &call_writer)
                                                .expect("uhdsfase"),
                                        )
                                    })
                                    .collect(),
                            )));
                        } else {
                            // Otherwise, it's whatever the function actually returned.
                            *return_type = Box::new(call_writer.return_term());
                        }

                        // Apply the rules of this function to us
                        for name in call_scope.namespace_fields() {
                            if !scope.has_field(&name) {
                                let field = call_scope
                                    .lookup_term(&Identifier::lazy(name.clone()), &call_writer)
                                    .expect("uhdsfase");
                                let marker = scope.inject_to_namespace(name.clone(), field.clone());
                                self.new_substitution(marker, field.clone())?;
                            }
                        }
                        match self_parameter {
                            Some(self_term) => match self_term.as_ref() {
                                Term::Trait(Trait::Contains(fields)) => {
                                    for (name, field) in fields {
                                        if !scope.has_field(name) {
                                            let marker = scope.inject_to_namespace(name.clone(), field.clone());
                                            self.new_substitution(marker, field.clone())?;
                                        }
                                    }
                                }
                                _ => panic!(),
                            },
                            None => {}
                        }

                        // Now unify us with the result
                        return self.unify_marker(marker, return_type, scope);
                    }
                }
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
                    Term::Trait(Trait::Contains(ops)) => {
                        let term = ops.get_mut(field_name).expect("rats");
                        return self.unify_marker(marker, term, scope);
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
                    self_parameter: lhs_self,
                    parameters: lhs_params,
                    return_type: lhs_return,
                    ..
                } => {
                    if let Term::App(App::Function {
                        self_parameter: rhs_self,
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

    fn apply_impl(&mut self, marker: &Marker, trt: &mut Trait, scope: &mut Scope) -> Result<(), TypeError> {
        if let Some(sub) = &mut self.substitutions.get_mut(marker).cloned() {
            match trt {
                Trait::Contains(imp_fields) => match sub {
                    Term::Trait(Trait::Contains(fields)) => {
                        for (name, imp_field) in imp_fields {
                            if let Some(field) = fields.get_mut(name) {
                                self.unify_terms(field, imp_field, scope)?;
                            } else {
                                fields.insert(name.into(), imp_field.clone());
                            }
                        }
                        self.new_substitution(*marker, Term::Trait(trt.clone()))
                    }
                    Term::App(App::Object(fields)) => {
                        for (name, term) in imp_fields.iter_mut() {
                            self.unify_terms(fields.get_mut(name).expect("missing field being read"), term, scope)?;
                        }
                        Ok(())
                    }
                    // maybe deref?
                    _ => Ok(()),
                },
            }
        } else {
            self.new_substitution(*marker, Term::Trait(trt.clone()))
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
                }
            },
            Term::Trait(trt) => match trt {
                Trait::Contains(fields) => fields.iter_mut().for_each(|(_, term)| self.normalize(term)),
            },
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
            let mut term = self.substitutions.remove(&marker).unwrap();
            self.normalize(&mut term);
            self.substitutions.insert(marker, term);
        }
        Ok(())
    }
}

pub struct TypeError(Type, Type);
