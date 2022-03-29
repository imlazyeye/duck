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
                    parameters,
                    return_type,
                    ..
                } => Type::Function {
                    parameters: parameters.into_iter().map(|param| param.into()).collect(),
                    return_type: Box::new(return_type.as_ref().clone().into()),
                },
                App::Call { .. } => unreachable!(),
            },
            Term::Deref(deref) => unreachable!("tried to convert deref to type: {}", Printer::deref(&deref)),
            Term::Trait(trt) => match trt {
                Trait::FieldOps(ops) => Type::Struct {
                    fields: ops
                        .into_iter()
                        .map(|(name, op)| (name, op.term().clone().into()))
                        .collect(),
                },
                Trait::Derive(_) => todo!(),
                Trait::Callable(args, return_type) => Type::Function {
                    parameters: args.into_iter().map(|v| v.into()).collect(),
                    return_type: Box::new(return_type.as_ref().clone().into()),
                },
            },
        }
    }
}
