use super::*;
use colored::Colorize;
use hashbrown::HashMap;
use itertools::Itertools;
use parking_lot::Mutex;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
pub struct Marker(pub u64);
impl Marker {
    pub const RETURN: Self = Marker(u64::MAX);
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
    pub fn term(term: &Term) -> String {
        match term {
            Term::Type(tpe) => Self::tpe(tpe),
            Term::Marker(marker) => Self::marker(marker),
            Term::App(app) => Self::app(app),
            Term::Trait(trt) => Self::trt(trt),
        }
    }

    #[must_use]
    pub fn tpe(tpe: &Type) -> String {
        let s = match tpe {
            Type::Generic { term } => format!("T: {}", Self::term(term)),
            Type::Any => "any".into(),
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
                self_parameter,
                parameters,
                return_type,
            } => format!(
                "fn ({}) -> {}",
                [self_parameter.as_ref().map(|v| format!("self<{}>", Printer::tpe(v)))]
                    .into_iter()
                    .flatten()
                    .chain(parameters.iter().map(Self::tpe))
                    .join(", "),
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
            App::Function {
                self_parameter,
                parameters,
                return_type,
                ..
            } => format!(
                "({}) → {}",
                [self_parameter.as_ref().map(|v| format!("self<{}>", Printer::term(v)))]
                    .into_iter()
                    .flatten()
                    .chain(parameters.iter().map(Self::term))
                    .join(", "),
                Self::term(return_type),
            ),
        }
    }

    #[must_use]
    pub fn trt(trt: &Trait) -> String {
        match trt {
            Trait::FieldOps(field_ops) => {
                if field_ops.is_empty() {
                    "*".into()
                } else {
                    field_ops
                        .iter()
                        .map(|(name, op)| match op.as_ref() {
                            FieldOp::Readable(term) => format!("Readable<{name}: {}>", Self::term(term)),
                            FieldOp::Writable(term) => format!("Writable<{name}: {}>", Self::term(term)),
                        })
                        .join(", ")
                }
            }
            Trait::Derive(term) => format!("Derive<{}>", Self::term(term)),
            Trait::Callable {
                calling_scope,
                arguments,
                expected_return,
            } => format!(
                "Callable<({}) -> {}>",
                [format!("self<{}>", Printer::term(calling_scope.as_ref()))]
                    .into_iter()
                    .chain(arguments.iter().map(Self::term))
                    .join(", "),
                Self::term(expected_return)
            ),
        }
    }

    #[must_use]
    pub fn term_unification(a: &Term, b: &Term) -> String {
        format!(
            "{}      {}   ≟   {}",
            "UNIFY T".bright_yellow(),
            Printer::term(a),
            Printer::term(b),
        )
    }

    #[must_use]
    pub fn marker_unification(marker: &Marker, term: &Term) -> String {
        format!(
            "{}      {}   ≟   {}",
            "UNIFY M".bright_yellow(),
            Printer::marker(marker),
            Printer::term(term),
        )
    }

    #[must_use]
    pub fn term_impl(term: &Term, trt: &Trait) -> String {
        format!(
            "{}         {}   ⊇   {}",
            "IMPL".bright_cyan(),
            Printer::term(term),
            Printer::trt(trt),
        )
    }

    #[must_use]
    pub fn substitution(marker: &Marker, term: &Term) -> String {
        format!(
            "{}          {}   →   {}",
            "SUB".bright_green(),
            Printer::marker(marker),
            Printer::term(term),
        )
    }

    #[must_use]
    pub fn constraint(constraint: &Constraint) -> String {
        match constraint {
            Constraint::Eq(marker, term) => format!(
                "{}          {}   =   {}",
                "CON".bright_magenta(),
                Self::marker(marker),
                Self::term(term),
            ),
        }
    }
}
