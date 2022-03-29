use crate::{analyze::*, parse::*};
use colored::Colorize;
use hashbrown::HashMap;
use pretty_assertions::assert_eq;

struct TestTypeWriter(Typewriter);
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

fn harness_type_ast(source: &'static str, pairs: impl Into<Vec<(&'static str, Type)>>) {
    for (name, expected_type) in pairs.into() {
        assert_eq!(get_var_type(source, name), expected_type, "{} was wrong value!", name);
    }
}

fn harness_type_expr(source: &'static str, expected_tpe: Type) {
    assert_eq!(get_type(source), expected_tpe);
}

fn get_type(source: &'static str) -> Type {
    let source = Box::leak(Box::new(format!("var a = {source}")));
    let (typewriter, scope) = harness_typewriter(source);
    scope.lookup_type(&Identifier::lazy("a"), &typewriter).unwrap()
}

fn get_var_type(source: &'static str, name: &'static str) -> Type {
    let (typewriter, scope) = harness_typewriter(source);
    scope.lookup_type(&Identifier::lazy(name), &typewriter).unwrap()
}

fn harness_typewriter(source: &str) -> (TestTypeWriter, Scope) {
    let source = Box::leak(Box::new(source.to_string()));
    let parser = Parser::new(source, 0);
    let mut typewriter = Typewriter::new();
    let mut scope = Scope::new(&mut typewriter);
    let mut ast = parser.into_ast().unwrap();
    typewriter.write(&mut scope, ast.stmts_mut());
    println!("Result for: \n{source}");
    for name in scope.local_fields().iter() {
        let str = name.bright_black();
        let tpe = scope.lookup_type(&Identifier::lazy(name), &typewriter).unwrap();
        let whitespace = String::from_utf8(vec![b' '; 75 - str.len()]).unwrap();
        println!("{str}{whitespace}{}\n", Printer::tpe(&tpe).bright_cyan().bold());
    }
    (TestTypeWriter(typewriter), scope)
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
fn addition() {
    harness_type_expr("1 + 1", Type::Real);
    harness_type_expr("\"foo\" + \"foo\"", Type::String);
}

#[test]
fn non_addition_evaluations() {
    harness_type_expr("1 * 1", Type::Real);
    harness_type_expr("1 / 1", Type::Real);
    harness_type_expr("1 div 1", Type::Real);
    harness_type_expr("1 mod 1", Type::Real);
}

#[test]
fn logical() {
    harness_type_expr("true && false", Type::Bool);
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
fn populated_struct() {
    harness_type_expr(
        "{ b: 0, c: undefined }",
        Type::Struct {
            fields: HashMap::from([("b".into(), Type::Real), ("c".into(), Type::Undefined)]),
        },
    );
}

#[test]
fn array_access() {
    harness_type_ast(
        "var foo = [0]
        var bar = foo[0];",
        [("bar", Type::Real)],
    );
}

#[test]
fn struct_access() {
    harness_type_ast(
        "var foo = { b: 0 }
        var bar = foo.b;",
        [("bar", Type::Real)],
    );
}

#[test]
fn function() {
    harness_type_expr(
        "function() {}",
        Type::Function {
            parameters: vec![],
            return_type: Box::new(Type::Undefined),
        },
    );
}

#[test]
fn return_constant() {
    harness_type_expr(
        "function () { return 0; }",
        Type::Function {
            parameters: vec![],
            return_type: Box::new(Type::Real),
        },
    );
}

#[test]
fn call_default_return_value() {
    harness_type_ast(
        "var foo = function() {}
        var bar = foo();",
        [("bar", Type::Undefined)],
    )
}

#[test]
fn call_constant_return_value() {
    harness_type_ast(
        "var foo = function() {
            return 0;
        }
        var bar = foo();",
        [("bar", Type::Real)],
    );
}

#[test]
fn simple_generic_function() {
    harness_type_ast(
        "function foo(a) {
            return a;
        }
        var bar = foo(0)",
        [("bar", Type::Real)],
    );
}

#[test]
fn generic_function() {
    harness_type_ast(
        "function foo(a) {
            return a[0];
        }
        var bar = foo([0])",
        [("bar", Type::Real)],
    );
}

#[test]
fn complex_generic_function() {
    harness_type_ast(
        "function foo(a, b) {
            return a[b];
        }
        function bar(x, y) { 
            return x + y * 2; 
        }
        var fizz = foo([\"hello\"], 0);
        var buzz = foo([ { a: true } ], bam(1, 2));",
        [
            ("fizz", Type::String),
            (
                "buzz",
                Type::Struct {
                    fields: HashMap::from([("a".to_string(), Type::Bool)]),
                },
            ),
        ],
    );
}

#[test]
fn multi_use_generic_function() {
    harness_type_ast(
        "var foo = function(a) {
            return a;
        }
        var bar = foo(true)
        var fizz = foo(0)",
        [("bar", Type::Bool), ("fizz", Type::Real)],
    );
}

/*
function x(n) { // fn foo<T>(T) -> T
    return n;
}

function foo(x) {
    var _ = x(1) + 1; // x impl fn x(int) -> int
    var _ = x(true) || true; // x impl fn x(bool) -> bool
}

Okay, here's my conclusion. At least in the context of the above code, `x` should not be expressed as a concrete type.
Instead, x is: `T: Callable<(int) -> int>, Callable<(bool) -> bool>`

*/

#[test]
fn inferred_function() {
    harness_type_ast(
        "var foo = function(x) {
            return x() + 1;
        }",
        [(
            "foo",
            Type::Function {
                parameters: vec![],
                return_type: Box::new(Type::Real),
            },
        )],
    );
}

#[test]
fn inferred_array() {
    harness_type_expr(
        "function(a) {
            return a[0] + 1;
        }",
        Type::Function {
            parameters: vec![Type::Array {
                member_type: Box::new(Type::Real),
            }],
            return_type: Box::new(Type::Real),
        },
    );
}

#[test]
fn inferred_struct() {
    harness_type_expr(
        "function(a) {
            return a.b + 1;
        }",
        Type::Function {
            parameters: vec![Type::Generic {
                term: Box::new(Term::Generic(vec![Trait::FieldOp(FieldOp::Readable(
                    "b".into(),
                    Box::new(Term::Type(Type::Real)),
                ))])),
            }],
            return_type: Box::new(Type::Real),
        },
    );
}

#[test]
fn wrapped_lambda_in_struct_field() {
    harness_type_ast(
        "function wrapper(lambda) {
            return { y: lambda(0) };
        }
        function inner(n) { return n; }
        var data = wrapper(inner);",
        [(
            "data",
            Type::Struct {
                fields: HashMap::from([("y".into(), Type::Real)]),
            },
        )],
    );
}

#[test]
fn complex_data() {
    harness_type_ast(
        "function build_data(x, y, z) {
            return {
                x: x,
                y: y(0),
                z: z[0][0].a.b.c,
            };
        }
        function build_x(x) { return x; }
        function y_fn(n) { return n; }
        var z = [[{ a: { b: { c: 0 }}}]];
        var data = build_data(build_x(0), y_fn, z);
        var output = data.x + data.y + data.z;",
        [("output", Type::Real)],
    );
}

// #[test]
// fn default_arguments() {
//     harness_type_ast(
//         "function foo(x=0) {
//             return x;
//         }
//         var bar = foo();",
//         [("bar", Type::Real)],
//     );
// }

#[test]
fn field_trait() {
    harness_type_ast(
        "var foo = function(a) {
            a.a = 0;
            a.b = 0;
            return a;
        }
        var bar = { a: 0, b: 0, c: 0 };
        foo(bar);",
        [(
            "bar",
            Type::Struct {
                fields: HashMap::from([
                    ("a".into(), Type::Real),
                    ("b".into(), Type::Real),
                    ("c".into(), Type::Real),
                ]),
            },
        )],
    );
}

#[test]
fn self_assignment() {
    harness_type_ast("foo = 0", [("foo", Type::Real)]);
}

#[test]
fn self_assignment_with_keyword() {
    harness_type_ast("self.foo = 0", [("foo", Type::Real)]);
}

#[test]
fn mutate_self_via_function() {
    harness_type_ast(
        "var foo = function() {
            self.a = 0;
        }
        foo();",
        [("a", Type::Real)],
    );
}

#[test]
fn mutate_self_via_nested_function() {
    harness_type_ast(
        "var foo = function() {
            self.a = 0;
        }
        var bar = function(foo) {
            foo();
        }
        bar(foo);",
        [("a", Type::Real)],
    );
}

#[test]
fn constructor() {
    harness_type_ast(
        "function Foo() constructor {
            self.a = 0;
        }
        var foo = new Foo();",
        [(
            "foo",
            Type::Struct {
                fields: HashMap::from([("a".into(), Type::Real)]),
            },
        )],
    );
}

#[test]
fn manual_inheritance() {
    harness_type_ast(
        "function Foo() {
            self.a = 0;
        }
        function Bar(foo) constructor {
            foo();
        }
        var bar = new Bar(Foo);",
        [(
            "bar",
            Type::Struct {
                fields: HashMap::from([("a".into(), Type::Real)]),
            },
        )],
    );
}

#[test]
fn inheritance() {
    harness_type_ast(
        "function Foo() constructor {
            self.a = 0;
        }
        function Bar() : Foo() constructor {
        }
        var bar = new Bar();",
        [(
            "bar",
            Type::Struct {
                fields: HashMap::from([("a".into(), Type::Real)]),
            },
        )],
    );
}
