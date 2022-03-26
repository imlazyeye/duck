use super::*;
use hashbrown::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Term {
    Type(Type),
    Marker(Marker),
    App(App),
    Deref(Deref),
    Impl(Impl),
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
                App::Function(params, return_type, _) => Type::Function {
                    parameters: params.into_iter().map(|param| param.into()).collect(),
                    return_type: Box::new(return_type.as_ref().clone().into()),
                },
            },
            Term::Deref(deref) => match deref {
                Deref::Call { target, arguments } => Type::Generic {
                    term: Box::new(Term::Deref(Deref::Call { target, arguments })),
                },
                _ => unreachable!(),
            },
            Term::Impl(imp) => match imp {
                Impl::Fields(fields) => Type::Generic {
                    term: Box::new(Term::Impl(Impl::Fields(fields))),
                },
            },
        }
    }
}
