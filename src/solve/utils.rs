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
macro_rules! option {
    ($ty:expr) => {
        Ty::Option(Box::new($ty))
    };
}

#[macro_export]
macro_rules! adt {
    ($($var:ident: $should_be:expr), * $(,)?) => {
        {
            #[allow(unused_mut)]
            let mut fields = vec![];
            $(
                let should_be = $should_be;
                fields.push((
                    crate::parse::Identifier::lazy(stringify!($var).to_string()),
                    should_be,
                ));
            )*
            Ty::Adt(crate::solve::Adt::new(AdtState::Extendable, fields))
        }
    };
}

#[macro_export]
macro_rules! function {
    (() => $return_type:expr) => {
        crate::solve::Ty::Func(crate::solve::Func::Def(crate::solve::Def {
            binding: None,
            parameters: vec![],
            minimum_arguments: 0,
            return_type: Box::new($return_type),
        }))
    };
    (($($arg:expr), * $(,)?) => $return_type:expr) => {
        crate::solve::Ty::Func(crate::solve::Func::Def(crate::solve::Def {
            binding: None,
            parameters:  vec![$($arg, )*],
            minimum_arguments: 0, // we don't check this, so we're just gonna yolo it
            return_type: Box::new($return_type),
        }))
    };
}

#[macro_export]
macro_rules! var {
    () => {
        Ty::Var(crate::solve::Var::Generated(rand::random()))
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

    #[must_use]
    pub fn var(var: &Var) -> String {
        let mut printer = PRINTER.lock();
        let var = *var;
        if let Some(expr_string) = printer.expr_strings.get(&var) {
            expr_string.clone()
        } else if var == Var::Return {
            "RET".to_string()
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

    #[must_use]
    pub fn ty(ty: &Ty) -> String {
        match ty {
            Ty::Identity => "identity".into(),
            Ty::Any => "any".into(),
            Ty::Undefined => "undefined".into(),
            Ty::Noone => "noone".into(),
            Ty::Bool => "bool".into(),
            Ty::Real => "real".into(),
            Ty::Str => "string".into(),
            Ty::Array(inner) => format!("[{}]", Printer::ty(inner)),
            Ty::Var(var) => Self::var(var),
            Ty::Adt(adt) => {
                if adt.fields.is_empty() {
                    "{}".into()
                } else {
                    format!(
                        "{{ {} }}",
                        adt.fields
                            .iter()
                            .sorted_by_key(|(n, _)| n.as_str())
                            .map(|(name, field)| {
                                format!(
                                    "{}: {}",
                                    name,
                                    if let Some(field_ty) = field.value.ty() {
                                        if field_ty.contains(ty) {
                                            "<cycle>".into()
                                        } else {
                                            Printer::ty(field_ty)
                                        }
                                    } else {
                                        "<null>".into()
                                    }
                                )
                            })
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
            Ty::Option(ty) => {
                format!("Option<{}>", Printer::ty(ty.as_ref()))
            }
        }
    }

    #[must_use]
    pub fn stmt(a: &crate::parse::Stmt) -> String {
        format!("{}         {a}", "STMT".bright_white())
    }

    #[must_use]
    pub fn query(a: &crate::parse::Expr) -> String {
        format!(
            "{}        {a}: {}",
            "QUERY".bright_red(),
            Printer::var(&Var::Expr(a.id())).bold().bright_black()
        )
    }

    #[must_use]
    pub fn ty_unification(a: &Ty, b: &Ty) -> String {
        format!(
            "{}      {}   ≟   {}",
            "UNIFY T".bright_yellow(),
            Printer::ty(a).blue().bold(),
            Printer::ty(b).blue().bold(),
        )
    }

    #[must_use]
    pub fn var_unification(var: &Var, ty: &Ty) -> String {
        format!(
            "{}      {}   ≟   {}",
            "UNIFY M".bright_yellow(),
            Printer::var(var).bright_black().bold(),
            Printer::ty(ty).blue().bold(),
        )
    }

    #[must_use]
    pub fn substitution(var: &Var, ty: &Ty) -> String {
        format!(
            "{}          {}   →   {}",
            "SUB".bright_green(),
            Printer::var(var).bright_black().bold(),
            Printer::ty(ty).blue().bold(),
        )
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
