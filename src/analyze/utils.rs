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
        match constraint {
            Constraint::Eq(marker, term) => format!(
                "{}    {} = {}",
                "CON".bright_magenta(),
                self.marker(marker),
                self.term(term)
            ),
            Constraint::Impl(marker, imp) => format!(
                "{}    {} ~> {}",
                "CON".bright_magenta(),
                self.marker(marker),
                self.imp(imp)
            ),
        }
    }
}
