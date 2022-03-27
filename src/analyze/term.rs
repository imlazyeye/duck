use super::*;
use hashbrown::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Term {
    Type(Type),
    Marker(Marker),
    App(App),
    Deref(Deref),
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
                    self_parameter,
                    parameters,
                    return_type,
                    ..
                } => Type::Function {
                    parameters: parameters.into_iter().map(|(_, param)| param.into()).collect(),
                    return_type: Box::new(return_type.as_ref().clone().into()),
                },
            },
            Term::Deref(deref) => match deref {
                Deref::Call {
                    target,
                    arguments,
                    uses_new,
                } => Type::Generic {
                    term: Box::new(Term::Deref(Deref::Call {
                        target,
                        arguments,
                        uses_new,
                    })),
                },
                _ => unreachable!(),
            },
            Term::Trait(trt) => match trt {
                Trait::Contains(fields) => Type::Generic {
                    term: Box::new(Term::Trait(Trait::Contains(fields))),
                },
            },
        }
    }
}
