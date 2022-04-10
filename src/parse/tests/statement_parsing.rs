use crate::parse::*;
use pretty_assertions::assert_eq;

fn harness_stmt(source: &'static str, expected: impl Into<StmtType>) {
    let expected = expected.into();
    let mut parser = Parser::new_no_markers(source, 0);
    let outputed = parser.stmt().unwrap();
    assert_eq!(*outputed.inner(), expected)
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
            Identifier::lazy("i").into_expr_lazy(),
        )]),
    )
}

#[test]
fn local_variable_with_value() {
    harness_stmt(
        "var i = 0;",
        LocalVariableSeries::new(vec![OptionalInitilization::Initialized(
            Assignment::new(
                Identifier::lazy("i").into_expr_lazy(),
                AssignmentOp::Identity(Token::lazy(TokenType::Equal)),
                Literal::Real(0.0).into_expr_lazy(),
            )
            .into_stmt_lazy(),
        )]),
    )
}

#[test]
fn local_variable_series() {
    harness_stmt(
        "var i, j = 0, h;",
        LocalVariableSeries::new(vec![
            OptionalInitilization::Uninitialized(Identifier::lazy("i").into_expr_lazy()),
            OptionalInitilization::Initialized(
                Assignment::new(
                    Identifier::lazy("j").into_expr_lazy(),
                    AssignmentOp::Identity(Token::lazy(TokenType::Equal)),
                    Literal::Real(0.0).into_expr_lazy(),
                )
                .into_stmt_lazy(),
            ),
            OptionalInitilization::Uninitialized(Identifier::lazy("h").into_expr_lazy()),
        ]),
    )
}

#[test]
fn local_variable_trailling_comma() {
    harness_stmt(
        "var i = 0,",
        LocalVariableSeries::new(vec![OptionalInitilization::Initialized(
            Assignment::new(
                Identifier::lazy("i").into_expr_lazy(),
                AssignmentOp::Identity(Token::lazy(TokenType::Equal)),
                Literal::Real(0.0).into_expr_lazy(),
            )
            .into_stmt_lazy(),
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
                    Identifier::lazy("i").into_expr_lazy(),
                    AssignmentOp::Identity(Token::lazy(TokenType::Equal)),
                    Literal::Real(0.0).into_expr_lazy(),
                )
                .into_stmt_lazy(),
            )])
            .into_stmt_lazy(),
            Assignment::new(
                Identifier::lazy("j").into_expr_lazy(),
                AssignmentOp::Identity(Token::lazy(TokenType::Equal)),
                Literal::Real(0.0).into_expr_lazy(),
            )
            .into_stmt_lazy(),
        ]),
    )
}

#[test]
fn try_catch() {
    harness_stmt(
        "try {} catch (e) {}",
        TryCatch::new(
            Block::lazy(vec![]).into_stmt_lazy(),
            Grouping::lazy(Identifier::lazy("e").into_expr_lazy()).into_expr_lazy(),
            Block::lazy(vec![]).into_stmt_lazy(),
        ),
    )
}

#[test]
fn try_catch_finally() {
    harness_stmt(
        "try {} catch (e) {} finally {}",
        TryCatch::new_with_finally(
            Block::lazy(vec![]).into_stmt_lazy(),
            Grouping::lazy(Identifier::lazy("e").into_expr_lazy()).into_expr_lazy(),
            Block::lazy(vec![]).into_stmt_lazy(),
            Block::lazy(vec![]).into_stmt_lazy(),
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
                    Identifier::lazy("i").into_expr_lazy(),
                    AssignmentOp::Identity(Token::lazy(TokenType::Equal)),
                    Literal::Real(0.0).into_expr_lazy(),
                )
                .into_stmt_lazy(),
            )])
            .into_stmt_lazy(),
            Equality::new(
                Identifier::lazy("i").into_expr_lazy(),
                EqualityOp::LessThan(Token::lazy(TokenType::LessThan)),
                Literal::Real(1.0).into_expr_lazy(),
            )
            .into_expr_lazy(),
            StmtType::Expr(
                Postfix::new(
                    Identifier::lazy("i").into_expr_lazy(),
                    PostfixOp::Increment(Token::lazy(TokenType::DoublePlus)),
                )
                .into_expr_lazy(),
            )
            .into_stmt_lazy(),
            Block::lazy(vec![]).into_stmt_lazy(),
        ),
    );
}

#[test]
fn with() {
    harness_stmt(
        "with foo {}",
        WithLoop::new(
            Identifier::lazy("foo").into_expr_lazy(),
            Block::lazy(vec![]).into_stmt_lazy(),
        ),
    )
}

#[test]
fn repeat() {
    harness_stmt(
        "repeat 1 {}",
        RepeatLoop::new(
            Literal::Real(1.0).into_expr_lazy(),
            Block::lazy(vec![]).into_stmt_lazy(),
        ),
    )
}

#[test]
fn do_until() {
    harness_stmt(
        "do { foo += 1; } until foo == 1;",
        DoUntil::new(
            Block::lazy(vec![
                Assignment::new(
                    Identifier::lazy("foo").into_expr_lazy(),
                    AssignmentOp::PlusEqual(Token::lazy(TokenType::PlusEqual)),
                    Literal::Real(1.0).into_expr_lazy(),
                )
                .into_stmt_lazy(),
            ])
            .into_stmt_lazy(),
            Equality::new(
                Identifier::lazy("foo").into_expr_lazy(),
                EqualityOp::Equal(Token::lazy(TokenType::DoubleEqual)),
                Literal::Real(1.0).into_expr_lazy(),
            )
            .into_expr_lazy(),
        ),
    )
}
#[test]
fn while_loop() {
    harness_stmt(
        "while foo == 1 { foo += 1; }",
        If::new(
            Equality::new(
                Identifier::lazy("foo").into_expr_lazy(),
                EqualityOp::Equal(Token::lazy(TokenType::DoubleEqual)),
                Literal::Real(1.0).into_expr_lazy(),
            )
            .into_expr_lazy(),
            Block::lazy(vec![
                Assignment::new(
                    Identifier::lazy("foo").into_expr_lazy(),
                    AssignmentOp::PlusEqual(Token::lazy(TokenType::PlusEqual)),
                    Literal::Real(1.0).into_expr_lazy(),
                )
                .into_stmt_lazy(),
            ])
            .into_stmt_lazy(),
        ),
    )
}

#[test]
fn if_stmt() {
    harness_stmt(
        "if foo == 1 {}",
        If::new(
            Equality::new(
                Identifier::lazy("foo").into_expr_lazy(),
                EqualityOp::Equal(Token::lazy(TokenType::DoubleEqual)),
                Literal::Real(1.0).into_expr_lazy(),
            )
            .into_expr_lazy(),
            Block::lazy(vec![]).into_stmt_lazy(),
        ),
    )
}

#[test]
fn if_then() {
    harness_stmt(
        "if foo == 1 then {}",
        If::new_with_then_keyword(
            Equality::new(
                Identifier::lazy("foo").into_expr_lazy(),
                EqualityOp::Equal(Token::lazy(TokenType::DoubleEqual)),
                Literal::Real(1.0).into_expr_lazy(),
            )
            .into_expr_lazy(),
            Block::lazy(vec![]).into_stmt_lazy(),
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
                Identifier::lazy("foo").into_expr_lazy(),
                EqualityOp::Equal(Token::lazy(TokenType::DoubleEqual)),
                Literal::Real(1.0).into_expr_lazy(),
            )
            .into_expr_lazy(),
            Block::lazy(vec![]).into_stmt_lazy(),
            Block::lazy(vec![]).into_stmt_lazy(),
        ),
    )
}

#[test]
fn switch() {
    harness_stmt(
        "switch foo {}",
        Switch::new(Identifier::lazy("foo").into_expr_lazy(), vec![], None),
    )
}

#[test]
fn switch_with_case() {
    harness_stmt(
        "switch foo { case bar: break; }",
        Switch::new(
            Identifier::lazy("foo").into_expr_lazy(),
            vec![SwitchCase::new(
                Identifier::lazy("bar").into_expr_lazy(),
                vec![StmtType::Break.into_stmt_lazy()],
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
            Identifier::lazy("foo").into_expr_lazy(),
            vec![
                SwitchCase::new(Identifier::lazy("bar").into_expr_lazy(), vec![]),
                SwitchCase::new(
                    Identifier::lazy("baz").into_expr_lazy(),
                    vec![StmtType::Break.into_stmt_lazy()],
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
            Identifier::lazy("foo").into_expr_lazy(),
            vec![],
            Some(vec![StmtType::Break.into_stmt_lazy()]),
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
fn return_stmt() {
    harness_stmt("return;", Return::new(None))
}

#[test]
fn return_with_value() {
    harness_stmt("return 0;", Return::new(Some(Literal::Real(0.0).into_expr_lazy())))
}

#[test]
fn throw() {
    harness_stmt("throw foo;", Throw::new(Identifier::lazy("foo").into_expr_lazy()))
}

#[test]
fn delete() {
    harness_stmt("delete foo;", Delete::new(Identifier::lazy("foo").into_expr_lazy()))
}

#[test]
fn break_stmt() {
    harness_stmt("break;", StmtType::Break)
}

#[test]
fn exit() {
    harness_stmt("exit;", StmtType::Exit)
}

#[test]
fn excess_semicolons() {
    harness_stmt("exit;;;", StmtType::Exit)
}

#[test]
fn assign() {
    harness_stmt(
        "foo = 1",
        Assignment::new(
            Identifier::lazy("foo").into_expr_lazy(),
            AssignmentOp::Identity(Token::lazy(TokenType::Equal)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn single_equals_equality() {
    harness_stmt(
        "foo = bar = 1",
        Assignment::new(
            Identifier::lazy("foo").into_expr_lazy(),
            AssignmentOp::Identity(Token::lazy(TokenType::Equal)),
            Equality::new(
                Identifier::lazy("bar").into_expr_lazy(),
                EqualityOp::Equal(Token::lazy(TokenType::Equal)),
                Literal::Real(1.0).into_expr_lazy(),
            )
            .into_expr_lazy(),
        ),
    )
}

#[test]
fn function_assignment() {
    harness_stmt(
        "foo = function() {}",
        Assignment::new(
            Identifier::lazy("foo").into_expr_lazy(),
            AssignmentOp::Identity(Token::lazy(TokenType::Equal)),
            Function::new_anonymous(vec![], Block::lazy(vec![]).into_stmt_lazy()).into_expr_lazy(),
        ),
    );
}

#[test]
fn logical_assignment() {
    harness_stmt(
        "foo = 1 && 1",
        Assignment::new(
            Identifier::lazy("foo").into_expr_lazy(),
            AssignmentOp::Identity(Token::lazy(TokenType::Equal)),
            Logical::new(
                Literal::Real(1.0).into_expr_lazy(),
                LogicalOp::And(Token::lazy(TokenType::DoubleAmpersand)),
                Literal::Real(1.0).into_expr_lazy(),
            )
            .into_expr_lazy(),
        ),
    );
}

#[test]
fn ternary_assignment() {
    harness_stmt(
        "foo = bar ? 1 : 2;",
        Assignment::new(
            Identifier::lazy("foo").into_expr_lazy(),
            AssignmentOp::Identity(Token::lazy(TokenType::Equal)),
            Ternary::new(
                Identifier::lazy("bar").into_expr_lazy(),
                Literal::Real(1.0).into_expr_lazy(),
                Literal::Real(2.0).into_expr_lazy(),
            )
            .into_expr_lazy(),
        ),
    );
}

#[test]
fn null_coalecence_assign() {
    harness_stmt(
        "foo = bar ?? 0;",
        Assignment::new(
            Identifier::lazy("foo").into_expr_lazy(),
            AssignmentOp::Identity(Token::lazy(TokenType::Equal)),
            NullCoalecence::new(
                Identifier::lazy("bar").into_expr_lazy(),
                Literal::Real(0.0).into_expr_lazy(),
            )
            .into_expr_lazy(),
        ),
    );
}

#[test]
fn dot_assign() {
    harness_stmt(
        "self.foo = 1",
        Assignment::new(
            Access::Current {
                right: Identifier::lazy("foo"),
            }
            .into_expr_lazy(),
            AssignmentOp::Identity(Token::lazy(TokenType::Equal)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn ds_assign() {
    harness_stmt(
        "foo[0] = 1",
        Assignment::new(
            Access::Array {
                left: Identifier::lazy("foo").into_expr_lazy(),
                index_one: Literal::Real(0.0).into_expr_lazy(),
                index_two: None,
                using_accessor: false,
            }
            .into_expr_lazy(),
            AssignmentOp::Identity(Token::lazy(TokenType::Equal)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
// Valid GML, as much as it hurts. See `assignment_to_call`
fn call_assign() {
    harness_stmt(
        "foo() = 1",
        Assignment::new(
            Call::new(Identifier::lazy("foo").into_expr_lazy(), vec![]).into_expr_lazy(),
            AssignmentOp::Identity(Token::lazy(TokenType::Equal)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn static_assign() {
    harness_stmt(
        "static foo = 1",
        Assignment::new(
            Identifier::lazy("foo").into_expr_lazy(),
            AssignmentOp::Identity(Token::lazy(TokenType::Equal)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn plus_equal() {
    harness_stmt(
        "foo += 1",
        Assignment::new(
            Identifier::lazy("foo").into_expr_lazy(),
            AssignmentOp::PlusEqual(Token::lazy(TokenType::PlusEqual)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn minus_equal() {
    harness_stmt(
        "foo -= 1",
        Assignment::new(
            Identifier::lazy("foo").into_expr_lazy(),
            AssignmentOp::MinusEqual(Token::lazy(TokenType::MinusEqual)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn star_equal() {
    harness_stmt(
        "foo *= 1",
        Assignment::new(
            Identifier::lazy("foo").into_expr_lazy(),
            AssignmentOp::StarEqual(Token::lazy(TokenType::StarEqual)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn slash_equal() {
    harness_stmt(
        "foo /= 1",
        Assignment::new(
            Identifier::lazy("foo").into_expr_lazy(),
            AssignmentOp::SlashEqual(Token::lazy(TokenType::SlashEqual)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn and_equal() {
    harness_stmt(
        "foo &= 1",
        Assignment::new(
            Identifier::lazy("foo").into_expr_lazy(),
            AssignmentOp::AndEqual(Token::lazy(TokenType::AmpersandEqual)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn or_equal() {
    harness_stmt(
        "foo |= 1",
        Assignment::new(
            Identifier::lazy("foo").into_expr_lazy(),
            AssignmentOp::OrEqual(Token::lazy(TokenType::PipeEqual)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn xor_equal() {
    harness_stmt(
        "foo ^= 1",
        Assignment::new(
            Identifier::lazy("foo").into_expr_lazy(),
            AssignmentOp::XorEqual(Token::lazy(TokenType::CaretEquals)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn mod_equal() {
    harness_stmt(
        "foo %= 1",
        Assignment::new(
            Identifier::lazy("foo").into_expr_lazy(),
            AssignmentOp::ModEqual(Token::lazy(TokenType::PercentEqual)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn general_self_reference() {
    harness_stmt(
        "foo = self",
        Assignment::new(
            Identifier::lazy("foo").into_expr_lazy(),
            AssignmentOp::Identity(Token::lazy(TokenType::Equal)),
            Identifier::lazy("self").into_expr_lazy(),
        ),
    );
}

#[test]
fn general_other_reference() {
    harness_stmt(
        "foo = other",
        Assignment::new(
            Identifier::lazy("foo").into_expr_lazy(),
            AssignmentOp::Identity(Token::lazy(TokenType::Equal)),
            Identifier::lazy("other").into_expr_lazy(),
        ),
    );
}

#[test]
fn comment_above_statement() {
    harness_stmt(
        "
            // nothing in here!
            foo = bar;    
        ",
        Assignment::new(
            Identifier::lazy("foo").into_expr_lazy(),
            AssignmentOp::Identity(Token::lazy(TokenType::Equal)),
            Identifier::lazy("bar").into_expr_lazy(),
        ),
    );
}

#[test]
fn two_macro_declaration() {
    harness_stmt(
        "{ \n#macro foo 0\n#macro bar 0\n }",
        Block::lazy(vec![
            Macro::new(Identifier::lazy("foo"), "0")
                .into_expr_lazy()
                .into_stmt_lazy(),
            Macro::new(Identifier::lazy("bar"), "0")
                .into_expr_lazy()
                .into_stmt_lazy(),
        ]),
    )
}
