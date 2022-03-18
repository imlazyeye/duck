use colored::Colorize;
use hashbrown::HashMap;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    /// We do not know the type of this symbol
    Unknown,
    /// The GM constant value of `undefined`
    Undefined,
    /// The GM constant value of `noone`
    Noone,
    /// True or false
    Bool,
    /// A number
    Real,
    /// A string of text
    String,
    /// An array containing values of the nested type
    Array(Box<Type>),
    /// A struct with the given fields
    Struct(HashMap<String, Type>),
}
impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Unknown => f.pad("<?>"),
            Type::Undefined => f.pad("Undefined"),
            Type::Noone => f.pad("Noone"),
            Type::Bool => f.pad("Bool"),
            Type::Real => f.pad("Real"),
            Type::String => f.pad("String"),
            Type::Array(inner) => f.pad(&format!("[{}]", *inner)),
            Type::Struct(fields) => {
                f.pad("{")?;
                for (name, symbol) in fields.iter() {
                    f.pad(&format!(" {name}: {symbol},"))?;
                }
                f.pad(" }")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Constraint {
    pub marker: Marker,
    pub symbol: Symbol,
}
impl Display for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(&format!(
            "{} = {}",
            self.marker.to_string().bright_cyan(),
            format!("{}", self.symbol).bright_blue()
        ))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Symbol {
    Constant(Type),
    Variable(Marker),
    Application(Application),
    Deref(Deref),
}
impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::Constant(tpe) => f.pad(&tpe.to_string()),
            Symbol::Variable(marker) => f.pad(&marker.to_string()),
            Symbol::Application(application) => f.pad(&application.to_string()),
            Symbol::Deref(deref) => f.pad(&deref.to_string()),
        }
    }
}
impl From<Symbol> for Type {
    fn from(symbol: Symbol) -> Self {
        match symbol {
            Symbol::Constant(tpe) => tpe,
            Symbol::Variable(_) => Type::Unknown,
            Symbol::Application(app) => match app {
                Application::Array(inner_symbol) => Type::Array(Box::new(Type::from(inner_symbol.as_ref().to_owned()))),
                Application::Object(fields) => {
                    let mut tpe_fields = HashMap::new();
                    for (name, symbol) in fields {
                        tpe_fields.insert(name, symbol.into());
                    }
                    Type::Struct(tpe_fields)
                }
            },
            Symbol::Deref(_) => Type::Unknown,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Application {
    Array(Box<Symbol>),
    Object(HashMap<String, Symbol>),
}
impl Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Application::Array(symbol) => f.pad(&format!("[{symbol}]")),
            Application::Object(fields) => {
                f.pad("{")?;
                for (name, symbol) in fields.iter() {
                    f.pad(&format!(" {name}: {symbol},"))?;
                }
                f.pad(" }")
            }
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
    pub fn new() -> Self {
        Self(rand::random())
    }
}
impl Display for Marker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(&format!("t{}", self.0))
    }
}
