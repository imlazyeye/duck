use std::sync::Mutex;

use super::*;
use colored::Colorize;
use hashbrown::HashMap;
use itertools::Itertools;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
pub struct Marker(pub u64);
impl Marker {
    pub const RETURN_VALUE: Self = Marker(u64::MAX);
    pub fn new() -> Self {
        Self(rand::random())
    }
}

lazy_static! {
    static ref PRINTER: Mutex<Printer> = Mutex::new(Printer {
        aliases: HashMap::default(),
        expr_strings: HashMap::default(),
        iter: 0,
    });
}

pub struct Printer {
    aliases: HashMap<Marker, usize>,
    expr_strings: HashMap<Marker, String>,
    iter: usize,
}
impl Printer {
    pub fn give_expr_alias(_marker: Marker, _expr_string: String) {
        // self.expr_strings.insert(marker, expr_string);
    }

    #[must_use]
    pub fn marker(marker: &Marker) -> String {
        let mut printer = PRINTER.lock().unwrap();
        if marker == &Marker::RETURN_VALUE {
            "tR".into()
        } else if let Some(expr_string) = printer.expr_strings.get(marker) {
            expr_string.clone()
        } else {
            let entry = if let Some(entry) = printer.aliases.get(marker) {
                *entry
            } else {
                let v = printer.iter;
                printer.iter += 1;
                v
            };
            format!("t{}", entry)
        }
        .bright_black()
        .bold()
        .to_string()
    }

    #[must_use]
    pub fn term(term: &Term) -> String {
        match term {
            Term::Type(tpe) => Self::tpe(tpe),
            Term::Marker(marker) => Self::marker(marker),
            Term::App(app) => Self::app(app),
            Term::Deref(deref) => Self::deref(deref),
            Term::Impl(imp) => Self::imp(imp),
        }
    }

    #[must_use]
    pub fn tpe(tpe: &Type) -> String {
        let s = match tpe {
            Type::Generic { term } => format!("T where T {}", Self::term(term)),
            Type::Unknown => "<?>".into(),
            Type::Undefined => "undefined".into(),
            Type::Noone => "noone".into(),
            Type::Bool => "bool".into(),
            Type::Real => "real".into(),
            Type::String => "string".into(),
            Type::Array { member_type } => format!("[{}]", Self::tpe(member_type)),
            Type::Struct { fields } => format!(
                "{{ {} }}",
                fields
                    .iter()
                    .map(|(name, inner_tpe)| format!("{name}: {}", Self::tpe(inner_tpe)))
                    .join(", ")
            ),
            Type::Union { types } => types.iter().map(Self::tpe).join("| "),
            Type::Function {
                parameters,
                return_type,
            } => format!(
                "fn({}) -> {}",
                parameters.iter().map(Self::tpe).join(", "),
                Self::tpe(return_type)
            ),
        };
        s.blue().bold().to_string()
    }

    #[must_use]
    pub fn app(app: &App) -> String {
        match app {
            App::Array(inner) => format!("[{}]", Self::term(inner)),
            App::Object(fields) => format!(
                "{{ {} }}",
                fields
                    .iter()
                    .map(|(name, term)| format!("{name}: {}", Self::term(term)))
                    .join(", ")
            ),
            App::Function(arguments, return_type, _) => format!(
                "({}) -> {}",
                arguments.iter().map(Self::term).join(", "),
                Self::term(return_type),
            ),
        }
    }

    #[must_use]
    pub fn deref(deref: &Deref) -> String {
        match deref {
            Deref::Call { target, arguments } => format!(
                "{}({})",
                Self::term(target),
                arguments.iter().map(Self::term).join(", ")
            ),
            Deref::Field { field_name, target } => {
                format!("{}.{field_name}", Self::term(target))
            }
            Deref::MemberType { target } => format!("{}[*]", Self::term(target)),
        }
    }

    #[must_use]
    pub fn imp(imp: &Impl) -> String {
        match imp {
            Impl::Fields(fields) => format!(
                "impl {{ {} }}",
                fields
                    .iter()
                    .map(|(name, term)| format!("{name}: {}", Self::term(term)))
                    .join(", ")
            ),
        }
    }

    #[must_use]
    pub fn constraint(constraint: &Constraint) -> String {
        match constraint {
            Constraint::Eq(marker, term) => format!(
                "{}    {} = {}",
                "CON".bright_magenta(),
                Self::marker(marker),
                Self::term(term)
            ),
            Constraint::Impl(marker, imp) => format!(
                "{}    {} ~> {}",
                "CON".bright_magenta(),
                Self::marker(marker),
                Self::imp(imp)
            ),
        }
    }
}
