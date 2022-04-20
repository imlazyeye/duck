use crate::{parse::*, solve::*};
use parking_lot::Mutex;

pub struct TestToken;
impl Drop for TestToken {
    fn drop(&mut self) {
        *SOLVER.lock() = Solver::default();
        Printer::flush()
    }
}

lazy_static! {
    pub(super) static ref SOLVER: Mutex<Solver> = Mutex::new(Solver::default());
}

pub fn harness_solver(source: &str) -> Result<TestToken, Vec<TypeError>> {
    let source = Box::leak(Box::new(source.to_string()));
    let parser = Parser::new(source, 0);
    let mut errors = vec![];
    let mut ast = parser.into_ast().unwrap();
    let mut solver = SOLVER.lock();
    if let Err(e) = &mut solver.process_statements(ast.stmts_mut()) {
        errors.append(e);
    }
    // if let Err(e) = &mut solver.check_promises() {
    //     errors.append(e)
    // }
    if errors.is_empty() { Ok(TestToken) } else { Err(errors) }
}

pub fn get_type(source: &'static str) -> (Ty, TestToken) {
    let source = Box::leak(Box::new(format!("var a = {source}")));
    match harness_solver(source) {
        Ok(token) => {
            let ty = SOLVER.lock().resolve_name("a").unwrap();
            // solver.check_promises().unwrap();
            (ty, token)
        }
        Err(e) => panic!("{}", e[0].message),
    }
}

pub fn assert_var_type(name: &'static str, should_be: Ty) {
    let mut solver = SOLVER.lock();
    let ty = solver.resolve_name(name).unwrap();
    assert!(
        ty.loose_eq(&should_be, &solver),
        "{name} was the wrong type! Expected {}, got {}",
        Printer::ty(&should_be, &solver),
        Printer::ty(&ty, &solver)
    );
}

impl Ty {
    pub fn loose_eq(&self, other: &Ty, solver: &Solver) -> bool {
        match (self, other) {
            (Ty::Adt(adt_id), Ty::Adt(other_adt_id)) => {
                let adt = solver.get_adt(*adt_id);
                let other_adt = solver.adts.get(other_adt_id).unwrap();
                adt.fields.iter().all(|(name, field)| {
                    other_adt
                        .get(name)
                        .map_or(false, |other_field| field.ty.loose_eq(&other_field.ty, solver))
                })
            }
            (Ty::Array(member), Ty::Array(other_member)) => member.loose_eq(other_member, solver),
            (Ty::Adt(_), _) => false,
            (Ty::Func(function), Ty::Func(other_function)) => {
                function.return_type().loose_eq(other_function.return_type(), solver)
                    && function.parameters().iter().enumerate().all(|(i, param)| {
                        other_function
                            .parameters()
                            .get(i)
                            .map_or(false, |other_param| param.loose_eq(other_param, solver))
                    })
            }
            (Ty::Func(_), _) => false,
            _ => self == other,
        }
    }
}

#[macro_export]
macro_rules! test_expr_type {
    ($name:ident, $($src:expr => $should_be:expr), * $(,)?) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            $({
                let should_be = $should_be;
                let (ty, _) = &get_type($src);
                let solver = SOLVER.lock();
                assert!(
                    ty.loose_eq(&should_be, &solver),
                    "Expected {}, got {}",
                    Printer::ty(&should_be, &solver),
                    Printer::ty(&ty, &solver)
                );
            })*
        }
    };
}

#[macro_export]
macro_rules! test_var_type {
    ($name:ident, $src:expr, $($var:ident: $should_be:expr), * $(,)?) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            let _token = harness_solver($src).unwrap();
            $(
                let should_be = $should_be;
                assert_var_type(stringify!($var), should_be);
            )*
        }
    };
}

#[macro_export]
macro_rules! test_success {
    ($name:ident, $src:expr) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            assert!(harness_solver($src).is_ok());
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
