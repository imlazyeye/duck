use hashbrown::HashMap;
use itertools::Itertools;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    Generic {
        marker: Marker,
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
impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Generic { marker } => f.pad(&marker.to_string()),
            Type::Unknown => f.pad("<?>"),
            Type::Undefined => f.pad("undefined"),
            Type::Noone => f.pad("noone"),
            Type::Bool => f.pad("bool"),
            Type::Real => f.pad("real"),
            Type::String => f.pad("string"),
            Type::Array { member_type } => f.pad(&format!("[{}]", *member_type)),
            Type::Struct { fields } => f.pad(&format!(
                "{{ {} }}",
                fields.iter().map(|(name, term)| format!("{name}: {term}")).join(", ")
            )),
            Type::Union { types } => f.pad(&types.iter().join("| ")),
            Type::Function {
                parameters,
                return_type,
            } => f.pad(&format!("function({}) -> {return_type}", parameters.iter().join(", "))),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Term {
    Type(Type),
    Marker(Marker),
    App(App),
}
impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::Type(tpe) => f.pad(&tpe.to_string()),
            Term::Marker(marker) => f.pad(&marker.to_string()),
            Term::App(application) => f.pad(&application.to_string()),
        }
    }
}
impl From<Term> for Type {
    fn from(term: Term) -> Self {
        match term {
            Term::Type(tpe) => tpe,
            Term::Marker(marker) => Type::Generic { marker },
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
                App::Call(call_target, arguments) => {
                    if let Term::Type(Type::Function {
                        parameters,
                        return_type,
                    }) = call_target.as_ref()
                    {
                        match return_type.as_ref() {
                            Type::Generic { marker } => {
                                let position = parameters
                                    .iter()
                                    .position(|v| v == &Type::Generic { marker: *marker })
                                    .expect("that ain't right");
                                let argument = arguments.get(position).expect("missing argument");
                                argument.clone().into()
                            }
                            tpe => tpe.clone(),
                        }
                    } else {
                        Type::Unknown
                    }
                }
                App::Inspect(_, _) => Type::Unknown,
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum App {
    Array(Box<Term>),
    Object(HashMap<String, Term>),
    Call(Box<Term>, Vec<Term>),
    Inspect(String, Box<Term>),
}
impl Display for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            App::Array(inner) => f.pad(&format!("[{inner}]")),
            App::Object(fields) => f.pad(&format!(
                "{{ {} }}",
                fields.iter().map(|(name, term)| format!("{name}: {term}")).join(", ")
            )),

            App::Call(call_target, arguments) => f.pad(&format!(
                "{call_target}({})",
                arguments.iter().map(|term| term.to_string()).join(", ")
            )),
            App::Inspect(name, term) => f.pad(&format!("{term}.{name}")),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
pub struct Marker(pub u64);
impl Marker {
    pub const RETURN_VALUE: Self = Marker(u64::MAX);
    pub fn new() -> Self {
        Self(rand::random())
    }
}
impl Display for Marker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self == &Marker::RETURN_VALUE {
            f.pad("tR")
        } else {
            f.pad(&format!("t{}", self.0))
        }
    }
}
