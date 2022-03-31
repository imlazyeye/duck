use super::*;
use hashbrown::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Term {
    Type(Type),
    Marker(Marker),
    App(App),
    Trait(Trait),
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
                    fields: object
                        .into_iter()
                        .map(|(n, term)| (n, term.term().clone().into()))
                        .collect(),
                },
                App::Function {
                    self_fields,
                    parameters,
                    return_type,
                } => Type::Function {
                    self_fields: self_fields.map(|self_fields| {
                        Box::new(Type::Struct {
                            fields: self_fields
                                .into_iter()
                                .map(|(n, op)| (n, op.term().clone().into()))
                                .collect(),
                        })
                    }),
                    parameters: parameters.into_iter().map(|param| param.into()).collect(),
                    return_type: Box::new(return_type.as_ref().clone().into()),
                },
            },
            Term::Trait(trt) => match trt {
                Trait::FieldOp(name, op) => Type::Struct {
                    fields: HashMap::from([(name, op.term().clone().into())]),
                },
                Trait::Callable {
                    calling_scope,
                    arguments,
                    expected_return,
                    ..
                } => Type::Function {
                    self_fields: if calling_scope.is_empty() {
                        None
                    } else {
                        Some(Box::new(Type::Struct {
                            fields: calling_scope
                                .into_iter()
                                .map(|(n, op)| (n, op.term().clone().into()))
                                .collect(),
                        }))
                    },
                    parameters: arguments.into_iter().map(|v| v.into()).collect(),
                    return_type: Box::new(expected_return.as_ref().clone().into()),
                },
            },
        }
    }
}
