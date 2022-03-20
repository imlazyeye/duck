use codespan_reporting::diagnostic::{Diagnostic, Label};
use colored::Colorize;
use hashbrown::HashMap;
use itertools::Itertools;
use std::fmt::Display;

use crate::{
    parse::{Expr, ExprId, Identifier},
    FileId,
};

use super::{Page, Scope};

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
                fields
                    .iter()
                    .map(|(name, symbol)| format!("{name}: {symbol}"))
                    .join(", ")
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
pub enum Symbol {
    Constant(Type),
    Variable(Marker),
    Application(Application),
    Deref(Deref),
    Union(Vec<Symbol>),
}
impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::Constant(tpe) => f.pad(&tpe.to_string()),
            Symbol::Variable(marker) => f.pad(&marker.to_string()),
            Symbol::Application(application) => f.pad(&application.to_string()),
            Symbol::Deref(deref) => f.pad(&deref.to_string()),
            Symbol::Union(unions) => f.pad(&unions.iter().join("| ")),
        }
    }
}
impl From<Symbol> for Type {
    fn from(symbol: Symbol) -> Self {
        match symbol {
            Symbol::Constant(tpe) => tpe,
            Symbol::Variable(marker) => Type::Generic { marker },
            Symbol::Application(app) => match app {
                Application::Array { member_type } => Type::Array {
                    member_type: Box::new(Type::from(member_type.as_ref().to_owned())),
                },
                Application::Object { fields } => {
                    let mut tpe_fields = HashMap::new();
                    for (name, symbol) in fields {
                        tpe_fields.insert(name, symbol.into());
                    }
                    Type::Struct { fields: tpe_fields }
                }
            },
            Symbol::Deref(_) => Type::Unknown,
            Symbol::Union(unions) => Type::Union {
                types: unions.iter().map(|u| u.clone().into()).collect(),
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Application {
    Array { member_type: Box<Symbol> },
    Object { fields: HashMap<String, Symbol> },
}
impl Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Application::Array { member_type: inner } => f.pad(&format!("[{inner}]")),
            Application::Object { fields } => f.pad(&format!(
                "{{ {} }}",
                fields
                    .iter()
                    .map(|(name, symbol)| format!("{name}: {symbol}"))
                    .join(", ")
            )),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Deref {
    Array(Marker),
    Object(Marker, String),
}
impl Display for Deref {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Deref::Array(marker) => f.pad(&format!("*{marker}")),
            Deref::Object(marker, field) => f.pad(&format!("{marker}.{}", field)),
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
