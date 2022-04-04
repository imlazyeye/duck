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
    // println!("Result for: \n{source}");
    println!("Local Fields:\n");
    // for name in scope.local_fields().iter() {
    //     let str = name.bright_black();
    //     match scope.lookup_type(&Identifier::lazy(name), &typewriter) {
    //         Ok(tpe) => {
    //             let whitespace = String::from_utf8(vec![b' '; 75 - str.len()]).unwrap();
    //             println!("{str}{whitespace}{}\n", Printer::tpe(&tpe).bright_cyan().bold());
    //         }
    //         Err(e) => errors.push(e),
    //     }
    // }

    // println!("\nSelf Fields:\n");
    // if let Some(self_object) = typewriter
    //     .find_term(&scope.self_marker)
    //     .and_then(|term| term.as_object())
    // {
    //     for (name, field) in self_object.fields() {
    //         let str = name.bright_black();
    //         let whitespace = String::from_utf8(vec![b' '; 75 - str.len()]).unwrap();
    //         println!(
    //             "{str}{whitespace}{}\n",
    //             Printer::tpe(&(field.clone()).into()).bright_cyan().bold()
    //         );
    //     }
    // }

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

pub fn get_var_type(source: &'static str, name: &'static str) -> Type {
    let typewriter = harness_typewriter(source).unwrap();
    typewriter.lookup_type(name).unwrap()
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
            assert_eq!(get_var_type($src, stringify!($var)), $should_be);
        }
    };
    ($name:ident, $src:expr, $($var:ident: $should_be:expr), * $(,)?) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            $(assert_eq!(get_var_type($src, stringify!($var)), $should_be);)*
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
