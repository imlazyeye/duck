use crate::parse::{Block, Expr, Function, StmtType};

use super::{Constraint, Page};
use colored::Colorize;
use hashbrown::HashMap;
use itertools::Itertools;

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Generic {
        term: Box<Term>,
    },
    Unknown,
    Undefined,
    Noone,
    Bool,
    Real,
    String,
    Array {
        member_type: Box<Type>,
    },
    Struct {
        fields: HashMap<String, Type>,
    },
    Union {
        types: Vec<Type>,
    },
    Function {
        parameters: Vec<Type>,
        return_type: Box<Type>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Term {
    Type(Type),
    Marker(Marker),
    App(App),
    Deref(Deref),
    Impl(Impl),
}

impl From<Term> for Type {
    fn from(term: Term) -> Self {
        match term {
            Term::Type(tpe) => tpe,
            Term::Marker(marker) => Type::Generic {
                term: Box::new(Term::Marker(marker)),
            },
            Term::App(app) => match app {
                App::Array(member_type) => Type::Array {
                    member_type: Box::new(Type::from(member_type.as_ref().to_owned())),
                },
                App::Object(fields) => {
                    let mut tpe_fields = HashMap::new();
                    for (name, term) in fields {
                        tpe_fields.insert(name, term.into());
                    }
                    Type::Struct { fields: tpe_fields }
                }
                App::Function(params, return_type, _) => Type::Function {
                    parameters: params.into_iter().map(|param| param.into()).collect(),
                    return_type: Box::new(return_type.as_ref().clone().into()),
                },
            },
            Term::Deref(deref) => match deref {
                Deref::Call { target, arguments } => Type::Generic {
                    term: Box::new(Term::Deref(Deref::Call { target, arguments })),
                },
                Deref::Field { field_name, target } => todo!(),
                Deref::MemberType { target } => todo!(),
            },
            Term::Impl(imp) => match imp {
                Impl::Fields(fields) => Type::Generic {
                    term: Box::new(Term::Impl(Impl::Fields(fields))),
                },
            },
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum App {
    Array(Box<Term>),
    Object(HashMap<String, Term>),
    Function(Vec<Term>, Box<Term>, Function),
}
impl App {
    pub fn process_function(function: Function, page: &mut Page, printer: &mut Printer) -> (Vec<Term>, Box<Term>) {
        let body = match function.body.inner() {
            StmtType::Block(Block { body, .. }) => body,
            _ => unreachable!(),
        };
        page.apply_stmts(body, printer);
        let mut parameters = Vec::new();
        for param in function.parameters.iter() {
            let param_marker = page.scope.get_expr_marker(param.name_expr());
            let param_term = page.marker_to_term(param_marker);
            parameters.push(param_term);
        }
        (parameters, Box::new(page.return_term()))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Impl {
    Fields(HashMap<String, Term>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Deref {
    Field { field_name: String, target: Box<Term> },
    MemberType { target: Box<Term> },
    Call { target: Box<Term>, arguments: Vec<Term> },
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
pub struct Marker(pub u64);
impl Marker {
    pub const RETURN_VALUE: Self = Marker(u64::MAX);
    pub fn new() -> Self {
        Self(rand::random())
    }
}

#[derive(Debug, Default, Clone)]
pub struct Printer {
    aliases: HashMap<Marker, usize>,
    expr_strings: HashMap<Marker, String>,
    iter: usize,
}
impl Printer {
    pub fn give_expr_alias(&mut self, _marker: Marker, _expr_string: String) {
        // self.expr_strings.insert(marker, expr_string);
    }

    #[must_use]
    pub fn marker(&mut self, marker: &Marker) -> String {
        if marker == &Marker::RETURN_VALUE {
            "tR".into()
        } else if let Some(expr_string) = self.expr_strings.get(marker) {
            expr_string.clone()
        } else {
            format!(
                "t{}",
                self.aliases.entry(*marker).or_insert_with(|| {
                    let v = self.iter;
                    self.iter += 1;
                    v
                })
            )
        }
        .bright_black()
        .bold()
        .to_string()
    }

    #[must_use]
    pub fn term(&mut self, term: &Term) -> String {
        match term {
            Term::Type(tpe) => self.tpe(tpe),
            Term::Marker(marker) => self.marker(marker),
            Term::App(app) => self.app(app),
            Term::Deref(deref) => self.deref(deref),
            Term::Impl(imp) => self.imp(imp),
        }
    }

    #[must_use]
    pub fn tpe(&mut self, tpe: &Type) -> String {
        let s = match tpe {
            Type::Generic { term } => format!("T where T {}", self.term(term)),
            Type::Unknown => "<?>".into(),
            Type::Undefined => "undefined".into(),
            Type::Noone => "noone".into(),
            Type::Bool => "bool".into(),
            Type::Real => "real".into(),
            Type::String => "string".into(),
            Type::Array { member_type } => format!("[{}]", self.tpe(member_type)),
            Type::Struct { fields } => format!(
                "{{ {} }}",
                fields
                    .iter()
                    .map(|(name, inner_tpe)| format!("{name}: {}", self.tpe(inner_tpe)))
                    .join(", ")
            ),
            Type::Union { types } => types.iter().map(|u| self.tpe(u)).join("| "),
            Type::Function {
                parameters,
                return_type,
            } => format!(
                "fn({}) -> {}",
                parameters.iter().map(|param| self.tpe(param)).join(", "),
                self.tpe(return_type)
            ),
        };
        s.blue().bold().to_string()
    }

    #[must_use]
    pub fn app(&mut self, app: &App) -> String {
        match app {
            App::Array(inner) => format!("[{}]", self.term(inner)),
            App::Object(fields) => format!(
                "{{ {} }}",
                fields
                    .iter()
                    .map(|(name, term)| format!("{name}: {}", self.term(term)))
                    .join(", ")
            ),
            App::Function(arguments, return_type, _) => format!(
                "({}) -> {}",
                arguments.iter().map(|term| self.term(term)).join(", "),
                self.term(return_type),
            ),
        }
    }

    #[must_use]
    pub fn deref(&mut self, deref: &Deref) -> String {
        match deref {
            Deref::Call { target, arguments } => format!(
                "{}({})",
                self.term(target),
                arguments.iter().map(|term| self.term(term)).join(", ")
            ),
            Deref::Field { field_name, target } => {
                format!("{}.{field_name}", self.term(target))
            }
            Deref::MemberType { target } => format!("{}[*]", self.term(target)),
        }
    }

    #[must_use]
    pub fn imp(&mut self, imp: &Impl) -> String {
        match imp {
            Impl::Fields(fields) => format!(
                "impl {{ {} }}",
                fields
                    .iter()
                    .map(|(name, term)| format!("{name}: {}", self.term(term)))
                    .join(", ")
            ),
        }
    }

    #[must_use]
    pub fn constraint(&mut self, constraint: &Constraint) -> String {
        format!(
            "{}     {} = {}",
            "EQ".bright_magenta(),
            self.marker(&constraint.marker),
            self.term(&constraint.term)
        )
    }
}
