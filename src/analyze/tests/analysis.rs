use crate::{
    analyze::{App, Page, Term, Type, TypeWriter},
    parse::*,
};
use colored::Colorize;
use hashbrown::HashMap;
use pretty_assertions::assert_eq;

fn harness_type_ast(source: &'static str, pairs: impl Into<Vec<(&'static str, Type)>>) {
    for (name, expected_type) in pairs.into() {
        assert_eq!(get_var_type(source, name), expected_type, "{} was wrong value!", name);
    }
}

fn harness_type_expr(source: &'static str, expected_tpe: Type) {
    assert_eq!(get_type(source), expected_tpe,);
}

fn get_type(source: &'static str) -> Type {
    let source = Box::leak(Box::new(format!("var a = {source}")));
    let page = harness_typewriter(source);
    page.read_field(&Identifier::lazy("a")).unwrap()
}

fn get_var_type(source: &'static str, name: &'static str) -> Type {
    let page = harness_typewriter(source);
    page.read_field(&Identifier::lazy(name)).unwrap()
}

fn get_function(source: &'static str) -> (Type, Vec<Type>, Box<Type>) {
    let tpe = get_type(source);
    match tpe.clone() {
        Type::Function {
            parameters,
            return_type,
        } => (tpe, parameters, return_type),
        _ => panic!("Expected a function"),
    }
}

fn harness_typewriter(source: &str) -> Page {
    let source = Box::leak(Box::new(source.to_string()));
    let parser = Parser::new(source, 0);
    let mut typewriter = TypeWriter::default();
    let mut ast = parser.into_ast().unwrap();
    let page = typewriter.write_types(&mut ast);
    println!("Result for: \n{source}");
    for (name, _) in page.scope.fields.iter() {
        let tpe = page.read_field(&Identifier::lazy(name)).unwrap();
        let str = name.bright_black();
        let whitespace = String::from_utf8(vec![b' '; 75 - str.len()]).unwrap();
        println!(
            "{str}{whitespace}{}\n",
            typewriter.printer.tpe(&tpe).bright_cyan().bold()
        );
    }

    page
}

#[test]
fn undefined() {
    harness_type_expr("undefined", Type::Undefined);
}

#[test]
fn noone() {
    harness_type_expr("noone", Type::Noone);
}

#[test]
fn bool() {
    harness_type_expr("true", Type::Bool);
    harness_type_expr("false", Type::Bool);
}

#[test]
fn real() {
    harness_type_expr("0", Type::Real);
}

#[test]
fn string() {
    harness_type_expr("\"hi\"", Type::String);
}

#[test]
fn grouping() {
    harness_type_expr("(0)", Type::Real);
}

#[test]
fn empty_array() {
    harness_type_expr(
        "[]",
        Type::Array {
            member_type: Box::new(Type::Unknown),
        },
    );
}

#[test]
fn constant_array() {
    harness_type_expr(
        "[0]",
        Type::Array {
            member_type: Box::new(Type::Real),
        },
    );
}

#[test]
fn nested_arrays() {
    harness_type_expr(
        "[[[0]]]",
        Type::Array {
            member_type: Box::new(Type::Array {
                member_type: Box::new(Type::Array {
                    member_type: Box::new(Type::Real),
                }),
            }),
        },
    );
}

#[test]
fn empty_struct() {
    harness_type_expr(
        "{}",
        Type::Struct {
            fields: HashMap::default(),
        },
    );
}

#[test]
fn filled_struct() {
    harness_type_expr(
        "{b: 0, c: undefined}",
        Type::Struct {
            fields: HashMap::from([("b".into(), Type::Real), ("c".into(), Type::Undefined)]),
        },
    );
}

#[test]
fn array_access() {
    harness_type_ast(
        "var a = [0]
        var b = a[0];",
        [("b", Type::Real)],
    );
}

#[test]
fn struct_access() {
    harness_type_ast(
        "var a = {b: 0, c: true}
        var b = a.b;",
        [("b", Type::Real)],
    );
}

#[test]
fn complex_struct_and_array_nesting() {
    harness_type_ast(
        "var foo = { a: [ { b: 20, c: \"hi\" } ] };
        var bar = foo.a;
        var fizz = bar[0].b;
        var buzz = [bar[0].c, \"test\"];
        var bam = buzz[0];
        var boom = { a: foo, b: bar, c: fizz, d: buzz, e: bam };
        var woo = boom.a.a[0].c;",
        [("woo", Type::String)],
    );
}

#[test]
fn postfix() {
    harness_type_expr("a++", Type::Real);
    harness_type_expr("a--", Type::Real);
}

#[test]
fn unary() {
    harness_type_expr("++a", Type::Real);
    harness_type_expr("--a", Type::Real);
    harness_type_expr("+a", Type::Real);
    harness_type_expr("-a", Type::Real);
    harness_type_expr("~b", Type::Real);
    harness_type_expr("!b", Type::Bool);
}

#[test]
fn ternary() {
    harness_type_expr("true ? 0 : 0", Type::Real);
}

#[test]
fn null_coalecence() {
    // TODO!
}

#[test]
fn evaluation() {
    harness_type_expr("1 + 1", Type::Real);
    harness_type_expr("\"foo\" + \"foo\"", Type::String);
}

#[test]
fn logical() {
    harness_type_expr("true && false", Type::Bool);
}

#[test]
fn function() {
    harness_type_expr(
        "function () {}",
        Type::Function {
            parameters: vec![],
            return_type: Box::new(Type::Undefined),
        },
    );
}

#[test]
fn function_returns_constant() {
    harness_type_expr(
        "function () { return 0; }",
        Type::Function {
            parameters: vec![],
            return_type: Box::new(Type::Real),
        },
    );
}

#[test]
fn function_call() {
    harness_type_ast(
        "var foo = function() {
            return 0;
        }
        var bar = foo();",
        [("bar", Type::Real)],
    )
}

#[test]
fn generic_call() {
    let call = get_type("foo(0, true, \"bar\")");
    if let Type::Generic { term } = call.clone() {
        if let Term::App(App::Call(target, _)) = term.as_ref() {
            assert_eq!(
                call,
                Type::Generic {
                    term: Box::new(Term::App(App::Call(
                        target.clone(),
                        vec![Term::Type(Type::Real), Term::Type(Type::Bool), Term::Type(Type::String)],
                    ))),
                },
            );
            return;
        }
    }
    panic!("Result was {:?}!", call);
}

#[test]
fn function_infer_generic_call() {
    harness_type_ast(
        "var foo = function(a) {
            return a(0) + \"hello!\";
        }
        var bar = function(b) { 
            return b;
        }
        var buzz = foo(bar, bar)",
        [("buzz", Type::String)],
    );
}

#[test]
fn function_call_generic() {
    harness_type_ast(
        "var foo = function(a) {
            return a[0];
        }
        var bar = foo([0])",
        [("bar", Type::Real)],
    );
    // harness_type_ast(
    //     "
    //     var foo = function(a, b) {
    //         return a[b];
    //     }
    //     var bar = foo([\"hello\"], 0);
    //     var fizz = foo([1], 0);
    //     var buzz = foo([ { a: [0] } ], 0);",
    //     [
    //         ("bar", Type::String),
    //         ("fizz", Type::Real),
    //         (
    //             "buzz",
    //             Type::Struct {
    //                 fields: HashMap::from([(
    //                     "a".to_string(),
    //                     Type::Array {
    //                         member_type: Box::new(Type::Real),
    //                     },
    //                 )]),
    //             },
    //         ),
    //     ],
    // );
}

#[test]
fn function_generics() {
    let (function, mut parameters, _) = get_function("function(a) { return a; }");
    let param_one = parameters.pop().unwrap();
    let expected = Type::Function {
        parameters: vec![param_one.clone()],
        return_type: Box::new(param_one),
    };
    assert_eq!(function, expected);
}

#[test]
fn function_generic_array() {
    let (function, mut parameters, _) = get_function("function(foo) { return foo[0]; }");
    let param_one = parameters.pop().unwrap();
    if let Type::Array { member_type } = param_one {
        let expected = Type::Function {
            parameters: vec![Type::Array {
                member_type: member_type.clone(),
            }],
            return_type: member_type,
        };
        assert_eq!(function, expected);
    } else {
        panic!("Expected a generic array!");
    }
}

#[test]
fn function_infer_arguments() {
    harness_type_expr(
        "function(a) {
            return a[0] && true;
        }",
        Type::Function {
            parameters: vec![Type::Array {
                member_type: Box::new(Type::Bool),
            }],
            return_type: Box::new(Type::Bool),
        },
    );
}
