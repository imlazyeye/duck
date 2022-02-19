use duck::parsing::expression::UnaryOperator;
use duck::parsing::{
    expression::{AssignmentOperator, EvaluationOperator, Expression, Literal, PostfixOperator},
    statement::{Case, Constructor, Function, Parameter, Statement},
};
use pretty_assertions::assert_eq;

use duck::parsing::parser::Parser;

fn harness_stmt(source: &str, expected: Statement) {
    let mut parser = Parser::new(source, "test".into());
    assert_eq!(*parser.statement().unwrap(), expected);
}

#[test]
fn enum_declaration() {
    harness_stmt(
        "enum Foo { Bar, Baz }",
        Statement::EnumDeclaration(
            "Foo".into(),
            vec![
                Expression::Identifier("Bar".into()).into(),
                Expression::Identifier("Baz".into()).into(),
            ],
        ),
    )
}

#[test]
fn enum_with_values() {
    harness_stmt(
        "enum Foo { Bar = 20, Baz }",
        Statement::EnumDeclaration(
            "Foo".into(),
            vec![
                Expression::Assignment(
                    Expression::Identifier("Bar".into()).into(),
                    AssignmentOperator::Equals,
                    Expression::Literal(Literal::Real(20.0)).into(),
                )
                .into(),
                Expression::Identifier("Baz".into()).into(),
            ],
        ),
    )
}

#[test]
fn function() {
    harness_stmt(
        "function foo() {}",
        Statement::FunctionDeclaration(Function::Named(
            "foo".into(),
            vec![],
            None,
            Statement::Block(vec![]).into(),
        )),
    )
}

#[test]
fn function_with_parameters() {
    harness_stmt(
        "function foo(bar, baz) {}",
        Statement::FunctionDeclaration(Function::Named(
            "foo".into(),
            vec![Parameter("bar".into(), None), Parameter("baz".into(), None)],
            None,
            Statement::Block(vec![]).into(),
        )),
    )
}

#[test]
fn default_parameters() {
    harness_stmt(
        "function foo(bar=20, baz) {}",
        Statement::FunctionDeclaration(Function::Named(
            "foo".into(),
            vec![
                Parameter(
                    "bar".into(),
                    Some(Expression::Literal(Literal::Real(20.0)).into()),
                ),
                Parameter("baz".into(), None),
            ],
            None,
            Statement::Block(vec![]).into(),
        )),
    )
}

#[test]
fn anonymous_function() {
    harness_stmt(
        "function() {}",
        Statement::FunctionDeclaration(Function::Anonymous(
            vec![],
            None,
            Statement::Block(vec![]).into(),
        )),
    )
}

#[test]
fn constructor() {
    harness_stmt(
        "function foo() constructor {}",
        Statement::FunctionDeclaration(Function::Named(
            "foo".into(),
            vec![],
            Some(Constructor(None)),
            Statement::Block(vec![]).into(),
        )),
    )
}

#[test]
fn inheritance() {
    harness_stmt(
        "function foo() : bar() constructor {}",
        Statement::FunctionDeclaration(Function::Named(
            "foo".into(),
            vec![],
            Some(Constructor(Some(
                Expression::Call(Expression::Identifier("bar".into()).into(), vec![]).into(),
            ))),
            Statement::Block(vec![]).into(),
        )),
    )
}

#[test]
fn local_variable() {
    harness_stmt("var i;", Statement::VariableDeclaration("i".into(), None))
}

#[test]
fn local_variable_with_value() {
    harness_stmt(
        "var i = 0;",
        Statement::VariableDeclaration(
            "i".into(),
            Some(Expression::Literal(Literal::Real(0.0)).into()),
        ),
    )
}

#[test]
fn r#for() {
    harness_stmt(
        "for (var i = 0; i < 1; i++) {}",
        Statement::For(
            Statement::VariableDeclaration(
                "i".into(),
                Some(Expression::Literal(Literal::Real(0.0)).into()),
            )
            .into(),
            Statement::Expression(
                Expression::Evaluation(
                    Expression::Identifier("i".into()).into(),
                    EvaluationOperator::LessThan,
                    Expression::Literal(Literal::Real(1.0)).into(),
                )
                .into(),
            )
            .into(),
            Statement::Expression(
                Expression::Postfix(
                    Expression::Identifier("i".into()).into(),
                    PostfixOperator::Increment,
                )
                .into(),
            )
            .into(),
            Statement::Block(vec![]).into(),
        ),
    );
}

#[test]
fn with() {
    harness_stmt(
        "with foo {}",
        Statement::With(
            Expression::Identifier("foo".into()).into(),
            Statement::Block(vec![]).into(),
        ),
    )
}

#[test]
fn repeat() {
    harness_stmt(
        "repeat 1 {}",
        Statement::Repeat(
            Expression::Literal(Literal::Real(1.0)).into(),
            Statement::Block(vec![]).into(),
        ),
    )
}

#[test]
fn do_until() {
    harness_stmt(
        "do { foo += 1; } until foo == 1;",
        Statement::DoUntil(
            Statement::Block(vec![Statement::Expression(
                Expression::Assignment(
                    Expression::Identifier("foo".into()).into(),
                    AssignmentOperator::PlusEquals,
                    Expression::Literal(Literal::Real(1.0)).into(),
                )
                .into(),
            )
            .into()])
            .into(),
            Expression::Evaluation(
                Expression::Identifier("foo".into()).into(),
                EvaluationOperator::Equals,
                Expression::Literal(Literal::Real(1.0)).into(),
            )
            .into(),
        ),
    )
}

#[test]
fn while_loop() {
    harness_stmt(
        "while foo == 1 { foo += 1; }",
        Statement::While(
            Expression::Evaluation(
                Expression::Identifier("foo".into()).into(),
                EvaluationOperator::Equals,
                Expression::Literal(Literal::Real(1.0)).into(),
            )
            .into(),
            Statement::Block(vec![Statement::Expression(
                Expression::Assignment(
                    Expression::Identifier("foo".into()).into(),
                    AssignmentOperator::PlusEquals,
                    Expression::Literal(Literal::Real(1.0)).into(),
                )
                .into(),
            )
            .into()])
            .into(),
        ),
    )
}

#[test]
fn if_statement() {
    harness_stmt(
        "if foo == 1 {}",
        Statement::If(
            Expression::Evaluation(
                Expression::Identifier("foo".into()).into(),
                EvaluationOperator::Equals,
                Expression::Literal(Literal::Real(1.0)).into(),
            )
            .into(),
            Statement::Block(vec![]).into(),
            None,
        ),
    )
}

#[test]
fn if_else() {
    harness_stmt(
        "if foo == 1 {} else {}",
        Statement::If(
            Expression::Evaluation(
                Expression::Identifier("foo".into()).into(),
                EvaluationOperator::Equals,
                Expression::Literal(Literal::Real(1.0)).into(),
            )
            .into(),
            Statement::Block(vec![]).into(),
            Some(Statement::Block(vec![]).into()),
        ),
    )
}

#[test]
fn switch() {
    harness_stmt(
        "switch foo {}",
        Statement::Switch(Expression::Identifier("foo".into()).into(), vec![], None),
    )
}

#[test]
fn switch_with_case() {
    harness_stmt(
        "switch foo { case bar: break; }",
        Statement::Switch(
            Expression::Identifier("foo".into()).into(),
            vec![Case(
                Expression::Identifier("bar".into()).into(),
                vec![Statement::Break.into()],
            )],
            None,
        ),
    )
}

#[test]
fn switch_case_fallthrough() {
    harness_stmt(
        "switch foo { case bar: case baz: break; }",
        Statement::Switch(
            Expression::Identifier("foo".into()).into(),
            vec![
                Case(Expression::Identifier("bar".into()).into(), vec![]),
                Case(
                    Expression::Identifier("baz".into()).into(),
                    vec![Statement::Break.into()],
                ),
            ],
            None,
        ),
    )
}

#[test]
fn switch_default() {
    harness_stmt(
        "switch foo { default: break; }",
        Statement::Switch(
            Expression::Identifier("foo".into()).into(),
            vec![],
            Some(vec![Statement::Break.into()]),
        ),
    )
}

#[test]
fn empty_block() {
    harness_stmt("{}", Statement::Block(vec![]))
}

#[test]
fn block() {
    harness_stmt(
        "{ return; }",
        Statement::Block(vec![Statement::Return(None).into()]),
    )
}

#[test]
fn r#return() {
    harness_stmt("return;", Statement::Return(None))
}

#[test]
fn return_with_value() {
    harness_stmt(
        "return 0;",
        Statement::Return(Some(Expression::Literal(Literal::Real(0.0)).into())),
    )
}

#[test]
fn r#break() {
    harness_stmt("break;", Statement::Break)
}

#[test]
fn exit() {
    harness_stmt("exit;", Statement::Exit)
}
