use duck::parsing::{
    expression::{
        AccessScope, AssignmentOperator, Constructor, EvaluationOperator, Expression, Function,
        Literal, PostfixOperator,
    },
    parser::Ast,
    statement::{Case, Statement},
};
use pretty_assertions::assert_eq;

use duck::parsing::parser::Parser;

fn harness_ast(source: &str, expected: Ast) {
    let parser = Parser::new(source, "test".into());
    assert_eq!(parser.into_ast().unwrap(), expected);
}
