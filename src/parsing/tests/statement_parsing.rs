use crate::{
    gml::{
        Access, Assignment, AssignmentOperator, Block, DoUntil, Enum, EnumMember, Equality, EqualityOperator, ForLoop,
        Globalvar, Grouping, Identifier, If, Literal, LocalVariable, LocalVariableSeries, Macro, Postfix,
        PostfixOperator, RepeatLoop, Return, Switch, SwitchCase, TryCatch, WithLoop,
    },
    parsing::{parser::Parser, statement::Statement},
    prelude::{IntoExpressionBox, IntoStatementBox},
};
use colored::Colorize;
use pretty_assertions::assert_eq;

fn harness_stmt(source: &str, expected: impl Into<Statement>) {
    let expected = expected.into();
    let mut parser = Parser::new(source, "test".into());
    let outputed = parser.statement().unwrap();
    println!("{}: {}", "Source".yellow(), source);
    assert_eq!(expected, *outputed.statement())
}

#[test]
fn macro_declaration() {
    harness_stmt("#macro foo 0", Statement::MacroDeclaration(Macro::new("foo", "0")))
}

#[test]
fn config_macro() {
    harness_stmt("#macro bar:foo 0", Macro::new_with_config("foo", "0", "bar"))
}

#[test]
fn two_macro_declaration() {
    harness_stmt(
        "{ \n#macro foo 0\n#macro bar 0\n }",
        Block::new(vec![
            Macro::new("foo", "0").into_lazy_box(),
            Macro::new("bar", "0").into_lazy_box(),
        ]),
    )
}

#[test]
fn enum_declaration() {
    harness_stmt(
        "enum Foo { Bar, Baz }",
        Enum::new_with_members("Foo", vec![EnumMember::new("Bar", None), EnumMember::new("Baz", None)]),
    )
}

#[test]
fn enum_with_values() {
    harness_stmt(
        "enum Foo { Bar = 20, Baz }",
        Enum::new_with_members(
            "Foo",
            vec![
                EnumMember::new("Bar", Some(Literal::Real(20.0).into_lazy_box())),
                EnumMember::new("Baz", None),
            ],
        ),
    )
}

#[test]
fn enum_with_neighbor_values() {
    harness_stmt(
        "enum Foo { Bar, Baz = Foo.Bar }",
        Enum::new_with_members(
            "Foo",
            vec![
                EnumMember::new("Bar", None),
                EnumMember::new(
                    "Baz",
                    Some(
                        Access::Dot {
                            left: Identifier::new("Foo").into_lazy_box(),
                            right: Identifier::new("Bar").into_lazy_box(),
                        }
                        .into_lazy_box(),
                    ),
                ),
            ],
        ),
    )
}

#[test]
fn globalvar() {
    harness_stmt("globalvar foo;", Globalvar::new("foo"))
}

#[test]
fn local_variable() {
    harness_stmt(
        "var i;",
        LocalVariableSeries::new(vec![LocalVariable::Uninitialized(Identifier::new("i").into_lazy_box())]),
    )
}

#[test]
fn local_variable_with_value() {
    harness_stmt(
        "var i = 0;",
        LocalVariableSeries::new(vec![LocalVariable::Initialized(
            Assignment::new(
                Identifier::new("i").into_lazy_box(),
                AssignmentOperator::Equal,
                Literal::Real(0.0).into_lazy_box(),
            )
            .into_lazy_box(),
        )]),
    )
}

#[test]
fn local_variable_series() {
    harness_stmt(
        "var i, j = 0, h;",
        LocalVariableSeries::new(vec![
            LocalVariable::Uninitialized(Identifier::new("i").into_lazy_box()),
            LocalVariable::Initialized(
                Assignment::new(
                    Identifier::new("j").into_lazy_box(),
                    AssignmentOperator::Equal,
                    Literal::Real(0.0).into_lazy_box(),
                )
                .into_lazy_box(),
            ),
            LocalVariable::Uninitialized(Identifier::new("h").into_lazy_box()),
        ]),
    )
}

#[test]
fn local_variable_trailling_comma() {
    harness_stmt(
        "var i = 0,",
        LocalVariableSeries::new(vec![LocalVariable::Initialized(
            Assignment::new(
                Identifier::new("i").into_lazy_box(),
                AssignmentOperator::Equal,
                Literal::Real(0.0).into_lazy_box(),
            )
            .into_lazy_box(),
        )]),
    )
}

#[test]
fn local_variable_series_ending_without_marker() {
    harness_stmt(
        "{ var i = 0 j = 0 }",
        Block::new(vec![
            LocalVariableSeries::new(vec![LocalVariable::Initialized(
                Assignment::new(
                    Identifier::new("i").into_lazy_box(),
                    AssignmentOperator::Equal,
                    Literal::Real(0.0).into_lazy_box(),
                )
                .into_lazy_box(),
            )])
            .into_lazy_box(),
            Statement::Expression(
                Assignment::new(
                    Identifier::new("j").into_lazy_box(),
                    AssignmentOperator::Equal,
                    Literal::Real(0.0).into_lazy_box(),
                )
                .into_lazy_box(),
            )
            .into_lazy_box(),
        ]),
    )
}

#[test]
fn try_catch() {
    harness_stmt(
        "try {} catch (e) {}",
        TryCatch::new(
            Block::new(vec![]).into_lazy_box(),
            Grouping::new(Identifier::new("e").into_lazy_box()).into_lazy_box(),
            Block::new(vec![]).into_lazy_box(),
        ),
    )
}

#[test]
fn try_catch_finally() {
    harness_stmt(
        "try {} catch (e) {} finally {}",
        TryCatch::new_with_finally(
            Block::new(vec![]).into_lazy_box(),
            Grouping::new(Identifier::new("e").into_lazy_box()).into_lazy_box(),
            Block::new(vec![]).into_lazy_box(),
            Block::new(vec![]).into_lazy_box(),
        ),
    )
}

#[test]
fn for_loop() {
    harness_stmt(
        "for (var i = 0; i < 1; i++) {}",
        ForLoop::new(
            LocalVariableSeries::new(vec![LocalVariable::Initialized(
                Assignment::new(
                    Identifier::new("i").into_lazy_box(),
                    AssignmentOperator::Equal,
                    Literal::Real(0.0).into_lazy_box(),
                )
                .into_lazy_box(),
            )])
            .into_lazy_box(),
            Equality::new(
                Identifier::new("i").into_lazy_box(),
                EqualityOperator::LessThan,
                Literal::Real(1.0).into_lazy_box(),
            )
            .into_lazy_box(),
            Statement::Expression(
                Postfix::new(Identifier::new("i").into_lazy_box(), PostfixOperator::Increment).into_lazy_box(),
            )
            .into_lazy_box(),
            Block::new(vec![]).into_lazy_box(),
        ),
    );
}

#[test]
fn with() {
    harness_stmt(
        "with foo {}",
        WithLoop::new(
            Identifier::new("foo").into_lazy_box(),
            Block::new(vec![]).into_lazy_box(),
        ),
    )
}

#[test]
fn repeat() {
    harness_stmt(
        "repeat 1 {}",
        RepeatLoop::new(Literal::Real(1.0).into_lazy_box(), Block::new(vec![]).into_lazy_box()),
    )
}

#[test]
fn do_until() {
    harness_stmt(
        "do { foo += 1; } until foo == 1;",
        DoUntil::new(
            Block::new(vec![
                Assignment::new(
                    Identifier::new("foo").into_lazy_box(),
                    AssignmentOperator::PlusEqual,
                    Literal::Real(1.0).into_lazy_box(),
                )
                .into_lazy_box()
                .into_lazy_box(),
            ])
            .into_lazy_box(),
            Equality::new(
                Identifier::new("foo").into_lazy_box(),
                EqualityOperator::Equal,
                Literal::Real(1.0).into_lazy_box(),
            )
            .into_lazy_box(),
        ),
    )
}
#[test]
fn while_loop() {
    harness_stmt(
        "while foo == 1 { foo += 1; }",
        If::new(
            Equality::new(
                Identifier::new("foo").into_lazy_box(),
                EqualityOperator::Equal,
                Literal::Real(1.0).into_lazy_box(),
            )
            .into_lazy_box(),
            Block::new(vec![
                Assignment::new(
                    Identifier::new("foo").into_lazy_box(),
                    AssignmentOperator::PlusEqual,
                    Literal::Real(1.0).into_lazy_box(),
                )
                .into_lazy_box()
                .into_lazy_box(),
            ])
            .into_lazy_box(),
        ),
    )
}

#[test]
fn if_statement() {
    harness_stmt(
        "if foo == 1 {}",
        If::new(
            Equality::new(
                Identifier::new("foo").into_lazy_box(),
                EqualityOperator::Equal,
                Literal::Real(1.0).into_lazy_box(),
            )
            .into_lazy_box(),
            Block::new(vec![]).into_lazy_box(),
        ),
    )
}

#[test]
fn if_then() {
    harness_stmt(
        "if foo == 1 then {}",
        If::new_with_then_keyword(
            Equality::new(
                Identifier::new("foo").into_lazy_box(),
                EqualityOperator::Equal,
                Literal::Real(1.0).into_lazy_box(),
            )
            .into_lazy_box(),
            Block::new(vec![]).into_lazy_box(),
            None,
        ),
    )
}

#[test]
fn if_else() {
    harness_stmt(
        "if foo == 1 {} else {}",
        If::new_with_else(
            Equality::new(
                Identifier::new("foo").into_lazy_box(),
                EqualityOperator::Equal,
                Literal::Real(1.0).into_lazy_box(),
            )
            .into_lazy_box(),
            Block::new(vec![]).into_lazy_box(),
            Block::new(vec![]).into_lazy_box(),
        ),
    )
}

#[test]
fn switch() {
    harness_stmt(
        "switch foo {}",
        Switch::new(Identifier::new("foo").into_lazy_box(), vec![], None),
    )
}

#[test]
fn switch_with_case() {
    harness_stmt(
        "switch foo { case bar: break; }",
        Switch::new(
            Identifier::new("foo").into_lazy_box(),
            vec![SwitchCase::new(
                Identifier::new("bar").into_lazy_box(),
                vec![Statement::Break.into_lazy_box()],
            )],
            None,
        ),
    )
}

#[test]
fn switch_case_fallthrough() {
    harness_stmt(
        "switch foo { case bar: case baz: break; }",
        Switch::new(
            Identifier::new("foo").into_lazy_box(),
            vec![
                SwitchCase::new(Identifier::new("bar").into_lazy_box(), vec![]),
                SwitchCase::new(
                    Identifier::new("baz").into_lazy_box(),
                    vec![Statement::Break.into_lazy_box()],
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
        Switch::new(
            Identifier::new("foo").into_lazy_box(),
            vec![],
            Some(vec![Statement::Break.into_lazy_box()]),
        ),
    )
}

#[test]
fn empty_block() {
    harness_stmt("{}", Block::new(vec![]))
}

#[test]
fn block() {
    harness_stmt("{ return; }", Block::new(vec![Return::new(None).into_lazy_box()]))
}

#[test]
fn return_statement() {
    harness_stmt("return;", Return::new(None))
}

#[test]
fn return_with_value() {
    harness_stmt("return 0;", Return::new(Some(Literal::Real(0.0).into_lazy_box())))
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
