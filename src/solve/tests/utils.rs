use crate::{parse::*, solve::*};
use parking_lot::Mutex;

lazy_static! {
    pub(super) static ref SOLVER: Mutex<Solver> = Mutex::new(create_test_solver());
}

fn create_test_solver() -> Solver {
    let mut solver = Solver::default();
    solver.adts.remove(&AdtId::GLOBAL);
    solver
}

pub fn harness_solver(source: &str) -> Result<Solver, Vec<TypeError>> {
    let mut solver = Solver::default();
    let source = Box::leak(Box::new(source.to_string()));
    let parser = Parser::new(source, 0);
    let mut errors = vec![];
    let mut ast = parser.into_ast().unwrap();
    if let Err(e) = &mut solver.process_statements(ast.stmts_mut()) {
        errors.append(e);
    }
    if let Err(e) = &mut solver.emit_uninitialized_variable_errors() {
        errors.append(e);
    }
    if errors.is_empty() { Ok(solver) } else { Err(errors) }
}

pub fn assert_var_type(name: &'static str, should_be: Ty, solver: &mut Solver) {
    let ty = solver.resolve_name(name).unwrap();
    assert!(
        ty.loose_eq(&should_be, solver),
        "{name} was the wrong type! Expected {}, got {}",
        Printer::ty(&should_be, solver),
        Printer::ty(&ty, solver)
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
                        .map_or(false, |other_field| field.ty.loose_eq(other_field, solver))
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
                let source = Box::leak(Box::new(format!("var a = {}", $src)));
                let mut solver = harness_solver(source).unwrap();
                let should_be = $should_be;
                solver.adts.extend(SOLVER.lock().adts.clone());
                assert_var_type("a", should_be, &mut solver);
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
            let mut solver = harness_solver($src).unwrap();
            $(
                let should_be = $should_be;
                solver.adts.extend(SOLVER.lock().adts.clone());
                assert_var_type(stringify!($var), should_be, &mut solver);
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
            harness_solver($src).unwrap();
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
