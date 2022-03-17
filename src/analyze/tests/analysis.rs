use crate::{
    analyze::{Constraint, Marker, Symbol, Type, TypeWriter},
    parse::*,
};
use colored::Colorize;
use hashbrown::{HashMap, HashSet};
use pretty_assertions::assert_eq;

fn harness_type(source: &'static str, name: &'static str, expected_tpe: Type) {
    let parser = Parser::new(source, 0);
    let mut type_writer = TypeWriter::default();
    let mut ast = parser.into_ast().unwrap();
    type_writer.write_types(&mut ast);

    println!("Result for: {source}");
    for (name, marker) in type_writer.scope.iter() {
        let tpe = type_writer.resolve_type(*marker);
        let str = name.bright_black();
        let whitespace = String::from_utf8(vec![b' '; 75 - str.len()]).unwrap();
        println!("{str}{whitespace}{tpe}\n");
    }
    assert_eq!(
        type_writer
            .scope
            .get(name)
            .map_or(Type::Unknown, |marker| type_writer.resolve_type(*marker)),
        expected_tpe,
    );
}

#[test]
fn unknown() {
    harness_type("var a;", "a", Type::Unknown);
}

#[test]
fn undefined() {
    harness_type("var a = undefined;", "a", Type::Undefined);
}

#[test]
fn noone() {
    harness_type("var a = noone;", "a", Type::Noone);
}

#[test]
fn bool() {
    harness_type("var a = true;", "a", Type::Bool);
    harness_type("var a = false;", "a", Type::Bool);
}

#[test]
fn real() {
    harness_type("var a = 0;", "a", Type::Real);
}

#[test]
fn string() {
    harness_type("var a = \"hi\";", "a", Type::String);
}

#[test]
fn empty_array() {
    harness_type("var a = []", "a", Type::Array(Box::new(Type::Unknown)));
}

#[test]
fn constant_array() {
    harness_type("var a = [0]", "a", Type::Array(Box::new(Type::Real)));
}

#[test]
fn nested_arrays() {
    harness_type(
        "var a = [[[0]]]",
        "a",
        Type::Array(Box::new(Type::Array(Box::new(Type::Array(Box::new(Type::Real)))))),
    );
}

#[test]
fn empty_struct() {
    harness_type("var a = {}", "a", Type::Struct(HashMap::default()));
}

#[test]
fn filled_struct() {
    harness_type(
        "var a = {b: 0, c: undefined}",
        "a",
        Type::Struct(HashMap::from([("b".into(), Type::Real), ("c".into(), Type::Undefined)])),
    );
}

#[test]
fn array_access() {
    harness_type(
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
    harness_type(
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
    harness_type(
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
