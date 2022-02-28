use crate::gml::{GmlEnum, GmlEnumMember, GmlSwitch, GmlSwitchCase};
use crate::parsing::expression::Scope;
use crate::parsing::parser::Parser;
use crate::parsing::{
    expression::{AssignmentOperator, EqualityOperator, Expression, Literal, PostfixOperator},
    statement::Statement,
};
use colored::Colorize;

fn harness_stmt(source: &str, expected: Statement) {
    let mut parser = Parser::new(source, "test".into());
    let outputed = parser.statement().unwrap();
    if *outputed.statement() != expected {
        panic!(
            "\n{}\n\n{}\n\n{}: {:?}\n\n{}: {:?}\n",
            "Failed a test on the following gml: ".yellow().bold(),
            source,
            "Expected".green().bold(),
            expected,
            "Outputed".red().bold(),
            *outputed.statement(),
        )
    }
}

#[test]
fn macro_declaration() {
    harness_stmt(
        "#macro foo 0",
        Statement::MacroDeclaration("foo".into(), None, "0".into()),
    )
}

#[test]
fn config_macro() {
    harness_stmt(
        "#macro bar:foo 0",
        Statement::MacroDeclaration("foo".into(), Some("bar".into()), "0".into()),
    )
}

#[test]
fn two_macro_declaration() {
    harness_stmt(
        "{ \n#macro foo 0\n#macro bar 0\n }",
        Statement::Block(vec![
            Statement::MacroDeclaration("foo".into(), None, "0".into()).lazy_box(),
            Statement::MacroDeclaration("bar".into(), None, "0".into()).lazy_box(),
        ]),
    )
}

#[test]
fn enum_declaration() {
    harness_stmt(
        "enum Foo { Bar, Baz }",
        Statement::EnumDeclaration(GmlEnum::new_with_members(
            "Foo",
            vec![
                GmlEnumMember::new("Bar", None),
                GmlEnumMember::new("Baz", None),
            ],
        )),
    )
}

#[test]
fn enum_with_values() {
    harness_stmt(
        "enum Foo { Bar = 20, Baz }",
        Statement::EnumDeclaration(GmlEnum::new_with_members(
            "Foo",
            vec![
                GmlEnumMember::new(
                    "Bar",
                    Some(Expression::Literal(Literal::Real(20.0)).lazy_box()),
                ),
                GmlEnumMember::new("Baz", None),
            ],
        )),
    )
}

#[test]
fn enum_with_neighbor_values() {
    harness_stmt(
        "enum Foo { Bar, Baz = Foo.Bar }",
        Statement::EnumDeclaration(GmlEnum::new_with_members(
            "Foo",
            vec![
                GmlEnumMember::new("Bar", None),
                GmlEnumMember::new(
                    "Baz",
                    Some(
                        Expression::Access(
                            Scope::Dot(Expression::Identifier("Foo".into()).lazy_box()),
                            Expression::Identifier("Bar".into()).lazy_box(),
                        )
                        .lazy_box(),
                    ),
                ),
            ],
        )),
    )
}

#[test]
fn globalvar() {
    harness_stmt(
        "globalvar foo;",
        Statement::GlobalvarDeclaration("foo".into()),
    )
}

#[test]
fn local_variable() {
    harness_stmt(
        "var i;",
        Statement::LocalVariableSeries(vec![("i".into(), None)]),
    )
}

#[test]
fn local_variable_with_value() {
    harness_stmt(
        "var i = 0;",
        Statement::LocalVariableSeries(vec![(
            "i".into(),
            Some(Expression::Literal(Literal::Real(0.0)).lazy_box()),
        )]),
    )
}

#[test]
fn local_variable_series() {
    harness_stmt(
        "var i, j = 0, h;",
        Statement::LocalVariableSeries(vec![
            ("i".into(), None),
            (
                "j".into(),
                Some(Expression::Literal(Literal::Real(0.0)).lazy_box()),
            ),
            ("h".into(), None),
        ]),
    )
}

#[test]
fn local_variable_trailling_comma() {
    harness_stmt(
        "var i = 0,",
        Statement::LocalVariableSeries(vec![(
            "i".into(),
            Some(Expression::Literal(Literal::Real(0.0)).lazy_box()),
        )]),
    )
}

#[test]
fn local_variable_series_ending_without_marker() {
    harness_stmt(
        "{ var i = 0 j = 0 }",
        Statement::Block(vec![
            Statement::LocalVariableSeries(vec![(
                "i".into(),
                Some(Expression::Literal(Literal::Real(0.0)).lazy_box()),
            )])
            .lazy_box(),
            Statement::Expression(
                Expression::Assignment(
                    Expression::Identifier("j".into()).lazy_box(),
                    AssignmentOperator::Equal,
                    Expression::Literal(Literal::Real(0.0)).lazy_box(),
                )
                .lazy_box(),
            )
            .lazy_box(),
        ]),
    )
}

#[test]
fn try_catch() {
    harness_stmt(
        "try {} catch (e) {}",
        Statement::TryCatch(
            Statement::Block(vec![]).lazy_box(),
            Expression::Grouping(Expression::Identifier("e".into()).lazy_box()).lazy_box(),
            Statement::Block(vec![]).lazy_box(),
            None,
        ),
    )
}

#[test]
fn try_catch_finally() {
    harness_stmt(
        "try {} catch (e) {} finally {}",
        Statement::TryCatch(
            Statement::Block(vec![]).lazy_box(),
            Expression::Grouping(Expression::Identifier("e".into()).lazy_box()).lazy_box(),
            Statement::Block(vec![]).lazy_box(),
            Some(Statement::Block(vec![]).lazy_box()),
        ),
    )
}

#[test]
fn for_loop() {
    harness_stmt(
        "for (var i = 0; i < 1; i++) {}",
        Statement::For(
            Statement::LocalVariableSeries(vec![(
                "i".into(),
                Some(Expression::Literal(Literal::Real(0.0)).lazy_box()),
            )])
            .lazy_box(),
            Expression::Equality(
                Expression::Identifier("i".into()).lazy_box(),
                EqualityOperator::LessThan,
                Expression::Literal(Literal::Real(1.0)).lazy_box(),
            )
            .lazy_box(),
            Statement::Expression(
                Expression::Postfix(
                    Expression::Identifier("i".into()).lazy_box(),
                    PostfixOperator::Increment,
                )
                .lazy_box(),
            )
            .lazy_box(),
            Statement::Block(vec![]).lazy_box(),
        ),
    );
}

#[test]
fn with() {
    harness_stmt(
        "with foo {}",
        Statement::With(
            Expression::Identifier("foo".into()).lazy_box(),
            Statement::Block(vec![]).lazy_box(),
        ),
    )
}

#[test]
fn repeat() {
    harness_stmt(
        "repeat 1 {}",
        Statement::Repeat(
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
            Statement::Block(vec![]).lazy_box(),
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
                    Expression::Identifier("foo".into()).lazy_box(),
                    AssignmentOperator::PlusEqual,
                    Expression::Literal(Literal::Real(1.0)).lazy_box(),
                )
                .lazy_box(),
            )
            .lazy_box()])
            .lazy_box(),
            Expression::Equality(
                Expression::Identifier("foo".into()).lazy_box(),
                EqualityOperator::Equal,
                Expression::Literal(Literal::Real(1.0)).lazy_box(),
            )
            .lazy_box(),
        ),
    )
}

#[test]
fn while_loop() {
    harness_stmt(
        "while foo == 1 { foo += 1; }",
        Statement::While(
            Expression::Equality(
                Expression::Identifier("foo".into()).lazy_box(),
                EqualityOperator::Equal,
                Expression::Literal(Literal::Real(1.0)).lazy_box(),
            )
            .lazy_box(),
            Statement::Block(vec![Statement::Expression(
                Expression::Assignment(
                    Expression::Identifier("foo".into()).lazy_box(),
                    AssignmentOperator::PlusEqual,
                    Expression::Literal(Literal::Real(1.0)).lazy_box(),
                )
                .lazy_box(),
            )
            .lazy_box()])
            .lazy_box(),
        ),
    )
}

#[test]
fn if_statement() {
    harness_stmt(
        "if foo == 1 {}",
        Statement::If(
            Expression::Equality(
                Expression::Identifier("foo".into()).lazy_box(),
                EqualityOperator::Equal,
                Expression::Literal(Literal::Real(1.0)).lazy_box(),
            )
            .lazy_box(),
            Statement::Block(vec![]).lazy_box(),
            None,
            false,
        ),
    )
}

#[test]
fn if_then() {
    harness_stmt(
        "if foo == 1 then {}",
        Statement::If(
            Expression::Equality(
                Expression::Identifier("foo".into()).lazy_box(),
                EqualityOperator::Equal,
                Expression::Literal(Literal::Real(1.0)).lazy_box(),
            )
            .lazy_box(),
            Statement::Block(vec![]).lazy_box(),
            None,
            true,
        ),
    )
}

#[test]
fn if_else() {
    harness_stmt(
        "if foo == 1 {} else {}",
        Statement::If(
            Expression::Equality(
                Expression::Identifier("foo".into()).lazy_box(),
                EqualityOperator::Equal,
                Expression::Literal(Literal::Real(1.0)).lazy_box(),
            )
            .lazy_box(),
            Statement::Block(vec![]).lazy_box(),
            Some(Statement::Block(vec![]).lazy_box()),
            false,
        ),
    )
}

#[test]
fn switch() {
    harness_stmt(
        "switch foo {}",
        Statement::Switch(GmlSwitch::new(
            Expression::Identifier("foo".into()).lazy_box(),
            vec![],
            None,
        )),
    )
}

#[test]
fn switch_with_case() {
    harness_stmt(
        "switch foo { case bar: break; }",
        Statement::Switch(GmlSwitch::new(
            Expression::Identifier("foo".into()).lazy_box(),
            vec![GmlSwitchCase::new(
                Expression::Identifier("bar".into()).lazy_box(),
                vec![Statement::Break.lazy_box()],
            )],
            None,
        )),
    )
}

#[test]
fn switch_case_fallthrough() {
    harness_stmt(
        "switch foo { case bar: case baz: break; }",
        Statement::Switch(GmlSwitch::new(
            Expression::Identifier("foo".into()).lazy_box(),
            vec![
                GmlSwitchCase::new(Expression::Identifier("bar".into()).lazy_box(), vec![]),
                GmlSwitchCase::new(
                    Expression::Identifier("baz".into()).lazy_box(),
                    vec![Statement::Break.lazy_box()],
                ),
            ],
            None,
        )),
    )
}

#[test]
fn switch_default() {
    harness_stmt(
        "switch foo { default: break; }",
        Statement::Switch(GmlSwitch::new(
            Expression::Identifier("foo".into()).lazy_box(),
            vec![],
            Some(vec![Statement::Break.lazy_box()]),
        )),
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
        Statement::Block(vec![Statement::Return(None).lazy_box()]),
    )
}

#[test]
fn return_statement() {
    harness_stmt("return;", Statement::Return(None))
}

#[test]
fn return_with_value() {
    harness_stmt(
        "return 0;",
        Statement::Return(Some(Expression::Literal(Literal::Real(0.0)).lazy_box())),
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

#[test]
fn excess_semicolons() {
    harness_stmt("exit;;;", Statement::Exit)
}
