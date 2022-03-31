use super::*;
use crate::parse::Stmt;
use hashbrown::HashMap;
use itertools::Itertools;

#[derive(Debug, PartialEq, Clone)]
pub struct Typewriter {
    pub substitutions: HashMap<Marker, Term>,
    pub constraints: Vec<Constraint>,
    pub scope: Scope,
}

// General
impl Typewriter {
    pub fn new(scope: Scope) -> Self {
        Self {
            substitutions: HashMap::default(),
            constraints: Vec::default(),
            scope,
        }
    }

    pub fn new_from(other: &Self, scope: Scope) -> Self {
        Self {
            substitutions: other.substitutions.clone(),
            constraints: other.constraints.clone(),
            scope,
        }
    }

    pub fn write(&mut self, stmts: &[Stmt]) {
        let constraints = Constraints::build(self, stmts);
        self.apply_constraints(constraints);
        let mut oh_no = self.scope.clone();
        for (_, term) in oh_no.self_fields.iter_mut() {
            self.normalize_term(term.term_mut());
        }
        self.scope = oh_no;
        println!("\nCurrent substitutions:");
        self.substitutions
            .iter()
            .for_each(|(marker, term)| println!("{}", Printer::substitution(marker, term)));
        println!("Local fields in scope: {:?}", self.scope.local_fields());
        println!(
            "Fields in self: [{}]",
            self.scope
                .self_fields
                .iter()
                .map(|(name, op)| format!("{name}: {}", Printer::term(op.term())))
                .join(", ")
        );
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
        Self::new(Scope::default())
    }
}

// Unification
impl Typewriter {
    pub fn apply_constraints(&mut self, mut constraints: Vec<Constraint>) {
        while let Some(mut pattern) = constraints.pop() {
            let result = match &mut pattern {
                Constraint::Eq(marker, term) => self.unify_marker(marker, term),
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

    pub(super) fn unify_marker(&mut self, marker: &Marker, term: &mut Term) -> Result<(), TypeError> {
        // Ensure our term is as simple as possible
        // self.normalize_term(term);
        println!("{}", Printer::marker_unification(marker, term));

        // If a substitution is already available for this marker, we will unify the term with that
        if let Some(sub) = &mut self.substitutions.get_mut(marker).cloned() {
            // If the rhs is a trait, apply it to the left
            if let Term::Trait(trt) = term {
                if !self.apply_trait(sub, trt)? {
                    return Ok(());
                }
            } else {
                // Otherwise, process the terms and get out of here
                return self.unify_terms(sub, term);
            }
        }

        // Check for occurance -- if there is any, then we won't register this
        if !self.occurs(marker, term) {
            self.new_substitution(*marker, term.clone())?;
        }

        Ok(())
    }

    fn unify_terms(&mut self, lhs: &mut Term, rhs: &mut Term) -> Result<(), TypeError> {
        // If these terms are equal, there's nothing to do
        if lhs == rhs {
            return Ok(());
        }

        // Normalize all inputs
        // self.normalize_term(lhs);
        // self.normalize_term(rhs);

        println!("{}", Printer::term_unification(lhs, rhs));

        // If the lhs is a marker, unify it to the right
        if let Term::Marker(marker) = lhs {
            return self.unify_marker(marker, rhs);
        }

        // If the rhs is a marker, unify it to the left
        if let Term::Marker(marker) = rhs {
            return self.unify_marker(marker, lhs);
        }

        // If the lhs is an app, we might be able to unify its interior
        if let Term::App(lhs_app) = lhs {
            match lhs_app {
                App::Array(lhs_member_type) => {
                    if let Term::App(App::Array(rhs_member_type)) = rhs {
                        self.unify_terms(lhs_member_type, rhs_member_type)?;
                    }
                }
                App::Object(_) => {}
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
                            self.unify_terms(&mut lhs_params[i], param)?;
                        }
                        self.unify_terms(lhs_return, rhs_return)?;
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

    fn apply_trait(&mut self, term: &mut Term, trt: &mut Trait) -> Result<bool, TypeError> {
        println!("{}", Printer::term_impl(term, trt));
        match trt {
            Trait::FieldOp(field_name, field_op) => {
                if let Term::App(App::Object(fields)) = term {
                    // If the field is already present, it must match, but otherwise we can inject it
                    if let Some(object_field_op) = fields.get_mut(field_name) {
                        self.unify_terms(object_field_op.term_mut(), field_op.term_mut())?;
                        return Ok(false);
                    } else {
                        fields.insert(field_name.clone(), *field_op.clone());
                        return Ok(true);
                    }
                }
            }
            Trait::Callable {
                calling_scope,
                arguments,
                expected_return,
                uses_new,
            } => {
                if let Term::App(App::Function {
                    self_fields,
                    parameters,
                    return_type: func_return,
                }) = term
                {
                    // Create a temporary writer to unify this call with the function. We do this so we can
                    // learn more information about the call without applying changes to the function itself,
                    // since the function should remain generic.
                    println!("--- Starting a call... ---");
                    let mut temp_writer = self.clone();

                    // Figure out the arguments
                    for (i, arg) in arguments.iter_mut().enumerate() {
                        temp_writer.unify_terms(arg, &mut parameters[i])?
                    }

                    // Figure out the return
                    temp_writer.unify_terms(expected_return, func_return)?;

                    // Now apply everything the temp writer knows onto our arguments
                    for arg in arguments.iter_mut() {
                        let mut new_arg = arg.clone();
                        temp_writer.normalize_term(&mut new_arg);
                        self.unify_terms(arg, &mut new_arg)?;
                    }

                    // Apply the self fields of the function to the current scope.
                    if let Some(self_fields) = self_fields {
                        for (name, op) in self_fields.iter() {
                            self.scope.self_fields.insert(name.clone(), op.clone());
                        }
                    }
                    for (name, op) in temp_writer.scope.self_fields.iter() {
                        self.scope.self_fields.insert(name.clone(), op.clone());
                    }

                    // If using new, we want to define the properties of the constructor itself
                    if *uses_new {
                        if let Some(self_fields) = self_fields {
                            self_fields.extend(&mut self.scope.self_fields.clone().into_iter());
                        } else {
                            *self_fields = Some(self.scope.self_fields.clone());
                        }
                        self.unify_terms(
                            expected_return,
                            &mut Term::App(App::Object(self_fields.clone().unwrap_or_default())),
                        )?;
                        return Ok(true);
                    } else {
                        // Otherwise its based on what the temp writer knows
                        let mut new_return_type = expected_return.clone();
                        temp_writer.normalize_term(&mut new_return_type);
                        self.unify_terms(expected_return, &mut new_return_type)?;
                    }
                    println!("the scope of {} is {:?}", Printer::term(term), self.scope.self_fields);
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
                App::Object(fields) => fields.iter().any(|(_, field)| self.occurs(marker, field.term())),
                App::Function {
                    self_fields: self_parameter,
                    parameters,
                    return_type,
                    ..
                } => {
                    self_parameter.as_ref().map_or(false, |fields| {
                        fields.iter().any(|(_, op)| self.occurs(marker, op.term()))
                    }) || self.occurs(marker, return_type)
                        || parameters.iter().any(|param| self.occurs(marker, param))
                }
            },
            Term::Trait(trt) => match trt {
                Trait::FieldOp(_, op) => self.occurs(marker, op.term()),
                Trait::Callable {
                    calling_scope,
                    arguments,
                    expected_return,
                    ..
                } => {
                    return calling_scope.iter().any(|(_, op)| self.occurs(marker, op.term()))
                        || self.occurs(marker, expected_return)
                        || arguments.iter().any(|arg| self.occurs(marker, arg));
                }
            },
        }
    }

    fn normalize_term(&self, term: &mut Term) {
        match term {
            Term::Type(_) => {}
            Term::Marker(marker) => {
                if let Some(sub) = self.substitutions.get(marker) {
                    *term = sub.clone();
                }
            }
            Term::App(app) => match app {
                App::Array(member_term) => self.normalize_term(member_term),
                App::Object(fields) => fields.iter_mut().for_each(|(_, op)| self.normalize_term(op.term_mut())),
                App::Function {
                    self_fields,
                    parameters,
                    return_type,
                    ..
                } => {
                    if let Some(self_fields) = self_fields.as_mut() {
                        self_fields
                            .iter_mut()
                            .for_each(|(_, op)| self.normalize_term(op.term_mut()))
                    }
                    self.normalize_term(return_type);
                    parameters.iter_mut().for_each(|param| self.normalize_term(param));
                }
            },
            Term::Trait(trt) => self.normalize_trait(trt),
        }
    }

    fn normalize_trait(&self, trt: &mut Trait) {
        match trt {
            Trait::FieldOp(_, op) => self.normalize_term(op.term_mut()),
            Trait::Callable {
                calling_scope,
                arguments,
                expected_return,
                ..
            } => {
                arguments.iter_mut().for_each(|arg| self.normalize_term(arg));
                calling_scope
                    .iter_mut()
                    .for_each(|(_, op)| self.normalize_term(op.term_mut()));
                self.normalize_term(expected_return);
            }
        }
    }

    /// ### Errors
    /// shut up
    pub fn new_substitution(&mut self, marker: Marker, mut term: Term) -> Result<(), TypeError> {
        self.normalize_term(&mut term);
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
            self.normalize_term(&mut term);
            self.substitutions.insert(marker, term);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct TypeError(Type, Type);
