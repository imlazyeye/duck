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
    Str,
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
        self_fields: Option<Box<Type>>,
        parameters: Vec<Type>,
        return_type: Box<Type>,
    },
}

impl From<Term> for Type {
    fn from(term: Term) -> Self {
        match term {
            Term::Type(tpe) => tpe,
            Term::Marker(marker) => Type::Generic {
                term: Box::new(Term::Marker(marker)),
            },
            Term::App(app) => match app {
                App::Array(member_type) => Type::Array {
                    member_type: Box::new(Type::from(member_type.as_ref().to_owned())),
                },
                App::Object(object) => Type::Struct {
                    fields: match object {
                        Object::Concrete(fields) => fields.into_iter().map(|(n, term)| (n, term.into())).collect(),
                        Object::Inferred(fields) => fields
                            .into_iter()
                            .map(|(n, term)| (n, term.term().clone().into()))
                            .collect(),
                    },
                },
                App::Function(Function {
                    self_fields,
                    parameters,
                    return_type,
                    ..
                }) => Type::Function {
                    self_fields: self_fields.map(|self_fields| Box::new(self_fields.into())),
                    parameters: parameters.into_iter().map(|param| param.into()).collect(),
                    return_type: Box::new(return_type.as_ref().clone().into()),
                },
                App::Union(terms) => Type::Union {
                    types: terms.into_iter().map(|v| v.into()).collect(),
                },
            },
            Term::Trait(trt) => match trt {
                Trait::FieldOp(name, op) => Type::Struct {
                    fields: HashMap::from([(name, op.term().clone().into())]),
                },
                Trait::Callable {
                    arguments,
                    expected_return,
                    ..
                } => Type::Function {
                    self_fields: None,
                    parameters: arguments.into_iter().map(|v| v.into()).collect(),
                    return_type: Box::new(expected_return.as_ref().clone().into()),
                },
            },
        }
    }
}

impl From<Object> for Type {
    fn from(obj: Object) -> Self {
        match obj {
            Object::Concrete(fields) => Type::Struct {
                fields: fields.into_iter().map(|(name, term)| (name, term.into())).collect(),
            },
            Object::Inferred(fields) => Type::Struct {
                fields: fields
                    .into_iter()
                    .map(|(name, op)| {
                        (
                            name,
                            match op {
                                FieldOp::Readable(term) | FieldOp::Writable(term) => term.into(),
                            },
                        )
                    })
                    .collect(),
            },
        }
    }
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
            self_fields: None,
            parameters: vec![],
            return_type: Box::new($return_type),
        }
    };
    ((self: $self_param:expr) => $return_type:expr) => {
        Type::Function {
            self_fields: Some(Box::new($self_param)),
            parameters: vec![],
            return_type: Box::new($return_type),
        }
    };
    ((self: $self_param:expr, $($arg:expr), * $(,)?) => $return_type:expr) => {
        Type::Function {
            self_fields: Some(Box::new($self_param)),
            parameters: vec![$($arg)*],
            return_type: Box::new($return_type),
        }
    };
    (($($arg:expr), * $(,)?) => $return_type:expr) => {
        Type::Function {
            self_fields: None,
            parameters: vec![$($arg)*],
            return_type: Box::new($return_type),
        }
    };
}

#[macro_export]
macro_rules! new_union {
   ($($tpe:expr),*) => (Type::Union { types: vec![$($tpe),*] })
}
