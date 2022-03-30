use super::*;
use crate::parse::Stmt;
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

    pub fn scope_self_trait(&self, scope: &Scope) -> Option<Trait> {
        match self.find_term(scope.self_marker) {
            Term::Trait(Trait::FieldOps(ops)) => {
                if ops.is_empty() {
                    None
                } else {
                    Some(Trait::FieldOps(ops))
                }
            }
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

        // If a substitution is already available for this marker, we will unify the term with that
        if let Some(sub) = &mut self.substitutions.get_mut(marker).cloned() {
            // If the rhs is a trait, apply it to the left
            if let Term::Trait(trt) = term {
                if !self.apply_trait(sub, trt, scope)? {
                    return Ok(());
                }
            } else {
                // Otherwise, process the terms and get out of here
                return self.unify_terms(sub, term, scope);
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

        // If the lhs is an app, we might be able to unify its interior
        if let Term::App(lhs_app) = lhs {
            match lhs_app {
                App::Array(lhs_member_type) => {
                    if let Term::App(App::Array(rhs_member_type)) = rhs {
                        self.unify_terms(lhs_member_type, rhs_member_type, scope)?;
                    }
                }
                App::Object(lhs_fields) => match rhs {
                    Term::App(App::Object(rhs_fields)) => {
                        for (name, field) in lhs_fields {
                            self.unify_terms(field, rhs_fields.get_mut(name).expect("eh"), scope)?;
                        }
                    }
                    Term::Trait(Trait::FieldOps(ops)) => {
                        for (name, field) in lhs_fields {
                            if let Some((_, op)) = ops.iter_mut().find(|(op_name, _)| *op_name == name) {
                                match op.as_mut() {
                                    FieldOp::Readable(term) | FieldOp::Writable(term) => {
                                        self.unify_terms(field, term, scope)?
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                },
                App::Function {
                    parameters: lhs_params,
                    return_type: lhs_return,
                    ..
                } => match rhs {
                    Term::App(App::Function {
                        parameters: rhs_params,
                        return_type: rhs_return,
                        ..
                    })
                    | Term::Trait(Trait::Callable {
                        arguments: rhs_params,
                        expected_return: rhs_return,
                        ..
                    }) => {
                        for (i, param) in rhs_params.iter_mut().enumerate() {
                            self.unify_terms(&mut lhs_params[i], param, scope)?;
                        }
                        self.unify_terms(lhs_return, rhs_return, scope)?;
                    }
                    _ => {}
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

        Ok(())
    }

    fn apply_trait(&mut self, term: &mut Term, trt: &mut Trait, scope: &mut Scope) -> Result<bool, TypeError> {
        println!("{}", Printer::term_impl(term, trt));
        match trt {
            Trait::FieldOps(field_ops) => match term {
                Term::App(App::Object(fields)) => {
                    for (name, field_op) in field_ops {
                        match field_op.as_mut() {
                            FieldOp::Readable(term) => {
                                self.unify_terms(fields.get_mut(name).expect("missing field being read"), term, scope)?
                            }
                            FieldOp::Writable(term) => {
                                if let Some(field) = fields.get_mut(name) {
                                    self.unify_terms(field, term, scope)?
                                } else {
                                    // add in the field then
                                    fields.insert(name.clone(), term.clone());
                                }
                            }
                        }
                    }
                }
                Term::Trait(Trait::FieldOps(term_ops)) => {
                    for (trt_name, trt_op) in field_ops {
                        // Unify the terms
                        match trt_op.as_mut() {
                            FieldOp::Readable(term) => {
                                let term_op = term_ops.get_mut(trt_name).expect("foo");
                                self.unify_terms(term_op.term_mut(), term, scope)?;
                            }
                            FieldOp::Writable(term) => {
                                if let Some(term_op) = term_ops.get_mut(trt_name) {
                                    self.unify_terms(term_op.term_mut(), term, scope)?
                                } else {
                                    // add in the field then
                                    term_ops.insert(trt_name.clone(), trt_op.clone());
                                }
                            }
                        }
                    }
                }
                _ => {}
            },
            Trait::Derive(_) => {}
            Trait::Callable {
                calling_scope,
                arguments,
                expected_return,
            } => {
                if let Term::App(App::Function {
                    self_parameter,
                    parameters,
                    return_type: func_return,
                }) = term
                {
                    // Create a temporary writer to unify this call with the function. We do this so we can
                    // learn more information about the call without applying changes to the function itself,
                    // since the function should remain generic.
                    println!("--- Starting a call... ---");
                    let mut temp_writer = self.clone();

                    // Figure out the argumentsF
                    for (i, arg) in arguments.iter_mut().enumerate() {
                        temp_writer.unify_terms(arg, &mut parameters[i], scope)?
                    }
                    temp_writer.unify_terms(expected_return, func_return, scope)?;

                    // Now use the temporary writer to normalize our return type, and unify our return type with
                    // that result
                    let mut new_return_type = expected_return.clone();
                    temp_writer.normalize_term(&mut new_return_type, scope);
                    self.unify_terms(expected_return, &mut new_return_type, scope)?;

                    // Also match up our scopes
                    if let Some(self_parameter) = self_parameter {
                        // let self_trait = if let Term::Trait(self_trait) = self_parameter.as_mut() {
                        //     self_trait
                        // } else {
                        //     unreachable!()
                        // };
                        // println!("WOW!");
                        // if let Term::Trait(Trait::FieldOps(ops)) = calling_scope.as_mut() {
                        //     self.apply_trait(calling_scope.as_mut(), self_trait, scope);
                        // } else {
                        //     unreachable!();
                        // }
                    }

                    println!("--- Call complete. ---");
                    return Ok(false);
                }
            }
        };
        Ok(true)
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
            },
            Term::Trait(trt) => match trt {
                Trait::FieldOps(field_ops) => field_ops.iter().any(|(_, op)| self.occurs(marker, op.term())),
                Trait::Derive(target) => self.occurs(marker, target),
                Trait::Callable {
                    calling_scope,
                    arguments,
                    expected_return,
                } => {
                    return self.occurs(marker, calling_scope)
                        || self.occurs(marker, expected_return)
                        || arguments.iter().any(|arg| self.occurs(marker, arg));
                }
            },
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
            },
            Term::Trait(trt) => self.normalize_trait(trt, scope),
        }
    }

    fn normalize_trait(&mut self, trt: &mut Trait, scope: &mut Scope) {
        match trt {
            Trait::FieldOps(field_ops) => field_ops
                .iter_mut()
                .for_each(|(_, op)| self.normalize_term(op.term_mut(), scope)),
            Trait::Derive(target) => {
                self.normalize_term(target, scope);
                match target.as_ref() {
                    Term::App(App::Function { self_parameter, .. }) => {
                        if let Some(self_parameter) = self_parameter {
                            if let Term::Trait(new_trt) = self_parameter.as_ref() {
                                *trt = new_trt.clone();
                            }
                        } else {
                            // this neither!
                            *trt = Trait::FieldOps(HashMap::default())
                        }
                    }
                    // This won't last lol
                    Term::Trait(Trait::Callable { .. }) => *trt = Trait::FieldOps(HashMap::default()),
                    _ => {}
                }
            }
            Trait::Callable {
                calling_scope,
                arguments,
                expected_return,
            } => {
                arguments.iter_mut().for_each(|arg| self.normalize_term(arg, scope));
                self.normalize_term(calling_scope, scope);
                self.normalize_term(expected_return, scope);
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
