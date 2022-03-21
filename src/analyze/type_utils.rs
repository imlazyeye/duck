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
    Application(Application),
    Inspection(Inspection),
    Union(Vec<Term>),
}
impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::Type(tpe) => f.pad(&tpe.to_string()),
            Term::Marker(marker) => f.pad(&marker.to_string()),
            Term::Application(application) => f.pad(&application.to_string()),
            Term::Inspection(inspection) => f.pad(&inspection.to_string()),
            Term::Union(unions) => f.pad(&unions.iter().join("| ")),
        }
    }
}
impl From<Term> for Type {
    fn from(term: Term) -> Self {
        match term {
            Term::Type(tpe) => tpe,
            Term::Marker(marker) => Type::Generic { marker },
            Term::Application(app) => match app {
                Application::Array { member_type } => Type::Array {
                    member_type: Box::new(Type::from(member_type.as_ref().to_owned())),
                },
                Application::Object { fields } => {
                    let mut tpe_fields = HashMap::new();
                    for (name, term) in fields {
                        tpe_fields.insert(name, term.into());
                    }
                    Type::Struct { fields: tpe_fields }
                }
                Application::Call { call_target, arguments } => {
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
            },
            Term::Inspection(_) => Type::Unknown,
            Term::Union(unions) => Type::Union {
                types: unions.iter().map(|u| u.clone().into()).collect(),
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Application {
    Array {
        member_type: Box<Term>,
    },
    Object {
        fields: HashMap<String, Term>,
    },
    Call {
        call_target: Box<Term>,
        arguments: Vec<Term>,
    },
}
impl Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Application::Array { member_type: inner } => f.pad(&format!("[{inner}]")),
            Application::Object { fields } => f.pad(&format!(
                "{{ {} }}",
                fields.iter().map(|(name, term)| format!("{name}: {term}")).join(", ")
            )),
            Application::Call { call_target, arguments } => f.pad(&format!(
                "{call_target}({})",
                arguments.iter().map(|term| term.to_string()).join(", ")
            )),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Inspection {
    pub marker: Marker,
    pub field: String,
}
impl Display for Inspection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(&format!("{}.{}", self.marker, self.field))
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
