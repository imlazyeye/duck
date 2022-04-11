use super::*;
use colored::Colorize;
use hashbrown::HashMap;
use itertools::Itertools;
use parking_lot::Mutex;

#[macro_export]
macro_rules! array {
    ($ty:expr) => {
        Ty::Array(Box::new($ty))
    };
}

#[macro_export]
macro_rules! record {
    ($($var:ident: $should_be:expr), * $(,)?) => {
        crate::analyze::Ty::Record(crate::analyze::Record {
            fields: hashbrown::HashMap::from([
                $((
                    stringify!($var).to_string(),
                    Field::write($should_be, crate::parse::Location::default(), crate::analyze::Var::Scope(0))
                ), )*
            ]),
            state: State::Extendable,
        })
    };
}

#[macro_export]
macro_rules! function {
    (() => $return_type:expr) => {
        crate::analyze::Ty::Func(crate::analyze::Func::Def(crate::analyze::Def {
            binding: None,
            parameters: vec![],
            return_type: Box::new($return_type),
        }))
    };
    (($($arg:expr), * $(,)?) => $return_type:expr) => {
        crate::analyze::Ty::Func(crate::analyze::Func::Def(crate::analyze::Def {
            binding: None,
            parameters:  vec![$($arg)*],
            return_type: Box::new($return_type),
        }))
    };
}

lazy_static! {
    static ref PRINTER: Mutex<Printer> = Mutex::new(Printer {
        aliases: HashMap::default(),
        expr_strings: HashMap::default(),
        alias_characters: vec![
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
            'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
            'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'Ƃ', 'Ɔ', 'ƈ', 'Ɖ', 'Ƌ', 'Ǝ', 'Ə', 'Ɛ', 'Ɣ', 'ƕ', 'Ɩ',
            'Ɨ', 'ƚ', 'ƛ', 'Ɯ', 'ƞ', 'Ɵ', 'ơ', 'Ƣ', 'Ƥ', 'ƥ', 'ƨ', 'Ʃ', 'ƫ', 'Ƭ', 'Ʊ', 'Ʋ', 'ƶ', 'ƹ', 'ƾ', 'ƿ',
        ],
        iter: 0,
    });
}

pub struct Printer {
    aliases: HashMap<Var, char>,
    expr_strings: HashMap<Var, String>,
    alias_characters: Vec<char>,
    iter: usize,
}
impl Printer {
    pub fn flush() {
        let mut printer = PRINTER.lock();
        printer.aliases.clear();
        printer.expr_strings.clear();
        printer.iter = 0;
    }

    pub fn give_expr_alias(var: Var, name: String) {
        if !PRINTER.lock().aliases.contains_key(&var) {
            println!("{}        {}   :   {}", "ALIAS".bright_red(), Printer::var(&var), name);
            // PRINTER.lock().expr_strings.insert(var, name);
        }
    }

    #[must_use]
    pub fn var(var: &Var) -> String {
        let mut printer = PRINTER.lock();
        match *var {
            Var::Return => "<R>".into(),
            var => {
                if let Some(expr_string) = printer.expr_strings.get(&var) {
                    expr_string.clone()
                } else {
                    let entry = if let Some(entry) = printer.aliases.get(&var) {
                        entry.to_string()
                    } else {
                        let v = printer.alias_characters[printer.iter];
                        printer.iter = if printer.iter + 1 >= printer.alias_characters.len() {
                            0
                        } else {
                            printer.iter + 1
                        };
                        printer.aliases.insert(var, v);
                        v.to_string()
                    };
                    entry
                }
            }
        }
        .bright_black()
        .bold()
        .to_string()
    }

    #[must_use]
    pub fn ty(ty: &Ty) -> String {
        let s = match ty {
            Ty::Null => "<null>".into(),
            Ty::Any => "any".into(),
            Ty::Undefined => "undefined".into(),
            Ty::Noone => "noone".into(),
            Ty::Bool => "bool".into(),
            Ty::Real => "real".into(),
            Ty::Str => "string".into(),
            Ty::Array(inner) => format!("[{}]", Self::ty(inner)),
            Ty::Var(var) => Self::var(var),
            Ty::Record(record) => {
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
                            .map(|(name, field)| format!("{}: {}", name, Printer::ty(field.ty())))
                            .join(", ")
                    )
                }
            }
            Ty::Func(function) => match function {
                Func::Def(Def {
                    parameters,
                    return_type,
                    ..
                }) => format!(
                    "fn ({}) -> {}",
                    parameters.iter().map(Printer::ty).join(", "),
                    Printer::ty(return_type)
                ),
                Func::Call(Call {
                    parameters,
                    return_type,
                }) => format!(
                    "({}) -> {}",
                    parameters.iter().map(Printer::ty).join(", "),
                    Printer::ty(return_type)
                ),
            },
        };
        s.blue().bold().to_string()
    }

    #[must_use]
    pub fn ty_unification(a: &Ty, b: &Ty) -> String {
        format!(
            "{}      {}   ≟   {}",
            "UNIFY T".bright_yellow(),
            Printer::ty(a),
            Printer::ty(b),
        )
    }

    #[must_use]
    pub fn var_unification(var: &Var, ty: &Ty) -> String {
        format!(
            "{}      {}   ≟   {}",
            "UNIFY M".bright_yellow(),
            Printer::var(var),
            Printer::ty(ty),
        )
    }

    #[must_use]
    pub fn substitution(var: &Var, ty: &Ty) -> String {
        format!(
            "{}          {}   →   {}",
            "SUB".bright_green(),
            Printer::var(var),
            Printer::ty(ty),
        )
    }

    #[must_use]
    pub fn goal(goal: &Goal) -> String {
        match goal {
            Goal::Eq(var, ty) => format!(
                "{}          {}   =   {}",
                "CON".bright_magenta(),
                Self::var(var),
                Self::ty(ty),
            ),
        }
    }
}

#[macro_export]
macro_rules! duck_error {
    ($($arg:tt)*) => {
        Err(crate::duck_error_unwrapped!($($arg)*))
    };
}

#[macro_export]
macro_rules! duck_error_unwrapped {
    ($($arg:tt)*) => {
        codespan_reporting::diagnostic::Diagnostic::error().with_message(format!($($arg)*))
    };
}

#[macro_export]
macro_rules! duck_bug {
    ($($msg_arg:expr), * $(,)?) => {
        Err(codespan_reporting::diagnostic::Diagnostic::bug().with_message(format!($($msg_arg, )*)))
    };
}
