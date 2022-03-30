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
                App::Object(fields) => {
                    let mut tpe_fields = HashMap::new();
                    for (name, term) in fields {
                        tpe_fields.insert(name, term.into());
                    }
                    Type::Struct { fields: tpe_fields }
                }
                App::Function {
                    parameters,
                    return_type,
                    self_parameter,
                } => Type::Function {
                    self_parameter: self_parameter.and_then(|v| match v.as_ref() {
                        Term::Trait(Trait::FieldOps(ops)) => {
                            if ops.is_empty() {
                                None
                            } else {
                                Some(Box::new(v.as_ref().clone().into()))
                            }
                        }
                        _ => unreachable!(),
                    }),
                    parameters: parameters.into_iter().map(|param| param.into()).collect(),
                    return_type: Box::new(return_type.as_ref().clone().into()),
                },
            },
            Term::Trait(trt) => match trt {
                Trait::FieldOps(ops) => Type::Struct {
                    fields: ops
                        .into_iter()
                        .map(|(name, op)| (name, op.term().clone().into()))
                        .collect(),
                },
                Trait::Derive(_) => todo!(),
                Trait::Callable {
                    calling_scope,
                    arguments,
                    expected_return,
                } => Type::Function {
                    self_parameter: Some(Box::new(calling_scope.as_ref().clone().into())),
                    parameters: arguments.into_iter().map(|v| v.into()).collect(),
                    return_type: Box::new(expected_return.as_ref().clone().into()),
                },
            },
        }
    }
}