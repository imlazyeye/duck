use super::*;
use crate::parse::{Identifier, Stmt};
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
        println!("\nCurrent substitutions:");
        self.substitutions
            .iter()
            .for_each(|(marker, term)| println!("{}", Printer::substitution(marker, term)));
        println!("Local fields in scope: {:?}", scope.local_fields());
    }

    pub fn take_return_term(&mut self) -> Term {
        if let Some(term) = self.substitutions.remove(&Marker::RETURN) {
            term
        } else {
            Term::Type(Type::Undefined)
        }
    }

    pub fn scope_self_traits(&self, scope: &Scope) -> Vec<Trait> {
        match self.find_term(scope.self_marker) {
            Term::Generic(fields) => fields,
            _ => unreachable!(),
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
        self.normalize_term(term, scope);

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
                                Trait::FieldOp(FieldOp::Readable(_, field))
                                | Trait::FieldOp(FieldOp::Writable(_, field)) => {
                                    self.unify_marker(marker, field, scope)?;
                                }
                                Trait::Derive(target) => self.unify_marker(marker, target, scope)?,
                                Trait::Callable(arguments, return_type) => todo!(),
                            }
                        }
                        return Ok(());
                    }
                    Term::Marker(_) => {}
                    _ => panic!("invalid obj deref target"),
                },
            }
        }

        // Check for occurance -- if there is any, then we won't register this
        if !self.occurs(marker, term) {
            self.new_substitution(*marker, term.clone(), scope)?;
        }

        Ok(())
    }

    fn unify_terms(&mut self, lhs: &mut Term, rhs: &mut Term, scope: &mut Scope) -> Result<(), TypeError> {
        // If these terms are equal, there's nothing to do
        if lhs == rhs {
            return Ok(());
        }

        // Normalize all inputs
        self.normalize_term(lhs, scope);
        self.normalize_term(rhs, scope);

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
                        for (i, param) in rhs_params.iter_mut().enumerate() {
                            self.unify_terms(&mut lhs_params[i], param, scope)?;
                        }
                        self.unify_terms(lhs_return, rhs_return, scope)?;
                    }
                }
                App::Call { function, arguments } => {
                    println!("could have ran here")
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
        self.normalize_trait(trt, scope);
        println!("{}", Printer::marker_impl(marker, trt));
        if let Some(sub) = &mut self.substitutions.get_mut(marker).cloned() {
            match trt {
                Trait::FieldOp(field_op) => match sub {
                    Term::App(App::Object(fields)) => match field_op {
                        FieldOp::Readable(name, term) => {
                            self.unify_terms(fields.get_mut(name).expect("missing field being read"), term, scope)
                        }
                        FieldOp::Writable(name, term) => {
                            if let Some(field) = fields.get_mut(name) {
                                self.unify_terms(field, term, scope)
                            } else {
                                // add in the field then
                                fields.insert(name.clone(), term.as_ref().clone());
                                self.new_substitution(*marker, sub.clone(), scope)
                            }
                        }
                    },
                    Term::Generic(sub_traits) => {
                        if let Some(sub_op) = sub_traits
                            .iter_mut()
                            .filter_map(|sub_trait| match sub_trait {
                                Trait::FieldOp(sub_op) if sub_op.name() == field_op.name() => Some(sub_op),
                                _ => None,
                            })
                            .next()
                        {
                            // Unify the terms
                            self.unify_terms(field_op.term_mut(), sub_op.term_mut(), scope)?;

                            // If our sub is currently a read, and the pattern is a write, we will upgarde
                            if let FieldOp::Readable(_, _) = sub_op {
                                if let FieldOp::Writable(_, _) = field_op {
                                    *sub_op = field_op.clone();
                                }
                            }
                        } else {
                            sub_traits.push(trt.clone());
                        }
                        self.new_substitution(*marker, Term::Generic(sub_traits.clone()), scope)
                    }
                    _ => Ok(()),
                },
                Trait::Derive(_) => self.new_substitution(*marker, Term::Generic(vec![trt.clone()]), scope),
                Trait::Callable(arguments, return_type) => match sub {
                    Term::App(App::Function {
                        self_parameter,
                        parameters,
                        return_type,
                    }) => {
                        println!("check to make sure that this works");
                        Ok(())
                    }
                    _ => Ok(()),
                },
            }
        } else {
            let term = Term::Generic(vec![trt.clone()]);
            if !self.occurs(marker, &term) {
                self.new_substitution(*marker, term, scope)
            } else {
                Ok(())
            }
        }
    }

    fn occurs(&self, marker: &Marker, term: &Term) -> bool {
        match term {
            Term::Type(_) => false,
            Term::Marker(term_marker) => {
                // If the term just points to our marker, then its an occurance
                if term_marker == marker {
                    true
                } else if let Some(sub) = self.substitutions.get(term_marker) {
                    // If the term exists in our substitutions, check if the sub occurs with our marker
                    self.occurs(marker, sub)
                } else {
                    false
                }
            }
            Term::App(term_app) => match term_app {
                App::Array(member_term) => self.occurs(marker, member_term),
                App::Object(fields) => fields.iter().any(|(_, field)| self.occurs(marker, field)),
                App::Function {
                    self_parameter,
                    parameters,
                    return_type,
                    ..
                } => {
                    self_parameter.as_ref().map_or(false, |v| self.occurs(marker, v))
                        || self.occurs(marker, return_type)
                        || parameters.iter().any(|param| self.occurs(marker, param))
                }
                App::Call { function, arguments } => {
                    self.occurs(marker, function) || arguments.iter().any(|arg| self.occurs(marker, arg))
                }
            },
            Term::Deref(deref) => match deref {
                Deref::Field { target, .. } | Deref::MemberType { target } => self.occurs(marker, target),
            },
            Term::Generic(traits) => {
                for trt in traits {
                    match trt {
                        Trait::FieldOp(field_op) => {
                            if self.occurs(marker, field_op.term()) {
                                return true;
                            }
                        }
                        Trait::Derive(target) => return self.occurs(marker, target),
                        Trait::Callable(arguments, return_type) => {
                            return self.occurs(marker, return_type)
                                || arguments.iter().any(|arg| self.occurs(marker, arg));
                        }
                    }
                }
                false
            }
        }
    }

    fn normalize_term(&mut self, term: &mut Term, scope: &mut Scope) {
        match term {
            Term::Type(_) => {}
            Term::Marker(marker) => {
                if let Some(sub) = self.substitutions.get(marker) {
                    *term = sub.clone();
                }
            }
            Term::App(app) => match app {
                App::Array(member_term) => self.normalize_term(member_term, scope),
                App::Object(fields) => fields.iter_mut().for_each(|(_, f)| self.normalize_term(f, scope)),
                App::Function {
                    self_parameter,
                    parameters,
                    return_type,
                    ..
                } => {
                    if let Some(self_parameter) = self_parameter.as_mut() {
                        self.normalize_term(self_parameter, scope);
                    }
                    self.normalize_term(return_type, scope);
                    parameters
                        .iter_mut()
                        .for_each(|param| self.normalize_term(param, scope));
                }
                App::Call { function, arguments } => {
                    self.normalize_term(function, scope);
                    arguments.iter_mut().for_each(|arg| self.normalize_term(arg, scope));
                    if let Term::App(App::Function {
                        parameters,
                        return_type,
                        self_parameter,
                        ..
                    }) = function.as_mut()
                    {
                        // Create a temporary typewriter that will be used to evaluate this call
                        let mut call_writer = self.clone();
                        let mut call_scope = Scope::new(&mut call_writer);

                        // Unify the parameters with our arguments
                        for (i, arg) in arguments.iter_mut().enumerate() {
                            call_writer
                                .unify_terms(&mut parameters[i], arg, &mut call_scope)
                                .unwrap(); // TODO THIS MUST BE THROWN
                        }

                        // Apply the traits on the functions self to our current scope
                        if let Some(self_parameter) = self_parameter {
                            call_writer.normalize_term(self_parameter, &mut call_scope);
                            match self_parameter.as_mut() {
                                Term::Generic(traits) => {
                                    for trt in traits {
                                        let self_marker = scope.self_marker;
                                        self.apply_trait(&self_marker, trt, scope).unwrap()
                                    }
                                }
                                _ => unreachable!(),
                            }
                        }

                        // Normalize the return type
                        call_writer.normalize_term(return_type, scope);
                        *term = return_type.as_ref().clone();
                    }
                }
            },
            Term::Deref(deref) => match deref {
                Deref::Field { target, .. } => self.normalize_term(target, scope),
                Deref::MemberType { target } => self.normalize_term(target, scope),
            },
            Term::Generic(traits) => {
                let iter_traits = traits.clone();
                traits.clear();
                for mut trt in iter_traits {
                    self.normalize_trait(&mut trt, scope);
                    if let Trait::Derive(target) = &mut trt {
                        if let Term::App(App::Function { self_parameter, .. }) = target.as_ref() {
                            if let Some(self_parameter) = self_parameter {
                                if let Term::Generic(new_traits) = self_parameter.as_ref() {
                                    for new_trt in new_traits {
                                        traits.push(new_trt.clone());
                                    }
                                    return;
                                }
                            }
                        }
                    }
                    traits.push(trt);
                }
            }
        }
    }

    fn normalize_trait(&mut self, trt: &mut Trait, scope: &mut Scope) {
        match trt {
            Trait::FieldOp(op) => {
                self.normalize_term(op.term_mut(), scope);
            }
            Trait::Derive(target) => {
                self.normalize_term(target, scope);
            }
            Trait::Callable(args, return_type) => {
                args.iter_mut().for_each(|arg| self.normalize_term(arg, scope));
                self.normalize_term(return_type, scope);
            }
        }
    }

    /// ### Errors
    /// shut up
    pub fn new_substitution(&mut self, marker: Marker, term: Term, scope: &mut Scope) -> Result<(), TypeError> {
        println!("{}", Printer::substitution(&marker, &term));
        self.substitutions.insert(marker, term);
        let markers_needing_updates: Vec<Marker> = self // todo why didn't i retain
            .substitutions
            .iter()
            .filter(|(_, sub_term)| self.occurs(&marker, sub_term))
            .map(|(marker, _)| *marker)
            .collect();
        for marker in markers_needing_updates {
            let mut term = self.substitutions.remove(&marker).unwrap();
            self.normalize_term(&mut term, scope);
            self.substitutions.insert(marker, term);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct TypeError(Type, Type);
