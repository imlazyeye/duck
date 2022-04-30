use crate::{parse::*, solve::*};
use parking_lot::Mutex;
use pretty_assertions::assert_eq;

lazy_static! {
    pub(super) static ref SOLVER: Mutex<Solver> = {
        let mut solver = Solver::default();
        solver.adts.remove(&AdtId::GLOBAL);
        Mutex::new(solver)
    };
}

pub fn harness_solver(source: &str, solver: &mut Solver) -> Result<(), TypeError> {
    let source = Box::leak(Box::new(source.to_string()));
    let parser = Parser::new(source, 0);
    let mut ast = parser.into_ast().unwrap();
    solver.process_statements(ast.stmts_mut())?;
    solver.emit_uninitialized_variable_errors()?;
    Ok(())
}

pub fn test_type(src: &'static str, should_be: Ty, solver: &mut Solver) {
    let source = Box::leak(Box::new(format!("var a = {}", src)));
    harness_solver(source, solver).unwrap();
    let field = solver
        .get_adt(solver.local_id())
        .get("a")
        .or_else(|| solver.get_adt(AdtId::GLOBAL).get("a"))
        .or_else(|| solver.get_adt(solver.self_id()).get("a"))
        .unwrap();
    let mut ty = field.clone();
    solver.normalize(&mut ty);
    if !ty.loose_eq(&should_be, solver) {
        let lhs = Printer::ty(&should_be, solver);
        let rhs = Printer::ty(&ty, solver);
        assert_eq!(lhs, rhs, "\n\n{source}");
        panic!(); // just to be sure
    }
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
macro_rules! test_type {
    ($name:ident, $($src:expr => $should_be:expr), * $(,)?) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            let mut solver = Solver::default();
            $({
                let should_be = $should_be;
                solver.adts.extend(SOLVER.lock().adts.clone());
                test_type($src, should_be, &mut solver);
            })*
        }
    };
    ($name:ident, $preamble:expr, $($src:expr => $should_be:expr), * $(,)?) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            let mut solver = Solver::default();
            harness_solver($preamble, &mut solver).unwrap();
            $({
                let should_be = $should_be;
                solver.adts.extend(SOLVER.lock().adts.clone());
                test_type($src, should_be, &mut solver);
            })*
        }
    };
}

#[macro_export]
macro_rules! test_success {
    ($name:ident, $src:expr) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            harness_solver($src, &mut Solver::default()).unwrap();
        }
    };
}

#[macro_export]
macro_rules! test_failure {
    ($name:ident, $src:expr) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            assert!(harness_solver($src, &mut Solver::default()).is_err());
        }
    };
}
