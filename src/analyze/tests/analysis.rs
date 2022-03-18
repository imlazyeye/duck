use crate::{
    analyze::{Type, TypeWriter},
    parse::*,
};
use colored::Colorize;
use hashbrown::HashMap;
use pretty_assertions::assert_eq;

fn harness_type_ast(source: &'static str, name: &'static str, expected_tpe: Type) {
    let typewriter = harness_typewriter(source);
    assert_eq!(
        typewriter
            .scope
            .get(name)
            .map_or(Type::Unknown, |marker| typewriter.resolve_type(*marker)),
        expected_tpe,
    );
}

fn harness_type_expr(source: &'static str, expected_tpe: Type) {
    let source = Box::leak(Box::new(format!("var a = {source}")));
    let typewriter = harness_typewriter(source);
    assert_eq!(
        typewriter
            .scope
            .get("a")
            .map_or(Type::Unknown, |marker| typewriter.resolve_type(*marker)),
        expected_tpe,
    );
}

fn harness_typewriter(source: &'static str) -> TypeWriter {
    let parser = Parser::new(source, 0);
    let mut typewriter = TypeWriter::default();
    let mut ast = parser.into_ast().unwrap();
    typewriter.write_types(&mut ast);

    println!("Result for: {source}");
    for (name, marker) in typewriter.scope.iter() {
        let tpe = typewriter.resolve_type(*marker);
        let str = name.bright_black();
        let whitespace = String::from_utf8(vec![b' '; 75 - str.len()]).unwrap();
        println!("{str}{whitespace}{tpe}\n");
    }

    typewriter
}

#[test]
fn unknown() {
    harness_type_ast("var a;", "a", Type::Unknown);
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
    harness_type_expr("[]", Type::Array(Box::new(Type::Unknown)));
}

#[test]
fn constant_array() {
    harness_type_expr("[0]", Type::Array(Box::new(Type::Real)));
}

#[test]
fn nested_arrays() {
    harness_type_expr(
        "[[[0]]]",
        Type::Array(Box::new(Type::Array(Box::new(Type::Array(Box::new(Type::Real)))))),
    );
}

#[test]
fn empty_struct() {
    harness_type_expr("{}", Type::Struct(HashMap::default()));
}

#[test]
fn filled_struct() {
    harness_type_expr(
        "{b: 0, c: undefined}",
        Type::Struct(HashMap::from([("b".into(), Type::Real), ("c".into(), Type::Undefined)])),
    );
}

#[test]
fn array_access() {
    harness_type_ast(
        "
        var a = [0]
        var b = a[0];
        ",
        "b",
        Type::Real,
    );
}

#[test]
fn struct_access() {
    harness_type_ast(
        "
        var a = {b: 0}
        var b = a.b;
        ",
        "b",
        Type::Real,
    );
}

#[test]
fn complex_struct_and_array_nesting() {
    harness_type_ast(
        "
        var foo = { a: [ { b: 20, c: \"hi\" } ] };
        var bar = foo.a;
        var fizz = bar[0].b;
        var buzz = [bar[0].c, \"test\"];
        var bam = buzz[0];
        var boom = { a: foo, b: bar, c: fizz, d: buzz, e: bam };
        var woo = boom.a.a[0].c;
        ",
        "woo",
        Type::String,
    );
}

#[test]
fn infer_from_postfix() {
    harness_type_expr("a++", Type::Real);
    harness_type_expr("a--", Type::Real);
}

#[test]
fn infer_from_unary() {
    harness_type_expr("++a", Type::Real);
    harness_type_expr("--a", Type::Real);
    harness_type_expr("+a", Type::Real);
    harness_type_expr("-a", Type::Real);
    harness_type_expr("~b", Type::Real);
    harness_type_expr("!b", Type::Bool);
}

#[test]
fn infer_from_ternary() {
    let source = "
        var a;
        var b;
        var c = a ? b : 0;
    ";
    harness_type_ast(source, "a", Type::Bool);
    harness_type_ast(source, "b", Type::Real);
    harness_type_ast(source, "c", Type::Real);
}

#[test]
fn infer_from_null_coalecence() {
    let source = "
        var a;
        var b = a ?? 0;
    ";
    harness_type_ast(source, "a", todo!());
    harness_type_ast(source, "b", Type::Real);
}

#[test]
fn infer_from_evaluation() {
    let source = "
        var a;
        var b = a == 0;
    ";
    harness_type_ast(source, "a", Type::Real);
    harness_type_ast(source, "b", Type::Bool);
}

#[test]
fn infer_from_logical() {
    let source = "
        var a;
        var b = a && true;
    ";
    harness_type_ast(source, "a", Type::Bool);
    harness_type_ast(source, "b", Type::Bool);
}