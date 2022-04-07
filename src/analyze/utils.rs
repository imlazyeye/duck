use super::*;
use colored::Colorize;
use hashbrown::HashMap;
use itertools::Itertools;
use parking_lot::Mutex;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
pub struct Marker(pub u64);
impl Marker {
    pub const RETURN: Self = Marker(u64::MAX);
    pub const NULL: Self = Marker(u64::MAX - 1);
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
    pub fn flush() {
        let mut printer = PRINTER.lock();
        printer.aliases.clear();
        printer.expr_strings.clear();
        printer.iter = 0;
    }

    pub fn give_expr_alias(marker: Marker, name: String) {
        println!(
            "{}        {}   :   {}",
            "ALIAS".bright_red(),
            Printer::marker(&marker),
            name
        );
        // PRINTER.lock().expr_strings.insert(marker, name);
    }

    #[must_use]
    pub fn marker(marker: &Marker) -> String {
        let mut printer = PRINTER.lock();
        match *marker {
            Marker::RETURN => "tR".into(),
            marker => {
                if let Some(expr_string) = printer.expr_strings.get(&marker) {
                    expr_string.clone()
                } else {
                    let entry = if let Some(entry) = printer.aliases.get(&marker) {
                        *entry
                    } else {
                        let v = printer.iter;
                        printer.iter += 1;
                        printer.aliases.insert(marker, v);
                        v
                    };
                    format!("t{}", entry)
                }
            }
        }
        .bright_black()
        .bold()
        .to_string()
    }

    #[must_use]
    pub fn term(term: &Term, tw: &Typewriter) -> String {
        match term {
            Term::Type(tpe) => Self::tpe(tpe, tw),
            Term::Marker(marker) => Self::marker(marker),
            Term::App(app) => Self::app(app, tw),
        }
    }

    #[must_use]
    pub fn tpe(tpe: &Type, tw: &Typewriter) -> String {
        let s = match tpe {
            Type::Generic { term } => Self::term(term, tw),
            Type::Any => "any".into(),
            Type::Undefined => "undefined".into(),
            Type::Noone => "noone".into(),
            Type::Bool => "bool".into(),
            Type::Real => "real".into(),
            Type::Str => "string".into(),
            Type::Array { member_type } => format!("[{}]", Self::tpe(member_type, tw)),
            Type::Struct { fields } => format!(
                "{{ {} }}",
                fields
                    .iter()
                    .map(|(name, inner_tpe)| format!("{name}: {}", Self::tpe(inner_tpe, tw)))
                    .join(", ")
            ),
            Type::Union { types } => types.iter().map(|v| Self::tpe(v, tw)).join("| "),
            Type::Function {
                parameters,
                return_type,
            } => format!(
                "fn ({}) -> {}",
                parameters.iter().map(|v| Self::tpe(v, tw)).join(", "),
                Self::tpe(return_type, tw)
            ),
        };
        s.blue().bold().to_string()
    }

    #[must_use]
    pub fn app(app: &App, tw: &Typewriter) -> String {
        match app {
            App::Array(inner) => format!("[{}]", Self::term(inner, tw)),
            App::Record(record) => {
                if record.fields.is_empty() {
                    "{}".into()
                } else {
                    format!(
                        "{}{{ {} }}",
                        match record.state {
                            State::Inferred => "?",
                            State::Extendable => "mut ",
                            State::Concrete => "",
                        },
                        record
                            .fields
                            .iter()
                            .map(|(name, field)| format!("{}: {}", name, Printer::marker(&field.marker)))
                            .join(", ")
                    )
                }
            }
            App::Function(Function {
                parameters,
                return_type,
                ..
            }) => format!(
                "fn ({}) -> {}",
                parameters.iter().map(|v| Printer::term(v, tw)).join(", "),
                Printer::term(return_type, tw)
            ),
            App::Call(Call { parameters, target }) => format!(
                "{}({})",
                Printer::term(target, tw),
                parameters.iter().map(|v| Printer::term(v, tw)).join(", "),
            ),
        }
    }

    #[must_use]
    pub fn term_unification(a: &Term, b: &Term, tw: &Typewriter) -> String {
        format!(
            "{}      {}   ≟   {}",
            "UNIFY T".bright_yellow(),
            Printer::term(a, tw),
            Printer::term(b, tw),
        )
    }

    #[must_use]
    pub fn marker_unification(marker: &Marker, term: &Term, tw: &Typewriter) -> String {
        format!(
            "{}      {}   ≟   {}",
            "UNIFY M".bright_yellow(),
            Printer::marker(marker),
            Printer::term(term, tw),
        )
    }

    #[must_use]
    pub fn substitution(marker: &Marker, term: &Term, tw: &Typewriter) -> String {
        format!(
            "{}          {}   →   {}",
            "SUB".bright_green(),
            Printer::marker(marker),
            Printer::term(term, tw),
        )
    }

    #[must_use]
    pub fn constraint(constraint: &Constraint, tw: &Typewriter) -> String {
        match constraint {
            Constraint::Eq(marker, term) => format!(
                "{}          {}   =   {}",
                "CON".bright_magenta(),
                Self::marker(marker),
                Self::term(term, tw),
            ),
        }
    }
}

#[macro_export]
macro_rules! duck_error {
    ($($arg:tt)*) => {
        Err(codespan_reporting::diagnostic::Diagnostic::error().with_message(format!($($arg)*)))
    };
}

#[macro_export]
macro_rules! duck_bug {
    ($($msg_arg:expr), * $(,)?) => {
        Err(codespan_reporting::diagnostic::Diagnostic::bug().with_message(format!($($msg_arg)*)))
    };
}
