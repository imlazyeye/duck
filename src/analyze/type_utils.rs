use colored::Colorize;
use hashbrown::HashMap;
use itertools::Itertools;

use super::{Constraint, Page, Scope, Unifier};

#[derive(Debug, PartialEq, Eq, Clone)]
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Term {
    Type(Type),
    Marker(Marker),
    App(App),
    Rule(Rule),
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
                App::Function(params, page) => Type::Function {
                    parameters: params.into_iter().map(|(_, param)| param.into()).collect(),
                    return_type: Box::new(page.return_term().into()),
                },
                App::Call(target, arguments) => match target.as_ref() {
                    Term::Rule(Rule::Function(..)) => target.as_ref().clone().into(),
                    _ => Type::Generic {
                        term: Box::new(Term::App(App::Call(target, arguments))),
                    },
                },
            },
            Term::Rule(rule) => match rule {
                Rule::Field(_, _) => todo!(),
                Rule::Function(term, parameters) => Type::Function {
                    parameters: parameters.into_iter().map(|v| v.into()).collect(),
                    return_type: Box::new(term.as_ref().clone().into()),
                },
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum App {
    Array(Box<Term>),
    Object(HashMap<String, Term>),
    Call(Box<Term>, Vec<Term>), // todo: can i remove this?
    Function(Vec<(String, Term)>, Page),
}
impl App {
    pub fn checkout_function(parameters: &[(String, Term)], page: &Page) -> (Vec<(String, Term)>, Page) {
        fn translate_term(term: &mut Term, translator: &HashMap<Marker, Marker>) {
            match term {
                Term::Type(tpe) => {
                    if let Type::Generic { term: inner_term } = tpe {
                        translate_term(inner_term, translator)
                    };
                }
                Term::Marker(marker) => *marker = *translator.get(marker).unwrap(),
                Term::App(app) => match app {
                    App::Array(inner_term) => translate_term(inner_term, translator),
                    App::Object(fields) => fields
                        .iter_mut()
                        .for_each(|(_, field)| translate_term(field, translator)),
                    App::Call(target, arguments) => {
                        translate_term(target, translator);
                        arguments.iter_mut().for_each(|arg| translate_term(arg, translator))
                    }
                    App::Function(parameters, page) => {
                        (*parameters, *page) = App::checkout_function(parameters, page);
                    }
                },
                Term::Rule(rule) => match rule {
                    Rule::Field(_, term) => translate_term(term, translator),
                    Rule::Function(term, parameters) => {
                        translate_term(term, translator);
                        parameters
                            .iter_mut()
                            .for_each(|param| translate_term(param, translator))
                    }
                },
            }
        }

        let mut translator: HashMap<Marker, Marker> = HashMap::default();
        translator.insert(Marker::RETURN_VALUE, Marker::RETURN_VALUE);
        let scope = Scope {
            fields: page.scope.fields.clone(),
            generics: page
                .scope
                .generics
                .iter()
                .map(|generic| {
                    let new_generic = Marker::new();
                    translator.insert(*generic, new_generic);
                    new_generic
                })
                .collect(),
            markers: page
                .scope
                .markers
                .iter()
                .map(|(expr_id, old_marker)| {
                    let new_marker = Marker::new();
                    translator.insert(*old_marker, new_marker);
                    (*expr_id, new_marker)
                })
                .collect(),

            file_id: page.scope.file_id,
            expr_strings: page.scope.expr_strings.clone(),
        };

        let unifier = Unifier {
            collection: page
                .unifier
                .collection
                .clone()
                .iter_mut()
                .map(|(mut marker, term)| {
                    marker = translator.get(marker).unwrap();
                    translate_term(term, &translator);
                    (*marker, term.clone())
                })
                .collect(),
        };

        let parameters = parameters
            .to_vec()
            .iter_mut()
            .map(|(n, param)| {
                translate_term(param, &translator);
                (n.clone(), param.clone())
            })
            .collect();

        let new_page = Page {
            scope,
            unifier,
            file_id: page.file_id,
        };

        (parameters, new_page)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Rule {
    Field(String, Box<Term>),
    Function(Box<Term>, Vec<Term>),
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
    pub fn give_expr_alias(&mut self, marker: Marker, expr_string: String) {
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
    }

    #[must_use]
    pub fn term(&mut self, term: &Term) -> String {
        match term {
            Term::Type(tpe) => self.tpe(tpe),
            Term::Marker(marker) => self.marker(marker),
            Term::App(app) => self.app(app),
            Term::Rule(rule) => self.rule(rule),
        }
    }

    #[must_use]
    pub fn tpe(&mut self, tpe: &Type) -> String {
        let s = match tpe {
            Type::Generic { term } => self.term(term),
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
                "function({}) -> {}",
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

            App::Call(call_target, arguments) => format!(
                "{}({})",
                self.term(call_target),
                arguments.iter().map(|term| self.term(term)).join(", ")
            ),
            App::Function(arguments, page) => format!(
                "({}) -> {}",
                arguments.iter().map(|(_, term)| self.term(term)).join(", "),
                self.term(&page.marker_to_term(Marker::RETURN_VALUE)),
            ),
        }
    }

    #[must_use]
    pub fn rule(&mut self, rule: &Rule) -> String {
        match rule {
            Rule::Field(name, term) => format!("?.{name} -> {}", self.term(term)),
            Rule::Function(term, params) => format!(
                "?({}) -> {}",
                params.iter().map(|param| self.term(param)).join(", "),
                self.term(term)
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
