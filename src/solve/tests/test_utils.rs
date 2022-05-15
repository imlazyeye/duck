use crate::{parse::*, solve::*};
use pretty_assertions::assert_eq;

pub fn harness_session(source: &str, session: &mut Session) -> Result<(), TypeError> {
    let source = Box::leak(Box::new(source.to_string()));
    let parser = Parser::new(source, 0);
    let mut ast = parser.into_ast().unwrap();
    session.process_statements(ast.stmts_mut())?;
    // session.emit_uninitialized_variable_errors()?;
    Ok(())
}

pub fn test_type(should_be: Ty, session: &mut Session, preamble: Option<&str>, src: &'static str) {
    let source = Box::leak(Box::new(format!("var TEST_VALUE = {}", src)));
    harness_session(source, session).unwrap();
    let ty = session.resolve_name("TEST_VALUE").unwrap();
    if !should_be.loose_eq(&ty) {
        let lhs = Printer::ty(&should_be);
        let rhs = Printer::ty(&ty);
        if let Some(preamble) = preamble {
            println!("\n-- Preamble --\n{preamble}");
        }
        println!("\n-- Source -- \n{source}");
        assert_eq!(lhs, rhs);
        panic!(); // just to be sure
    }
}

impl Ty {
    pub fn loose_eq(&self, other: &Ty) -> bool {
        match (self, other) {
            (Ty::Adt(adt), Ty::Adt(other_adt)) => adt.fields.iter().all(|(name, field)| {
                other_adt.ty(name).map_or(false, |other_field| {
                    field.value.ty().map_or(false, |ty| ty.loose_eq(other_field))
                })
            }),
            (Ty::Array(member), Ty::Array(other_member)) => member.loose_eq(other_member),
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
macro_rules! global_test {
    ($name:ident, $($src:expr => $should_be:expr), * $(,)?) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            let mut subs = crate::solve::Subs::default();
            let mut session = crate::solve::Session::new(&mut subs);
            $({
                crate::solve::tests::test_utils::test_type($should_be, &mut session, None, $src);
            })*
        }
    };
    ($name:ident, $preamble:expr, $($src:expr => $should_be:expr), * $(,)?) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            let mut subs = crate::solve::Subs::default();
            let mut session = crate::solve::Session::new(&mut subs);
            crate::solve::tests::test_utils::harness_session($preamble, &mut session).unwrap();
            $({
                crate::solve::tests::test_utils::test_type($should_be, &mut session, Some($preamble), $src);
            })*
        }
    };
}

#[macro_export]
macro_rules! identity_test {
    ($name:ident, $($src:expr => $should_be:expr), * $(,)?) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            let mut subs = crate::solve::Subs::default();
            let mut session = crate::solve::Session::new(&mut subs);
            let var = crate::solve::Var::Generated(rand::random());
            let adt = crate::solve::Adt::new(AdtState::Extendable, vec![]);
            session.subs.register(var, crate::solve::Ty::Adt(adt));
            session.push_identity(var);
            $({
                crate::solve::tests::test_utils::test_type($should_be, &mut session, None, $src);
            })*
        }
    };
    ($name:ident, $preamble:expr, $($src:expr => $should_be:expr), * $(,)?) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            let mut subs = crate::solve::Subs::default();
            let mut session = crate::solve::Session::new(&mut subs);
            let var = crate::solve::Var::Generated(rand::random());
            let adt = crate::solve::Adt::new(AdtState::Extendable, vec![]);
            session.subs.register(var, crate::solve::Ty::Adt(adt)).unwrap();
            session.push_identity(var);
            crate::solve::tests::test_utils::harness_session($preamble, &mut session).unwrap();
            $({
                crate::solve::tests::test_utils::test_type($should_be, &mut session, Some($preamble), $src);
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
            let mut subs = crate::solve::Subs::default();
            let mut session = Session::new(&mut subs);
            harness_session($src, &mut session).unwrap();
        }
    };
}

#[macro_export]
macro_rules! test_failure {
    ($name:ident, $src:expr) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            let mut subs = crate::solve::Subs::default();
            let mut session = Session::new(&mut subs);
            assert!(harness_session($src, &mut session).is_err());
        }
    };
}
