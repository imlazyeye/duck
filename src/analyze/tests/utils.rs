use crate::{analyze::*, parse::*};

pub struct TestTypeWriter(Solver);
impl std::ops::Deref for TestTypeWriter {
    type Target = Solver;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for TestTypeWriter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Drop for TestTypeWriter {
    fn drop(&mut self) {
        Printer::flush()
    }
}

pub fn harness_solver(source: &str) -> Result<TestTypeWriter, Vec<TypeError>> {
    let source = Box::leak(Box::new(source.to_string()));
    let parser = Parser::new(source, 0);
    let mut errors = vec![];
    let mut solver = Solver::default();
    let mut ast = parser.into_ast().unwrap();
    if let Err(e) = &mut solver.process_statements(ast.stmts_mut()) {
        errors.append(e);
    }
    for (name, _) in solver
        .self_scope()
        .fields
        .iter()
        .chain(solver.local_scope().fields.iter())
    {
        let _ = solver.resolve_name(name).map_err(|e| errors.push(e));
    }

    if errors.is_empty() {
        Ok(TestTypeWriter(solver))
    } else {
        Err(errors)
    }
}

pub fn get_type(source: &'static str) -> Ty {
    let source = Box::leak(Box::new(format!("var a = {source}")));
    match harness_solver(source) {
        Ok(solver) => solver.resolve_name("a").unwrap(),
        Err(e) => panic!("{}", e[0].message),
    }
}

pub fn assert_var_type(source: &'static str, name: &'static str, should_be: Ty) {
    match harness_solver(source) {
        Ok(solver) => {
            let ty = solver.resolve_name(name).unwrap();
            assert!(
                ty.loose_eq(&should_be),
                "{name} was the wrong type! Expected {}, got {}",
                Printer::ty(&should_be),
                Printer::ty(&ty)
            );
        }
        Err(e) => panic!("{}", e[0].message),
    }
}

impl Ty {
    pub fn loose_eq(&self, other: &Ty) -> bool {
        match (self, other) {
            (Ty::Record(record), Ty::Record(other_record)) => record.fields.iter().all(|(name, field)| {
                other_record
                    .get(name)
                    .map_or(false, |other_field| field.ty.loose_eq(&other_field.ty))
            }),
            (Ty::Array(member), Ty::Array(other_member)) => member.loose_eq(other_member),
            (Ty::Record(_), _) => false,
            (Ty::Func(function), Ty::Func(other_function)) => {
                function.return_type().loose_eq(other_function.return_type())
                    && function.parameters().iter().enumerate().all(|(i, param)| {
                        other_function
                            .parameters()
                            .get(i)
                            .map_or(false, |other_param| param.loose_eq(other_param))
                    })
            }
            (Ty::Func(_), _) => false,
            _ => self == other,
        }
    }
}

#[macro_export]
macro_rules! test_expr_type {
    ($name:ident, $src:expr => $should_be:expr) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            let ty = get_type($src);
            assert!(
                ty.loose_eq(&$should_be),
                "Wrong type! Expected {}, got {}",
                Printer::ty(&$should_be),
                Printer::ty(&ty)
            );
        }
    };
    ($name:ident, $($src:expr => $should_be:expr), * $(,)?) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            $(assert!($should_be.eq(&get_type($src)));)*
        }
    };
}

#[macro_export]
macro_rules! test_var_type {
    ($name:ident, $src:expr, $var:ident: $should_be:expr) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            assert_var_type($src, stringify!($var), $should_be);
        }
    };
    ($name:ident, $src:expr, $($var:ident: $should_be:expr), * $(,)?) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            $(assert_var_type($src, stringify!($var), $should_be);)*
        }
    };
}

#[macro_export]
macro_rules! test_failure {
    ($name:ident, $src:expr) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            assert!(harness_solver($src).is_err());
        }
    };
}

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
                    Field {
                        location: crate::parse::Location::default(),
                        ty: $should_be,
                        op: crate::analyze::RecordOp::Write,
                    }
                ), )*
            ]),
            state: State::Concrete,
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

#[macro_export]
macro_rules! enum_ty {
    ($($member:expr), * $(,)?) => {
        crate::analyze::Ty::Enum(vec![$(stringify!($member).to_string(),)*])
    };
}
