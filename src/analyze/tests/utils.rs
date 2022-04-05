use crate::{analyze::*, parse::*};

pub struct TestTypeWriter(Typewriter);
impl std::ops::Deref for TestTypeWriter {
    type Target = Typewriter;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Drop for TestTypeWriter {
    fn drop(&mut self) {
        Printer::flush()
    }
}

pub fn harness_typewriter(source: &str) -> Result<TestTypeWriter, Vec<TypeError>> {
    let source = Box::leak(Box::new(source.to_string()));
    let parser = Parser::new(source, 0);
    let mut errors = vec![];
    let mut typewriter = Typewriter::default();
    let mut ast = parser.into_ast().unwrap();
    if let Err(e) = &mut typewriter.process_statements(ast.stmts_mut()) {
        errors.append(e);
    }
    for (name, _) in typewriter
        .active_self()
        .fields
        .iter()
        .chain(typewriter.locals().fields.iter())
    {
        let _ = typewriter.lookup_type(name).map_err(|e| errors.push(e));
    }

    if errors.is_empty() {
        Ok(TestTypeWriter(typewriter))
    } else {
        Err(errors)
    }
}

pub fn get_type(source: &'static str) -> Type {
    let source = Box::leak(Box::new(format!("var a = {source}")));
    let typewriter = harness_typewriter(source).unwrap();
    typewriter.lookup_type("a").unwrap()
}

pub fn assert_var_type(source: &'static str, name: &'static str, should_be: Type) {
    let typewriter = harness_typewriter(source).unwrap();
    let tpe = typewriter.lookup_type(name).unwrap();
    assert_eq!(
        tpe,
        should_be,
        "`{name}` should be {}, but it is {}",
        Printer::tpe(&should_be, &typewriter),
        Printer::tpe(&tpe, &typewriter),
    );
}

#[macro_export]
macro_rules! test_expr_type {
    ($name:ident, $src:expr => $should_be:expr) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            assert_eq!(get_type($src), $should_be);
        }
    };
    ($name:ident, $($src:expr => $should_be:expr), * $(,)?) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            $(assert_eq!(get_type($src), $should_be);)*
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
            assert!(harness_typewriter($src).is_err());
        }
    };
}
