use crate::{
    analyze::{Page, Type, TypeWriter},
    parse::*,
};
use colored::Colorize;
use hashbrown::HashMap;
use pretty_assertions::assert_eq;

fn harness_type_ast(source: &'static str, name: &'static str, expected_tpe: Type) {
    let page = harness_typewriter(source);
    assert_eq!(
        page.fields
            .get(name)
            .map_or(Type::Unknown, |marker| page.seek_type_for(*marker)),
        expected_tpe,
    );
}

fn harness_type_expr(source: &'static str, expected_tpe: Type) {
    let source = Box::leak(Box::new(format!("var a = {source}")));
    let page = harness_typewriter(source);
    assert_eq!(
        page.fields
            .get("a")
            .map_or(Type::Unknown, |marker| page.seek_type_for(*marker)),
        expected_tpe,
    );
}

fn harness_typewriter(source: &'static str) -> Page {
    let parser = Parser::new(source, 0);
    let mut typewriter = TypeWriter::default();
    let mut ast = parser.into_ast().unwrap();
    let page = typewriter.write_types(&mut ast);

    println!("Result for: {source}");
    for (name, marker) in page.fields.iter() {
        let tpe = page.seek_type_for(*marker);
        let str = name.bright_black();
        let whitespace = String::from_utf8(vec![b' '; 75 - str.len()]).unwrap();
        println!("{str}{whitespace}{tpe}\n");
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
    todo!()
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
