use crate::parse::*;
use pretty_assertions::assert_eq;

fn harness_stmt(source: &'static str, expected: impl Into<Statement>) {
    let expected = expected.into();
    let mut parser = Parser::new(source, 0);
    let outputed = parser.statement().unwrap();
    assert_eq!(*outputed.statement(), expected)
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
        Block::lazy(vec![
            Macro::new("foo", "0").into_lazy_box(),
            Macro::new("bar", "0").into_lazy_box(),
        ]),
    )
}

#[test]
fn enum_declaration() {
    harness_stmt(
        "enum Foo { Bar, Baz }",
        Enum::new_with_members(
            Identifier::lazy("Foo"),
            vec![
                OptionalInitilization::Uninitialized(Identifier::lazy("Bar").into_lazy_box()),
                OptionalInitilization::Uninitialized(Identifier::lazy("Baz").into_lazy_box()),
            ],
        ),
    )
}

#[test]
fn enum_declaration_begin_end() {
    harness_stmt(
        "enum Foo begin Bar, Baz end",
        Enum::new_with_members(
            Identifier::lazy("Foo"),
            vec![
                OptionalInitilization::Uninitialized(Identifier::lazy("Bar").into_lazy_box()),
                OptionalInitilization::Uninitialized(Identifier::lazy("Baz").into_lazy_box()),
            ],
        ),
    )
}

#[test]
fn enum_with_values() {
    harness_stmt(
        "enum Foo { Bar = 20, Baz }",
        Enum::new_with_members(
            Identifier::lazy("Foo"),
            vec![
                OptionalInitilization::Initialized(
                    Assignment::new(
                        Identifier::lazy("Bar").into_lazy_box(),
                        AssignmentOperator::Equal(Token::lazy(TokenType::Equal)),
                        Literal::Real(20.0).into_lazy_box(),
                    )
                    .into_lazy_box(),
                ),
                OptionalInitilization::Uninitialized(Identifier::lazy("Baz").into_lazy_box()),
            ],
        ),
    )
}

#[test]
fn enum_with_neighbor_values() {
    harness_stmt(
        "enum Foo { Bar, Baz = Foo.Bar }",
        Enum::new_with_members(
            Identifier::lazy("Foo"),
            vec![
                OptionalInitilization::Uninitialized(Identifier::lazy("Bar").into_lazy_box()),
                OptionalInitilization::Initialized(
                    Assignment::new(
                        Identifier::lazy("Baz").into_lazy_box(),
                        AssignmentOperator::Equal(Token::lazy(TokenType::Equal)),
                        Access::Dot {
                            left: Identifier::lazy("Foo").into_lazy_box(),
                            right: Identifier::lazy("Bar").into_lazy_box(),
                        }
                        .into_lazy_box(),
                    )
                    .into_lazy_box(),
                ),
            ],
        ),
    )
}

#[test]
fn globalvar() {
    harness_stmt("globalvar foo;", Globalvar::new(Identifier::lazy("foo")))
}

#[test]
fn local_variable() {
    harness_stmt(
        "var i;",
        LocalVariableSeries::new(vec![OptionalInitilization::Uninitialized(
            Identifier::lazy("i").into_lazy_box(),
        )]),
    )
}

#[test]
fn local_variable_with_value() {
    harness_stmt(
        "var i = 0;",
        LocalVariableSeries::new(vec![OptionalInitilization::Initialized(
            Assignment::new(
                Identifier::lazy("i").into_lazy_box(),
                AssignmentOperator::Equal(Token::lazy(TokenType::Equal)),
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
            OptionalInitilization::Uninitialized(Identifier::lazy("i").into_lazy_box()),
            OptionalInitilization::Initialized(
                Assignment::new(
                    Identifier::lazy("j").into_lazy_box(),
                    AssignmentOperator::Equal(Token::lazy(TokenType::Equal)),
                    Literal::Real(0.0).into_lazy_box(),
                )
                .into_lazy_box(),
            ),
            OptionalInitilization::Uninitialized(Identifier::lazy("h").into_lazy_box()),
        ]),
    )
}

#[test]
fn local_variable_trailling_comma() {
    harness_stmt(
        "var i = 0,",
        LocalVariableSeries::new(vec![OptionalInitilization::Initialized(
            Assignment::new(
                Identifier::lazy("i").into_lazy_box(),
                AssignmentOperator::Equal(Token::lazy(TokenType::Equal)),
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
        Block::lazy(vec![
            LocalVariableSeries::new(vec![OptionalInitilization::Initialized(
                Assignment::new(
                    Identifier::lazy("i").into_lazy_box(),
                    AssignmentOperator::Equal(Token::lazy(TokenType::Equal)),
                    Literal::Real(0.0).into_lazy_box(),
                )
                .into_lazy_box(),
            )])
            .into_lazy_box(),
            Assignment::new(
                Identifier::lazy("j").into_lazy_box(),
                AssignmentOperator::Equal(Token::lazy(TokenType::Equal)),
                Literal::Real(0.0).into_lazy_box(),
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
            Block::lazy(vec![]).into_lazy_box(),
            Grouping::new(Identifier::lazy("e").into_lazy_box()).into_lazy_box(),
            Block::lazy(vec![]).into_lazy_box(),
        ),
    )
}

#[test]
fn try_catch_finally() {
    harness_stmt(
        "try {} catch (e) {} finally {}",
        TryCatch::new_with_finally(
            Block::lazy(vec![]).into_lazy_box(),
            Grouping::new(Identifier::lazy("e").into_lazy_box()).into_lazy_box(),
            Block::lazy(vec![]).into_lazy_box(),
            Block::lazy(vec![]).into_lazy_box(),
        ),
    )
}

#[test]
fn for_loop() {
    harness_stmt(
        "for (var i = 0; i < 1; i++) {}",
        ForLoop::new(
            LocalVariableSeries::new(vec![OptionalInitilization::Initialized(
                Assignment::new(
                    Identifier::lazy("i").into_lazy_box(),
                    AssignmentOperator::Equal(Token::lazy(TokenType::Equal)),
                    Literal::Real(0.0).into_lazy_box(),
                )
                .into_lazy_box(),
            )])
            .into_lazy_box(),
            Equality::new(
                Identifier::lazy("i").into_lazy_box(),
                EqualityOperator::LessThan(Token::lazy(TokenType::LessThan)),
                Literal::Real(1.0).into_lazy_box(),
            )
            .into_lazy_box(),
            Statement::Expression(
                Postfix::new(
                    Identifier::lazy("i").into_lazy_box(),
                    PostfixOperator::Increment(Token::lazy(TokenType::DoublePlus)),
                )
                .into_lazy_box(),
            )
            .into_lazy_box(),
            Block::lazy(vec![]).into_lazy_box(),
        ),
    );
}

#[test]
fn with() {
    harness_stmt(
        "with foo {}",
        WithLoop::new(
            Identifier::lazy("foo").into_lazy_box(),
            Block::lazy(vec![]).into_lazy_box(),
        ),
    )
}

#[test]
fn repeat() {
    harness_stmt(
        "repeat 1 {}",
        RepeatLoop::new(Literal::Real(1.0).into_lazy_box(), Block::lazy(vec![]).into_lazy_box()),
    )
}

#[test]
fn do_until() {
    harness_stmt(
        "do { foo += 1; } until foo == 1;",
        DoUntil::new(
            Block::lazy(vec![
                Assignment::new(
                    Identifier::lazy("foo").into_lazy_box(),
                    AssignmentOperator::PlusEqual(Token::lazy(TokenType::PlusEqual)),
                    Literal::Real(1.0).into_lazy_box(),
                )
                .into_lazy_box(),
            ])
            .into_lazy_box(),
            Equality::new(
                Identifier::lazy("foo").into_lazy_box(),
                EqualityOperator::Equal(Token::lazy(TokenType::DoubleEqual)),
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
                Identifier::lazy("foo").into_lazy_box(),
                EqualityOperator::Equal(Token::lazy(TokenType::DoubleEqual)),
                Literal::Real(1.0).into_lazy_box(),
            )
            .into_lazy_box(),
            Block::lazy(vec![
                Assignment::new(
                    Identifier::lazy("foo").into_lazy_box(),
                    AssignmentOperator::PlusEqual(Token::lazy(TokenType::PlusEqual)),
                    Literal::Real(1.0).into_lazy_box(),
                )
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
                Identifier::lazy("foo").into_lazy_box(),
                EqualityOperator::Equal(Token::lazy(TokenType::DoubleEqual)),
                Literal::Real(1.0).into_lazy_box(),
            )
            .into_lazy_box(),
            Block::lazy(vec![]).into_lazy_box(),
        ),
    )
}

#[test]
fn if_then() {
    harness_stmt(
        "if foo == 1 then {}",
        If::new_with_then_keyword(
            Equality::new(
                Identifier::lazy("foo").into_lazy_box(),
                EqualityOperator::Equal(Token::lazy(TokenType::DoubleEqual)),
                Literal::Real(1.0).into_lazy_box(),
            )
            .into_lazy_box(),
            Block::lazy(vec![]).into_lazy_box(),
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
                Identifier::lazy("foo").into_lazy_box(),
                EqualityOperator::Equal(Token::lazy(TokenType::DoubleEqual)),
                Literal::Real(1.0).into_lazy_box(),
            )
            .into_lazy_box(),
            Block::lazy(vec![]).into_lazy_box(),
            Block::lazy(vec![]).into_lazy_box(),
        ),
    )
}

#[test]
fn switch() {
    harness_stmt(
        "switch foo {}",
        Switch::new(Identifier::lazy("foo").into_lazy_box(), vec![], None),
    )
}

#[test]
fn switch_with_case() {
    harness_stmt(
        "switch foo { case bar: break; }",
        Switch::new(
            Identifier::lazy("foo").into_lazy_box(),
            vec![SwitchCase::new(
                Identifier::lazy("bar").into_lazy_box(),
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
            Identifier::lazy("foo").into_lazy_box(),
            vec![
                SwitchCase::new(Identifier::lazy("bar").into_lazy_box(), vec![]),
                SwitchCase::new(
                    Identifier::lazy("baz").into_lazy_box(),
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
            Identifier::lazy("foo").into_lazy_box(),
            vec![],
            Some(vec![Statement::Break.into_lazy_box()]),
        ),
    )
}

#[test]
fn block() {
    harness_stmt("{}", Block::lazy(vec![]))
}

#[test]
fn block_begin_end() {
    harness_stmt(
        "begin end",
        Block::new(
            vec![],
            Some((Token::lazy(TokenType::Begin), Token::lazy(TokenType::End))),
        ),
    )
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

#[test]
fn assign() {
    harness_stmt(
        "foo = 1",
        Assignment::new(
            Identifier::lazy("foo").into_lazy_box(),
            AssignmentOperator::Equal(Token::lazy(TokenType::Equal)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
}

#[test]
fn single_equals_equality() {
    harness_stmt(
        "foo = bar = 1",
        Assignment::new(
            Identifier::lazy("foo").into_lazy_box(),
            AssignmentOperator::Equal(Token::lazy(TokenType::Equal)),
            Equality::new(
                Identifier::lazy("bar").into_lazy_box(),
                EqualityOperator::Equal(Token::lazy(TokenType::Equal)),
                Literal::Real(1.0).into_lazy_box(),
            )
            .into_lazy_box(),
        ),
    )
}

#[test]
fn function_assignment() {
    harness_stmt(
        "foo = function() {}",
        Assignment::new(
            Identifier::lazy("foo").into_lazy_box(),
            AssignmentOperator::Equal(Token::lazy(TokenType::Equal)),
            Function::new_anonymous(vec![], Block::lazy(vec![]).into_lazy_box()).into_lazy_box(),
        ),
    );
}

#[test]
fn logical_assignment() {
    harness_stmt(
        "foo = 1 && 1",
        Assignment::new(
            Identifier::lazy("foo").into_lazy_box(),
            AssignmentOperator::Equal(Token::lazy(TokenType::Equal)),
            Logical::new(
                Literal::Real(1.0).into_lazy_box(),
                LogicalOperator::And(Token::lazy(TokenType::DoubleAmpersand)),
                Literal::Real(1.0).into_lazy_box(),
            )
            .into_lazy_box(),
        ),
    );
}

#[test]
fn ternary_assignment() {
    harness_stmt(
        "foo = bar ? 1 : 2;",
        Assignment::new(
            Identifier::lazy("foo").into_lazy_box(),
            AssignmentOperator::Equal(Token::lazy(TokenType::Equal)),
            Ternary::new(
                Identifier::lazy("bar").into_lazy_box(),
                Literal::Real(1.0).into_lazy_box(),
                Literal::Real(2.0).into_lazy_box(),
            )
            .into_lazy_box(),
        ),
    );
}

#[test]
fn null_coalecence_assign() {
    harness_stmt(
        "foo = bar ?? 0;",
        Assignment::new(
            Identifier::lazy("foo").into_lazy_box(),
            AssignmentOperator::Equal(Token::lazy(TokenType::Equal)),
            NullCoalecence::new(
                Identifier::lazy("bar").into_lazy_box(),
                Literal::Real(0.0).into_lazy_box(),
            )
            .into_lazy_box(),
        ),
    );
}

#[test]
fn dot_assign() {
    harness_stmt(
        "self.foo = 1",
        Assignment::new(
            Access::Current {
                right: Identifier::lazy("foo").into_lazy_box(),
            }
            .into_lazy_box(),
            AssignmentOperator::Equal(Token::lazy(TokenType::Equal)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
}

#[test]
fn ds_assign() {
    harness_stmt(
        "foo[0] = 1",
        Assignment::new(
            Access::Array {
                left: Identifier::lazy("foo").into_lazy_box(),
                index_one: Literal::Real(0.0).into_lazy_box(),
                index_two: None,
                using_accessor: false,
            }
            .into_lazy_box(),
            AssignmentOperator::Equal(Token::lazy(TokenType::Equal)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
}

#[test]
// Valid GML, as much as it hurts. See `assignment_to_call`
fn call_assign() {
    harness_stmt(
        "foo() = 1",
        Assignment::new(
            Call::new(Identifier::lazy("foo").into_lazy_box(), vec![]).into_lazy_box(),
            AssignmentOperator::Equal(Token::lazy(TokenType::Equal)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
}

#[test]
fn static_assign() {
    harness_stmt(
        "static foo = 1",
        Assignment::new(
            Identifier::lazy("foo").into_lazy_box(),
            AssignmentOperator::Equal(Token::lazy(TokenType::Equal)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
}

#[test]
fn plus_equal() {
    harness_stmt(
        "foo += 1",
        Assignment::new(
            Identifier::lazy("foo").into_lazy_box(),
            AssignmentOperator::PlusEqual(Token::lazy(TokenType::PlusEqual)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
}

#[test]
fn minus_equal() {
    harness_stmt(
        "foo -= 1",
        Assignment::new(
            Identifier::lazy("foo").into_lazy_box(),
            AssignmentOperator::MinusEqual(Token::lazy(TokenType::MinusEqual)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
}

#[test]
fn star_equal() {
    harness_stmt(
        "foo *= 1",
        Assignment::new(
            Identifier::lazy("foo").into_lazy_box(),
            AssignmentOperator::StarEqual(Token::lazy(TokenType::StarEqual)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
}

#[test]
fn slash_equal() {
    harness_stmt(
        "foo /= 1",
        Assignment::new(
            Identifier::lazy("foo").into_lazy_box(),
            AssignmentOperator::SlashEqual(Token::lazy(TokenType::SlashEqual)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
}

#[test]
fn and_equal() {
    harness_stmt(
        "foo &= 1",
        Assignment::new(
            Identifier::lazy("foo").into_lazy_box(),
            AssignmentOperator::AndEqual(Token::lazy(TokenType::AmpersandEqual)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
}

#[test]
fn or_equal() {
    harness_stmt(
        "foo |= 1",
        Assignment::new(
            Identifier::lazy("foo").into_lazy_box(),
            AssignmentOperator::OrEqual(Token::lazy(TokenType::PipeEqual)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
}

#[test]
fn xor_equal() {
    harness_stmt(
        "foo ^= 1",
        Assignment::new(
            Identifier::lazy("foo").into_lazy_box(),
            AssignmentOperator::XorEqual(Token::lazy(TokenType::CirumflexEqual)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
}

#[test]
fn mod_equal() {
    harness_stmt(
        "foo %= 1",
        Assignment::new(
            Identifier::lazy("foo").into_lazy_box(),
            AssignmentOperator::ModEqual(Token::lazy(TokenType::PercentEqual)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
}

#[test]
fn general_self_reference() {
    harness_stmt(
        "foo = self",
        Assignment::new(
            Identifier::lazy("foo").into_lazy_box(),
            AssignmentOperator::Equal(Token::lazy(TokenType::Equal)),
            Identifier::lazy("self").into_lazy_box(),
        ),
    );
}

#[test]
fn general_other_reference() {
    harness_stmt(
        "foo = other",
        Assignment::new(
            Identifier::lazy("foo").into_lazy_box(),
            AssignmentOperator::Equal(Token::lazy(TokenType::Equal)),
            Identifier::lazy("other").into_lazy_box(),
        ),
    );
}
