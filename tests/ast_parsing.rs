use duck::parsing::parser::Ast;
use pretty_assertions::assert_eq;

use duck::parsing::parser::Parser;

#[allow(dead_code)]
fn harness_ast(source: &str, expected: Ast) {
    let parser = Parser::new(source, "test".into());
    assert_eq!(parser.into_ast().unwrap(), expected);
}
