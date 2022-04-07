use super::*;
use crate::{
    duck_error,
    parse::{Expr, ExprId, Stmt},
    FileId,
};
use codespan_reporting::diagnostic::Diagnostic;
use hashbrown::HashMap;

#[derive(Debug, Clone)]
pub struct Typewriter {
    pub substitutions: HashMap<Marker, Term>,
    active_self: Marker,
    locals: Marker,
    pub markers: HashMap<ExprId, Marker>,
}

// General
impl Typewriter {
    pub fn process_statements(&mut self, stmts: &[Stmt]) -> Result<(), Vec<TypeError>> {
        let mut constraints = Constraints::build(self, stmts)?;
        while let Some(Constraint::Eq(marker, mut term)) = constraints.pop() {
            self.unify_marker(&marker, &mut term).map_err(|v| vec![v])?;
        }

        // println!("\nCurrent substitutions:");
        // self.substitutions
        //     .iter()
        //     .for_each(|(marker, term)| println!("{}", Printer::substitution(marker, term, self)));
        Ok(())
    }

    pub fn take_return_term(&mut self) -> Term {
        if let Some(term) = self.substitutions.remove(&Marker::RETURN) {
            term
        } else {
            Term::Type(Type::Undefined)
        }
    }

    pub fn lookup_type(&self, name: &str) -> Result<Type, TypeError> {
        let marker = if let Some(field) = self.locals().get(name) {
            field.marker
        } else if let Some(field) = self.active_self().get(name) {
            field.marker
        } else {
            return duck_error!("Could not resolve a type for `{name}`");
        };
        self.term_to_type(self.substitutions.get(&marker).cloned().unwrap())
    }

    pub fn lookup_normalized_term(&self, marker: &Marker) -> Result<Term, TypeError> {
        let mut term = self.substitutions.get(marker).cloned().unwrap();
        self.normalize_term(&mut term)?;
        Ok(term)
    }

    pub fn term_to_type(&self, mut term: Term) -> Result<Type, TypeError> {
        self.normalize_term(&mut term)?;
        let tpe = match term {
            Term::Type(tpe) => tpe,
            Term::Marker(marker) => Type::Generic {
                term: Box::new(Term::Marker(marker)),
            },
            Term::App(app) => match app {
                App::Array(member_type) => Type::Array {
                    member_type: Box::new(self.term_to_type(member_type.as_ref().to_owned())?),
                },
                App::Record(record) => Type::Struct {
                    fields: record
                        .fields
                        .into_iter()
                        .map(|(name, field)| self.term_to_type(Term::Marker(field.marker)).map(|tpe| (name, tpe)))
                        .collect::<Result<HashMap<String, Type>, TypeError>>()?,
                },
                App::Function(function) => Type::Function {
                    parameters: function
                        .parameters
                        .into_iter()
                        .map(|term| self.term_to_type(term))
                        .collect::<Result<Vec<Type>, TypeError>>()?,
                    return_type: Box::new(self.term_to_type(*function.return_type)?),
                },
                App::Call(call) => Type::Generic {
                    term: Box::new(Term::App(App::Call(call))),
                },
            },
        };

        Ok(tpe)
    }

    pub fn marker_for(&mut self, expr: &Expr) -> Result<Marker, TypeError> {
        if let Some(marker) = self.markers.get(&expr.id()) {
            Ok(*marker)
        } else if let Some(iden) = expr
            .inner()
            .as_identifier()
            .or_else(|| expr.inner().as_current_access())
        {
            if let Some(field) = self.locals().get(&iden.lexeme) {
                Ok(field.marker)
            } else if let Some(field) = self.active_self().get(&iden.lexeme) {
                Ok(field.marker)
            } else {
                duck_error!("Unrecognized variable: {iden}")
            }
        } else {
            Ok(self.new_marker(expr))
        }
    }

    pub fn apply_field_to_self(
        &mut self,
        name: &str,
        expr: &Expr,
        value: Marker,
        op: RecordOp,
    ) -> Result<Marker, TypeError> {
        let field = Field::new(expr, op);
        Printer::give_expr_alias(field.marker, expr.to_string());
        let marker = self.active_self_mut().apply_field(name, field, value)?.apply(self)?;
        Ok(marker)
    }

    pub fn apply_field_to_local(
        &mut self,
        name: &str,
        expr: &Expr,
        value: Marker,
        op: RecordOp,
    ) -> Result<Marker, TypeError> {
        let field = Field::new(expr, op);
        Printer::give_expr_alias(field.marker, expr.to_string());
        let marker = self.locals_mut().apply_field(name, field, value)?.apply(self)?;
        Ok(marker)
    }

    pub fn new_marker(&mut self, expr: &Expr) -> Marker {
        let marker = Marker::new();
        Printer::give_expr_alias(marker, expr.to_string());
        self.markers.insert(expr.id(), marker);
        marker
    }

    pub fn active_self(&self) -> &Record {
        match self.substitutions.get(&self.active_self).unwrap() {
            Term::App(App::Record(record)) => record,
            _ => unreachable!(),
        }
    }

    pub fn active_self_mut(&mut self) -> &mut Record {
        match self.substitutions.get_mut(&self.active_self).unwrap() {
            Term::App(App::Record(record)) => record,
            _ => unreachable!(),
        }
    }

    pub fn active_self_marker(&self) -> Marker {
        self.active_self
    }

    pub fn locals_marker(&self) -> Marker {
        self.locals
    }

    pub fn locals(&self) -> &Record {
        match self.substitutions.get(&self.locals).unwrap() {
            Term::App(App::Record(record)) => record,
            _ => unreachable!(),
        }
    }

    pub fn locals_mut(&mut self) -> &mut Record {
        match self.substitutions.get_mut(&self.locals).unwrap() {
            Term::App(App::Record(record)) => record,
            _ => unreachable!(),
        }
    }

    pub fn set_locals(&mut self, marker: Marker) -> Marker {
        let m = self.locals;
        self.locals = marker;
        m
    }

    pub fn set_active_self(&mut self, marker: Marker) -> Marker {
        let m = self.active_self;
        self.active_self = marker;
        m
    }

    pub fn new_scope(&mut self) -> Marker {
        let marker = Marker::new();
        self.substitutions
            .insert(marker, Term::App(App::Record(Record::extendable())));
        marker
    }
}

// Unification
impl Typewriter {
    pub fn unify_marker(&mut self, marker: &Marker, term: &mut Term) -> Result<(), TypeError> {
        println!("{}", Printer::marker_unification(marker, term, self));

        if let Some(mut sub) = self.substitutions.get_mut(marker).cloned() {
            // self.normalize_term(term);
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
                            record.apply_field(name, *rhs_field, rhs_field.marker)?.apply(self)?;
                        }
                        Ok(())
                    }
                    _ => {
                        duck_error!("{} is does not contain fields", Printer::term(rhs, self))
                    }
                },
                _ => Ok(()),
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
                App::Record(record) => record.fields.iter().any(|(_, field)| marker == &field.marker),
                App::Function(Function {
                    parameters,
                    return_type,
                    ..
                }) => self.occurs(marker, return_type) || parameters.iter().any(|v| self.occurs(marker, v)),
                App::Call(Call { target, parameters }) => {
                    self.occurs(marker, target) || parameters.iter().any(|v| self.occurs(marker, v))
                }
            },
        }
    }

    fn normalize_term(&self, term: &mut Term) -> Result<(), TypeError> {
        match term {
            Term::Type(_) => Ok(()),
            Term::Marker(marker) => {
                if let Some(sub) = self.substitutions.get(marker) {
                    *term = sub.clone();
                    self.normalize_term(term)
                } else {
                    Ok(())
                }
            }
            Term::App(app) => match app {
                App::Array(member_term) => self.normalize_term(member_term),
                App::Record(_) => Ok(()),
                App::Function(super::Function {
                    parameters,
                    return_type,
                    ..
                }) => {
                    parameters.iter_mut().try_for_each(|v| self.normalize_term(v))?;
                    self.normalize_term(return_type)
                }
                App::Call(super::Call {
                    target,
                    parameters: arguments,
                }) => {
                    // arguments.iter_mut().try_for_each(|v| self.normalize_term(v))?;
                    self.normalize_term(target)?;
                    match target.as_mut() {
                        Term::Marker(_) => Ok(()),
                        Term::App(App::Function(function)) => {
                            println!("\n--- Calling function... ---\n");
                            let mut temp_writer = self.clone();
                            let mut temp_parameters = function.parameters.clone();
                            let mut temp_return = function.return_type.clone();
                            for (i, param) in temp_parameters.iter_mut().enumerate() {
                                let arg = if let Some(arg) = arguments.get_mut(i) {
                                    arg
                                } else {
                                    return duck_error!("Missing argument {i} in call.");
                                };
                                temp_writer.unify_terms(param, arg)?;
                            }
                            temp_writer.normalize_term(&mut temp_return)?;
                            *term = *temp_return;
                            println!("\n--- Ending call... ---\n");
                            Ok(())
                        }
                        t => duck_error!("Invalid call target: {}", Printer::term(t, self)),
                    }
                }
            },
        }
    }

    /// ### Errors
    /// shut up
    pub fn new_substitution(&mut self, marker: Marker, term: Term) -> Result<(), TypeError> {
        // self.normalize_term(&mut term);
        println!("{}", Printer::substitution(&marker, &term, self));
        self.substitutions.insert(marker, term);
        // let markers_needing_updates: Vec<Marker> = self
        //     .substitutions
        //     .iter()
        //     .filter(|(_, sub_term)| self.occurs(&marker, sub_term))
        //     .map(|(marker, _)| *marker)
        //     .collect();
        // for marker in markers_needing_updates {
        //     let mut term = self.substitutions.remove(&marker).unwrap();
        //     // self.normalize_term(&mut term);
        //     self.substitutions.insert(marker, term);
        // }
        Ok(())
    }
}

impl Default for Typewriter {
    fn default() -> Self {
        let active_self = Marker::new();
        let locals = Marker::new();
        let substitutions = HashMap::from([
            (active_self, Term::App(App::Record(Record::extendable()))),
            (locals, Term::App(App::Record(Record::extendable()))),
        ]);
        Self {
            substitutions,
            active_self,
            locals,
            markers: HashMap::default(),
        }
    }
}

pub type TypeError = Diagnostic<FileId>;
