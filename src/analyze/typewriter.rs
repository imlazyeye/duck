use super::*;
use crate::{
    parse::{Identifier, Stmt},
    FileId,
};
use codespan_reporting::diagnostic::Diagnostic;
use hashbrown::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Typewriter {
    pub scope: Scope,
    pub substitutions: HashMap<Marker, Term>,
    pub collection: Vec<Constraint>,
}

// General
impl Typewriter {
    pub fn new(scope: Scope) -> Self {
        Self {
            scope,
            substitutions: HashMap::default(),
            collection: Vec::default(),
        }
    }

    pub fn write(&mut self, stmts: &[Stmt]) {
        println!("\n--- Start TypeWriter::write... ---\n");
        let constraints = Constraints::build(&mut self.scope, stmts);
        self.apply_constraints(constraints);
        println!("\nFinal substitutions:");
        self.substitutions
            .iter()
            .for_each(|(marker, term)| println!("{}", Printer::substitution(marker, term)));
        println!("\n--- Ending TypeWriter::write... ---\n");
    }

    /// ### Errors
    /// Returns an error if the field is not in scope.
    pub fn read_field(&self, identifier: &Identifier) -> Result<Type, Diagnostic<FileId>> {
        self.scope
            .field_marker(identifier)
            .map(|marker| self.marker_to_type(marker))
    }

    pub fn return_term(&self) -> Term {
        let tpe = self.marker_to_term(Marker::RETURN_VALUE);
        if let Term::Marker(Marker::RETURN_VALUE) = tpe {
            Term::Type(Type::Undefined)
        } else {
            tpe
        }
    }

    pub fn marker_to_type(&self, marker: Marker) -> Type {
        self.marker_to_term(marker).into()
    }

    pub fn marker_to_term(&self, marker: Marker) -> Term {
        self.substitutions.get(&marker).cloned().unwrap_or(Term::Marker(marker))
    }
}

// Unification
impl Typewriter {
    pub fn apply_constraints(&mut self, mut constraints: Vec<Constraint>) {
        while let Some(mut pattern) = constraints.pop() {
            let result = match &mut pattern {
                Constraint::Eq(marker, term) => self.unify_marker(marker, term),
                Constraint::Impl(marker, imp) => self.apply_impl(marker, imp),
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

    fn unify_marker(&mut self, marker: &Marker, term: &mut Term) -> Result<(), TypeError> {
        // Ensure our term is as simple as possible
        self.normalize(term);

        println!("{}", Printer::marker_unification(marker, term));

        // If there is an impl, we should apply it
        if let Term::Impl(imp) = term {
            return self.apply_impl(marker, imp);
        }

        // If a substitution is already available for this marker, we will unify the term with that
        if let Some(sub) = &mut self.substitutions.get_mut(marker).cloned() {
            return self.unify_terms(sub, term);
        }

        // If the term is a deref, we might be able to translate it
        if let Term::Deref(deref) = term {
            match deref {
                Deref::Call { target, arguments } => {
                    if let Term::App(App::Function(_, _, function)) = target.as_mut() {
                        let mut new_writer = Typewriter::new(self.scope.clone());
                        for (i, arg) in arguments.iter().enumerate() {
                            let param = &function.parameters[i];
                            new_writer.scope.new_field(param.name(), param.name_expr());
                            let param_marker = new_writer.scope.get_expr_marker(param.name_expr());
                            new_writer.unify_marker(&param_marker, &mut arg.clone())?;
                        }
                        let (_, mut return_type) = App::process_function(function.clone(), &mut new_writer);
                        return self.unify_marker(marker, &mut return_type);
                    }
                }
                Deref::MemberType { target } => match target.as_mut() {
                    Term::App(App::Array(member_type)) => {
                        return self.unify_marker(marker, member_type.as_mut());
                    }
                    Term::Type(Type::Array { member_type }) => {
                        return self.unify_marker(marker, &mut Term::Type(member_type.as_ref().clone()));
                    }
                    Term::Marker(_) => {}
                    _ => panic!("invalid array deref target"),
                },
                Deref::Field { target, field_name } => match target.as_mut() {
                    Term::App(App::Object(fields)) => {
                        return self.unify_marker(marker, fields.get_mut(field_name).expect("doh"));
                    }
                    Term::Impl(Impl::Fields(ops)) => {
                        let term = ops.get_mut(field_name).expect("rats");
                        return self.unify_marker(marker, term);
                    }
                    Term::Marker(_) => {}
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

    fn unify_terms(&mut self, lhs: &mut Term, rhs: &mut Term) -> Result<(), TypeError> {
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
            return self.unify_marker(marker, rhs);
        }

        // If the rhs is a marker, unify it to the left
        if let Term::Marker(marker) = rhs {
            return self.unify_marker(marker, lhs);
        }

        // Are these equivelent apps?
        if let Term::App(lhs_app) = lhs {
            match lhs_app {
                App::Array(lhs_member_type) => {
                    if let Term::App(App::Array(rhs_member_type)) = rhs {
                        self.unify_terms(lhs_member_type, rhs_member_type)?;
                    }
                }
                App::Object(lhs_fields) => {
                    if let Term::App(App::Object(rhs_fields)) = rhs {
                        for (name, field) in lhs_fields {
                            self.unify_terms(field, rhs_fields.get_mut(name).expect("eh"))?;
                        }
                    }
                }
                App::Function(lhs_parameters, lhs_return_type, _) => {
                    if let Term::App(App::Function(rhs_parameters, rhs_return_type, _)) = rhs {
                        for (i, param) in rhs_parameters.iter_mut().enumerate() {
                            self.unify_terms(&mut lhs_parameters[i], param)?;
                        }
                        self.unify_terms(lhs_return_type, rhs_return_type)?;
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

    fn apply_impl(&mut self, marker: &Marker, imp: &mut Impl) -> Result<(), TypeError> {
        if let Some(sub) = &mut self.substitutions.get_mut(marker).cloned() {
            match imp {
                Impl::Fields(imp_fields) => match sub {
                    Term::Impl(Impl::Fields(fields)) => {
                        for (name, imp_field) in imp_fields {
                            if let Some(field) = fields.get_mut(name) {
                                self.unify_terms(field, imp_field)?;
                            } else {
                                fields.insert(name.into(), imp_field.clone());
                            }
                        }
                        self.new_substitution(*marker, Term::Impl(imp.clone()))
                    }
                    Term::App(App::Object(fields)) => {
                        for (name, term) in imp_fields.iter_mut() {
                            self.unify_terms(fields.get_mut(name).expect("missing field being read"), term)?;
                        }
                        Ok(())
                    }
                    // maybe deref?
                    _ => Ok(()),
                },
            }
        } else {
            self.new_substitution(*marker, Term::Impl(imp.clone()))
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
                App::Function(params, return_type, _) => {
                    self.occurs(marker, return_type) || params.iter().any(|param| self.occurs(marker, param))
                }
            };
        }

        // If the term is a deref, it might be dereffing our marker
        if let Term::Deref(deref) = term {
            return match deref {
                Deref::Call { target, arguments } => {
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
                App::Function(arguments, return_type, _) => {
                    self.normalize(return_type);
                    arguments.iter_mut().for_each(|arg| self.normalize(arg));
                }
            },
            Term::Deref(deref) => match deref {
                Deref::Field { target, .. } => self.normalize(target),
                Deref::MemberType { target } => self.normalize(target),
                Deref::Call { target, arguments } => {
                    self.normalize(target);
                    arguments.iter_mut().for_each(|arg| self.normalize(arg));
                }
            },
            Term::Impl(imp) => match imp {
                Impl::Fields(fields) => fields.iter_mut().for_each(|(_, term)| self.normalize(term)),
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
