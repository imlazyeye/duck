use super::*;
use hashbrown::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Generic {
        term: Box<Term>,
    },
    Any,
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
        self_parameter: Option<Box<Type>>,
        parameters: Vec<Type>,
        return_type: Box<Type>,
    },
}

#[macro_export]
macro_rules! new_array {
    ($tpe:expr) => {
        Type::Array {
            member_type: Box::new($tpe),
        }
    };
}

#[macro_export]
macro_rules! new_struct {
    ($($var:ident: $should_be:expr), * $(,)?) => {
        Type::Struct {
            fields: hashbrown::HashMap::from([
                $((stringify!($var).to_string(), $should_be), )*
            ])
        }
    }
}

#[macro_export]
macro_rules! new_function {
    (() => $return_type:expr) => {
        Type::Function {
            self_parameter: None,
            parameters: vec![],
            return_type: Box::new($return_type),
        }
    };
    ((self: $self_param:expr) => $return_type:expr) => {
        Type::Function {
            self_parameter: Some(Box::new($self_param)),
            parameters: vec![],
            return_type: Box::new($return_type),
        }
    };
    ((self: $self_param:expr, $($arg:expr), * $(,)?) => $return_type:expr) => {
        Type::Function {
            self_parameter: Some(Box::new($self_param)),
            parameters: vec![$($arg)*],
            return_type: Box::new($return_type),
        }
    };
    (($($arg:expr), * $(,)?) => $return_type:expr) => {
        Type::Function {
            self_parameter: None,
            parameters: vec![$($arg)*],
            return_type: Box::new($return_type),
        }
    };
}
