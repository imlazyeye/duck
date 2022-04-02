use super::*;
use crate::{parse::Stmt, FileId};
use codespan_reporting::diagnostic::Diagnostic;
use hashbrown::HashMap;
use itertools::Itertools;

#[derive(Debug, PartialEq, Clone)]
pub struct Typewriter {
    pub substitutions: HashMap<Marker, Term>,
    pub constraints: Vec<Constraint>,
}

// General
impl Typewriter {
    pub fn new() -> Self {
        Self {
            substitutions: HashMap::default(),
            constraints: Vec::default(),
        }
    }

    /// ### Errors
    /// shut up
    pub fn write(&mut self, stmts: &[Stmt], scope: &mut Scope) -> Result<(), Vec<TypeError>> {
        let constraints = match Constraints::build(self, scope, stmts) {
            Ok(constraints) => constraints,
            Err(e) => return Err(e),
        };

        match self.apply_constraints(constraints, scope) {
            Ok(_) => {}
            Err(e) => return Err(vec![e]),
        }

        println!("\nCurrent substitutions:");
        self.substitutions
            .iter()
            .for_each(|(marker, term)| println!("{}", Printer::substitution(marker, term)));
        println!("Local fields in scope: {:?}", scope.local_fields());
        println!(
            "Fields in self: [{}]",
            self.find_term(&scope.self_marker)
                .unwrap()
                .as_object()
                .unwrap()
                .fields()
                .iter()
                .map(|(name, term)| format!("{name}: {}", Printer::term(term)))
                .join(", ")
        );

        Ok(())
    }

    pub fn take_return_term(&mut self) -> Term {
        if let Some(term) = self.substitutions.remove(&Marker::RETURN) {
            term
        } else {
            Term::Type(Type::Undefined)
        }
    }

    pub fn find_term(&self, marker: &Marker) -> Option<&Term> {
        self.substitutions.get(marker)
    }

    pub fn find_term_mut(&mut self, marker: &Marker) -> Option<&mut Term> {
        self.substitutions.get_mut(marker)
    }
}

impl Default for Typewriter {
    fn default() -> Self {
        Self::new()
    }
}

// Unification
impl Typewriter {
    /// ### Errors
    /// shut up
    pub fn apply_constraints(&mut self, mut constraints: Vec<Constraint>, scope: &mut Scope) -> Result<(), TypeError> {
        while let Some(mut pattern) = constraints.pop() {
            match &mut pattern {
                Constraint::Eq(marker, term) => self.unify_marker(marker, term, scope)?,
            }
        }
        Ok(())
    }

    pub(super) fn unify_marker(
        &mut self,
        marker: &Marker,
        term: &mut Term,
        scope: &mut Scope,
    ) -> Result<(), TypeError> {
        // Ensure our term is as simple as possible
        println!("{}", Printer::marker_unification(marker, term));

        // If a substitution is already available for this marker, we will unify the term with that
        if let Some(mut sub) = self.substitutions.get_mut(marker).cloned() {
            // If the rhs is a trait, apply it to the left
            if let Term::Trait(trt) = term {
                self.apply_trait(&mut sub, trt, scope)?;
            } else {
                // Otherwise, process the terms
                self.normalize_term(term);
                self.unify_terms(&mut sub, term, scope)?;
            }
            *term = sub;
        }

        // Check for occurance -- if there is any, then we won't register this
        if !self.occurs(marker, term) {
            self.new_substitution(*marker, term.clone())?;
        }

        Ok(())
    }

    pub(super) fn unify_terms(&mut self, lhs: &mut Term, rhs: &mut Term, scope: &mut Scope) -> Result<(), TypeError> {
        // If these terms are equal, there's nothing to do
        if lhs == rhs {
            return Ok(());
        }

        println!("{}", Printer::term_unification(lhs, rhs));

        // If the lhs is a marker, unify it to the right
        if let Term::Marker(marker) = lhs {
            return self.unify_marker(marker, rhs, scope);
        }

        // If the rhs is a marker, unify it to the left
        if let Term::Marker(marker) = rhs {
            return self.unify_marker(marker, lhs, scope);
        }

        // Apps?
        match lhs {
            Term::App(App::Array(lhs_member_type)) => {
                if let Term::App(App::Array(rhs_member_type)) = rhs {
                    self.unify_terms(lhs_member_type, rhs_member_type, scope)?;
                } else {
                    return Err(Diagnostic::error().with_message(format!("{} is not an array", Printer::term(rhs))));
                }
            }
            Term::App(App::Object(object)) => {
                match rhs {
                    Term::App(App::Object(other_object)) => {
                        object.apply_object(other_object, self, scope)?;
                    }
                    Term::Trait(Trait::FieldOp(_, _)) => {
                        // hmm this could replace the trait thing, yknow?
                    }
                    _ => {
                        return Err(Diagnostic::error()
                            .with_message(format!("{} is does not contain fields", Printer::term(rhs))));
                    }
                }
            }
            Term::App(App::Union(lhs_types)) => match rhs {
                Term::App(App::Union(rhs_types)) => {
                    for (i, tpe) in lhs_types.iter_mut().enumerate() {
                        self.unify_terms(tpe, rhs_types.get_mut(i).expect("foo"), scope)?
                    }
                }
                _ => {
                    for tpe in lhs_types.iter_mut() {
                        self.unify_terms(tpe, rhs, scope)?
                    }
                }
            },
            _ => (),
        }

        // Trait?
        if let Term::Trait(trt) = lhs {
            if let Trait::Callable { .. } = trt {
                if let Term::App(App::Function { .. }) = rhs {
                    // todo: this just doesn't feel right why is this here
                    self.apply_trait(rhs, trt, scope)?;
                    *lhs = rhs.clone();
                }
            }
        }

        // Are these clashing types?
        if let Term::Type(lhs_type) = lhs {
            if let Term::Type(rhs_type) = rhs {
                if lhs_type != rhs_type {
                    return Err(Diagnostic::error().with_message(format!(
                        "Attempted to equate two incompatible types: {} and {}",
                        Printer::tpe(lhs_type),
                        Printer::tpe(rhs_type)
                    )));
                }
            } else {
                // Flip it around -- we technically could just panic if lhs != rhs in general, but this will let us
                // get better errors
                self.unify_terms(rhs, lhs, scope)?;
            }
        }

        Ok(())
    }

    fn apply_trait(&mut self, term: &mut Term, trt: &mut Trait, scope: &mut Scope) -> Result<(), TypeError> {
        println!("{}", Printer::term_impl(term, trt));
        match trt {
            Trait::FieldOp(field_name, field_op) => {
                if let Term::App(App::Object(object)) = term {
                    object.apply_field_op(field_name, field_op, self, scope)?;
                } else {
                    return Err(
                        Diagnostic::error().with_message(format!("{} is does not contain fields", Printer::term(term)))
                    );
                }
            }
            Trait::Callable {
                arguments,
                expected_return,
                uses_new,
            } => {
                if let Term::App(App::Function(function)) = term {
                    self.call_function(function, arguments, expected_return, *uses_new, scope)?;
                } else {
                    return Err(
                        Diagnostic::error().with_message(format!("{} is not a valid call target", Printer::term(term)))
                    );
                }
            }
        };
        Ok(())
    }

    fn call_function(
        &mut self,
        function: &mut super::Function,
        arguments: &mut Vec<Term>,
        expected_return: &mut Box<Term>,
        uses_new: bool,
        scope: &mut Scope,
    ) -> Result<(), TypeError> {
        // Create a temporary writer to unify this call with the function. We do this so we can
        // learn more information about the call without applying changes to the function itself,
        // since the function should remain generic.
        println!(
            "--- Calling a {}... ---",
            if uses_new { "constructor" } else { "function " }
        );
        let mut call_writer = self.clone();

        // If we're using new, we treat this as a constructor, which will change behavior
        let mut call_scope = if uses_new {
            // The constructor gets its own new scope, so we do not need to pass it our
            // self terms. It just uses whatever the function itself calls for.
            let obj = function
                .self_fields
                .clone()
                .unwrap_or_else(|| Object::Inferred(HashMap::default()));

            Scope::new(&mut call_writer, obj)
        } else {
            // If the function uses self parameters, we need to create a new scope for it that knows about
            // our fields
            let obj = if let Some(mut func_self) = function.self_fields.as_ref().cloned() {
                func_self.apply_object(
                    self.find_term_mut(&scope.self_marker)
                        .and_then(|term| term.as_object_mut())
                        .unwrap(),
                    &mut call_writer,
                    scope,
                )?;
                func_self
            } else {
                // Otherwise, it has no scope
                Object::Inferred(HashMap::default())
            };

            Scope::new(&mut call_writer, obj)
        };

        // If we have inheritance, we need to call that function in this scope
        if let Some(inheritance) = &mut function.inheritance {
            let mut our_term = scope.lookup_term(inheritance, self)?;
            if let Term::App(App::Function(function)) = &mut our_term {
                call_writer.call_function(
                    function,
                    arguments,
                    &mut expected_return.clone(),
                    false,
                    &mut call_scope,
                )?;
                println!("sdasd");
            }
        }

        // Figure out the arguments
        println!("Unifying arguments.");
        for (i, arg) in arguments.iter_mut().enumerate() {
            call_writer.unify_terms(arg, &mut function.parameters[i].clone(), &mut call_scope)?
        }

        // Figure out the return
        println!("Unifying the return.");
        call_writer.unify_terms(expected_return, &mut function.return_type, &mut call_scope)?;

        println!("--- Extracting information from call... ---");

        // Now apply everything the temp writer knows onto our arguments
        println!("Reverse unifying the arguments.");
        for arg in arguments.iter_mut() {
            let mut new_arg = arg.clone();
            call_writer.normalize_term(&mut new_arg);
            self.unify_terms(arg, &mut new_arg, scope)?;
        }

        // If we just made a constructor, we need to adjust our return value to be a struct made of its
        // scope
        let self_marker = scope.self_marker;
        if uses_new {
            println!(
                "Crafting the constructed return from {}.",
                Printer::marker(&call_scope.self_marker)
            );
            let call_scope_object = call_writer.find_term_mut(&call_scope.self_marker).unwrap();
            self.unify_terms(expected_return, call_scope_object, scope)?;
        } else {
            println!("Modifying the scope.");
            // Otherwise, we need to apply any modifications this function made to our scope
            let call_scope_object = call_writer.find_term_mut(&call_scope.self_marker).unwrap();
            self.unify_marker(&self_marker, call_scope_object, scope)?;
            let temp_self_fields = call_writer.find_term_mut(&scope.self_marker).unwrap();
            self.unify_marker(&self_marker, temp_self_fields, scope)?;

            // Once that's done, our return value is just the normalized value that we
            // can find from the call writer
            println!("Normalizing the return.");
            let mut new_return_type = expected_return.clone();
            call_writer.normalize_term(&mut new_return_type);
            self.unify_terms(expected_return, &mut new_return_type, scope)?;
        }
        println!("--- Call complete. ---");

        Ok(())
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
                App::Object(object) => match object {
                    Object::Concrete(fields) => fields.iter().any(|(_, field)| self.occurs(marker, field)),
                    Object::Inferred(fields) => fields.iter().any(|(_, field)| self.occurs(marker, field.term())),
                },
                App::Function(super::Function {
                    self_fields,
                    parameters,
                    return_type,
                    ..
                }) => {
                    self_fields.as_ref().map_or(false, |object| match object {
                        Object::Concrete(fields) => fields.iter().any(|(_, field)| self.occurs(marker, field)),
                        Object::Inferred(fields) => fields.iter().any(|(_, field)| self.occurs(marker, field.term())),
                    }) || self.occurs(marker, return_type)
                        || parameters.iter().any(|param| self.occurs(marker, param))
                }
                App::Union(terms) => terms.iter().any(|v| self.occurs(marker, v)),
            },
            Term::Trait(trt) => match trt {
                Trait::FieldOp(_, op) => self.occurs(marker, op.term()),
                Trait::Callable {
                    arguments,
                    expected_return,
                    ..
                } => {
                    return self.occurs(marker, expected_return)
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
                App::Object(object) => match object {
                    Object::Concrete(fields) => fields.iter_mut().for_each(|(_, op)| self.normalize_term(op)),
                    Object::Inferred(fields) => {
                        fields.iter_mut().for_each(|(_, op)| self.normalize_term(op.term_mut()))
                    }
                },
                App::Function(super::Function {
                    self_fields,
                    parameters,
                    return_type,
                    ..
                }) => {
                    if let Some(object) = self_fields.as_mut() {
                        match object {
                            Object::Concrete(fields) => fields.iter_mut().for_each(|(_, op)| self.normalize_term(op)),
                            Object::Inferred(fields) => {
                                fields.iter_mut().for_each(|(_, op)| self.normalize_term(op.term_mut()))
                            }
                        }
                    }
                    self.normalize_term(return_type);
                    parameters.iter_mut().for_each(|param| self.normalize_term(param));
                }
                App::Union(terms) => terms.iter_mut().for_each(|v| self.normalize_term(v)),
            },
            Term::Trait(trt) => match trt {
                Trait::FieldOp(_, op) => self.normalize_term(op.term_mut()),
                Trait::Callable {
                    arguments,
                    expected_return,
                    ..
                } => {
                    arguments.iter_mut().for_each(|arg| self.normalize_term(arg));
                    self.normalize_term(expected_return);
                }
            },
        }
    }

    /// ### Errors
    /// shut up
    pub fn new_substitution(&mut self, marker: Marker, mut term: Term) -> Result<(), TypeError> {
        self.normalize_term(&mut term);
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
            self.normalize_term(&mut term);
            self.substitutions.insert(marker, term);
        }
        Ok(())
    }
}

pub type TypeError = Diagnostic<FileId>;
