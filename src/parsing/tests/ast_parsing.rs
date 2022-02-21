use crate::parsing::parser::Ast;
use crate::parsing::Parser;
use pretty_assertions::assert_eq;

#[allow(dead_code)]
fn harness_ast(source: &str, expected: Ast) {
    let parser = Parser::new(source, "test".into());
    assert_eq!(parser.into_ast().unwrap(), expected);
}
