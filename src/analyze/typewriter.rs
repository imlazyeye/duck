use super::*;
use crate::{
    duck_error,
    parse::{Expr, ExprId, Stmt},
    FileId,
};
use codespan_reporting::diagnostic::Diagnostic;
use hashbrown::HashMap;

#[derive(Debug, Default, Clone)]
pub struct Typewriter {
    pub substitutions: HashMap<Marker, Term>,
    pub active_self: Record,
    pub locals: Record,
    pub markers: HashMap<ExprId, Marker>,
}

// General
impl Typewriter {
    pub fn write(&mut self, stmts: &[Stmt]) -> Result<(), Vec<TypeError>> {
        let mut constraints = Constraints::build(self, stmts)?;
        while let Some(Constraint::Eq(marker, mut term)) = constraints.pop() {
            self.unify_marker(&marker, &mut term).map_err(|v| vec![v])?;
        }

        println!("\nCurrent substitutions:");
        self.substitutions
            .iter()
            .for_each(|(marker, term)| println!("{}", Printer::substitution(marker, term, self)));
        // println!("Local fields in scope: {:?}", scope.local_fields());
        // println!(
        //     "Fields in self: [{}]",
        //     self.find_term(&scope.self_marker)
        //         .unwrap()
        //         .as_object()
        //         .unwrap()
        //         .fields()
        //         .iter()
        //         .map(|(name, term)| format!("{name}: {}", Printer::term(term)))
        //         .join(", ")
        // );

        Ok(())
    }

    pub fn take_return_term(&mut self) -> Term {
        if let Some(term) = self.substitutions.remove(&Marker::RETURN) {
            term
        } else {
            Term::Type(Type::Undefined)
        }
    }

    pub fn lookup(&self, name: &str) -> Option<Type> {
        let marker = if let Some(field) = self.locals.get(name) {
            field.marker
        } else if let Some(field) = self.active_self.get(name) {
            field.marker
        } else {
            return None;
        };
        let term = self.lookup_normalized_term(&marker);
        Some(self.term_to_type(term))
    }

    pub fn lookup_normalized_term(&self, marker: &Marker) -> Term {
        let mut term = self.substitutions.get(marker).cloned().unwrap();
        self.normalize_term(&mut term);
        term
    }

    pub fn term_to_type(&self, term: Term) -> Type {
        match term {
            Term::Type(tpe) => tpe,
            Term::Marker(marker) => Type::Generic {
                term: Box::new(Term::Marker(marker)),
            },
            Term::App(app) => match app {
                App::Array(member_type) => Type::Array {
                    member_type: Box::new(self.term_to_type(member_type.as_ref().to_owned())),
                },
                App::Record(record) => Type::Struct {
                    fields: record
                        .fields
                        .into_iter()
                        .map(|(name, field)| {
                            let term = self.lookup_normalized_term(&field.marker);
                            (name, self.term_to_type(term))
                        })
                        .collect(),
                },
            },
        }
    }

    pub fn marker_for(&mut self, expr: &Expr) -> Result<Marker, TypeError> {
        if let Some(marker) = self.markers.get(&expr.id()) {
            Ok(*marker)
        } else if let Some(iden) = expr.inner().as_identifier() {
            if let Some(field) = self.locals.get(&iden.lexeme) {
                Ok(field.marker)
            } else if let Some(field) = self.active_self.get(&iden.lexeme) {
                Ok(field.marker)
            } else {
                duck_error!("Unrecognized variable: {iden}")
            }
        } else {
            Ok(self.new_marker(expr))
        }
    }

    pub fn write_self(&mut self, name: &str, expr: &Expr, value: Marker) -> Result<Marker, TypeError> {
        let marker = self
            .active_self
            .write_field(name, expr.id(), expr.location(), value)?
            .apply(self)?;
        Printer::give_expr_alias(marker, expr.to_string());
        Ok(marker)
    }

    pub fn write_local(&mut self, name: &str, expr: &Expr, value: Marker) -> Result<Marker, TypeError> {
        let marker = self
            .locals
            .write_field(name, expr.id(), expr.location(), value)?
            .apply(self)?;
        Printer::give_expr_alias(marker, expr.to_string());
        Ok(marker)
    }

    pub fn new_marker(&mut self, expr: &Expr) -> Marker {
        let marker = Marker::new();
        Printer::give_expr_alias(marker, expr.to_string());
        self.markers.insert(expr.id(), marker);
        marker
    }
}

// Unification
impl Typewriter {
    pub fn unify_marker(&mut self, marker: &Marker, term: &mut Term) -> Result<(), TypeError> {
        println!("{}", Printer::marker_unification(marker, term, self));

        if let Some(mut sub) = self.substitutions.get_mut(marker).cloned() {
            self.normalize_term(term);
            self.unify_terms(&mut sub, term)?;
            *term = sub;
        }

        if !self.occurs(marker, term) {
            self.new_substitution(*marker, term.clone())
        } else {
            Ok(())
        }
    }

    pub fn unify_terms(&mut self, lhs: &mut Term, rhs: &mut Term) -> Result<(), TypeError> {
        if lhs == rhs {
            return Ok(());
        }

        // Early out of the rhs is a marker
        if let Term::Marker(marker) = rhs {
            return self.unify_marker(marker, lhs);
        }

        println!("{}", Printer::term_unification(lhs, rhs, self));

        // Unify the terms
        match lhs {
            Term::Marker(marker) => self.unify_marker(marker, rhs),
            Term::App(lhs_app) => match lhs_app {
                App::Array(lhs_member) => match rhs {
                    Term::App(App::Array(rhs_member)) => self.unify_terms(lhs_member, rhs_member),
                    _ => {
                        duck_error!("{} is not an array", Printer::term(rhs, self))
                    }
                },
                App::Record(record) => match rhs {
                    Term::App(App::Record(other_record)) => {
                        // Apply the other record to this record
                        for (name, rhs_field) in other_record.fields.iter() {
                            if let Some(lhs_field) = record.get(name) {
                                self.unify_marker(&lhs_field.marker, &mut Term::Marker(rhs_field.marker))?;
                            } else if other_record.is_writer {
                                record
                                    .write_field(name, rhs_field.expr_id, rhs_field.location, rhs_field.marker)?
                                    .apply(self)?;
                            } else {
                                return duck_error!("Missing field: {name}");
                            }
                        }
                        Ok(())
                    }
                    _ => {
                        duck_error!("{} is does not contain fields", Printer::term(rhs, self))
                    }
                },
            },
            Term::Type(_) => {
                if lhs != rhs {
                    duck_error!(
                        "Attempted to equate two incompatible terms: {lhs:?} and {rhs:?}",
                        // Printer::term(lhs),
                        // Printer::term(rhs)
                    )
                } else {
                    Ok(())
                }
            }
        }
    }

    fn occurs(&self, marker: &Marker, term: &Term) -> bool {
        match term {
            Term::Type(_) => false,
            Term::Marker(term_marker) => {
                term_marker == marker
                    || self
                        .substitutions
                        .get(term_marker)
                        .map_or(false, |term| self.occurs(marker, term))
            }
            Term::App(term_app) => match term_app {
                App::Array(member_term) => self.occurs(marker, member_term),
                App::Record(record) => record.fields.iter().any(|(_, field)| {
                    let term = self.lookup_normalized_term(&field.marker);
                    self.occurs(marker, &term)
                }),
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
                App::Record(_) => {}
            },
        }
    }

    /// ### Errors
    /// shut up
    pub fn new_substitution(&mut self, marker: Marker, mut term: Term) -> Result<(), TypeError> {
        self.normalize_term(&mut term);
        println!("{}", Printer::substitution(&marker, &term, self));
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

// fn call_function(
//     &mut self,
//     function: &mut super::Function,
//     arguments: &mut Vec<Term>,
//     expected_return: &mut Box<Term>,
//     uses_new: bool,
//     scope: &mut Scope,
// ) -> Result<(), TypeError> {
//     // Create a temporary writer to unify this call with the function. We do this so we can
//     // learn more information about the call without applying changes to the function itself,
//     // since the function should remain generic.
//     println!(
//         "--- Calling a {}... ---",
//         if uses_new { "constructor" } else { "function " }
//     );
//     let mut call_writer = self.clone();

//     // If we're using new, we treat this as a constructor, which will change behavior
//     let mut call_scope = if uses_new {
//         // The constructor gets its own new scope, so we do not need to pass it our
//         // self terms. It just uses whatever the function itself calls for.
//         // let obj = function
//         //     .self_fields
//         //     .clone()
//         //     .unwrap_or_else(|| Object::Inferred(HashMap::default()));

//         Scope::new_concrete(self)
//     } else {
//         // If the function uses self parameters, we need to create a new scope for it that knows
// about         // our fields
//         let obj = if let Some(binding) = function.binding {
//             self.find_term_mut(&binding)
//                 .and_then(|term| term.as_object_mut())
//                 .unwrap()
//                 .clone()
//         } else {
//             Object::Inferred(HashMap::default())
//         };

//         Scope::new(&mut call_writer, obj)
//     };

//     // If we have inheritance, we need to call that function in this scope
//     if let Some(inheritance) = &mut function.inheritance {
//         let mut our_term = scope.lookup_term(inheritance, self)?;
//         if let Term::App(App::Function(function)) = &mut our_term {
//             call_writer.call_function(
//                 function,
//                 arguments,
//                 &mut expected_return.clone(),
//                 false,
//                 &mut call_scope,
//             )?;
//             println!("sdasd");
//         }
//     }

//     // Figure out the arguments
//     println!("Unifying arguments.");
//     for (i, arg) in arguments.iter_mut().enumerate() {
//         call_writer.unify_terms(arg, &mut function.parameters[i].clone(), &mut call_scope)?
//     }

//     // Figure out the return
//     println!("Unifying the return.");
//     call_writer.unify_terms(expected_return, &mut function.return_type, &mut call_scope)?;

//     println!("--- Extracting information from call... ---");

//     // Now apply everything the temp writer knows onto our arguments
//     println!("Reverse unifying the arguments.");
//     for arg in arguments.iter_mut() {
//         let mut new_arg = arg.clone();
//         call_writer.normalize_term(&mut new_arg);
//         self.unify_terms(arg, &mut new_arg, scope)?;
//     }

//     // If we just made a constructor, we need to adjust our return value to be a struct made of
// its     // scope
//     let self_marker = scope.self_marker;
//     if uses_new {
//         println!(
//             "Crafting the constructed return from {}.",
//             Printer::marker(&call_scope.self_marker)
//         );
//         let call_scope_object = call_writer.find_term_mut(&call_scope.self_marker).unwrap();
//         self.unify_terms(expected_return, call_scope_object, scope)?;
//     } else {
//         println!("Modifying the scope.");
//         // Otherwise, we need to apply any modifications this function made to our scope
//         let call_scope_object = call_writer.find_term_mut(&call_scope.self_marker).unwrap();
//         self.unify_marker(&self_marker, call_scope_object, scope)?;
//         let temp_self_fields = call_writer.find_term_mut(&scope.self_marker).unwrap();
//         self.unify_marker(&self_marker, temp_self_fields, scope)?;

//         // Once that's done, our return value is just the normalized value that we
//         // can find from the call writer
//         println!("Normalizing the return.");
//         let mut new_return_type = expected_return.clone();
//         call_writer.normalize_term(&mut new_return_type);
//         self.unify_terms(expected_return, &mut new_return_type, scope)?;
//     }
//     println!("--- Call complete. ---");

//     Ok(())
// }
