use crate::parse::*;
use pretty_assertions::assert_eq;

macro_rules! stmt_test {
    ($name:ident, $source:expr, $expected:expr) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            let expected: StmtKind = $expected.into();
            let mut parser = Parser::new_with_default_ids($source, 0);
            let outputed = parser.stmt().unwrap();
            assert_eq!(*outputed.inner(), expected, "`{}` failed!", $source)
        }
    };
}

stmt_test!(globalvar, "globalvar foo;", Globalvar::new(Identifier::lazy("foo")));

stmt_test!(
    local_variable,
    "var i;",
    LocalVariableSeries::new(vec![OptionalInitilization::Uninitialized(
        Identifier::lazy("i").into_expr_lazy(),
    )])
);

stmt_test!(
    local_variable_with_value,
    "var i = 0;",
    LocalVariableSeries::new(vec![OptionalInitilization::Initialized(
        Assignment::new(
            Identifier::lazy("i").into_expr_lazy(),
            AssignmentOp::Identity(Token::lazy(TokenKind::Equal)),
            Literal::Real(0.0).into_expr_lazy(),
        )
        .into_stmt_lazy(),
    )])
);

stmt_test!(
    local_variable_series,
    "var i, j = 0, h;",
    LocalVariableSeries::new(vec![
        OptionalInitilization::Uninitialized(Identifier::lazy("i").into_expr_lazy()),
        OptionalInitilization::Initialized(
            Assignment::new(
                Identifier::lazy("j").into_expr_lazy(),
                AssignmentOp::Identity(Token::lazy(TokenKind::Equal)),
                Literal::Real(0.0).into_expr_lazy(),
            )
            .into_stmt_lazy(),
        ),
        OptionalInitilization::Uninitialized(Identifier::lazy("h").into_expr_lazy()),
    ])
);

stmt_test!(
    local_variable_trailling_comma,
    "var i = 0,",
    LocalVariableSeries::new(vec![OptionalInitilization::Initialized(
        Assignment::new(
            Identifier::lazy("i").into_expr_lazy(),
            AssignmentOp::Identity(Token::lazy(TokenKind::Equal)),
            Literal::Real(0.0).into_expr_lazy(),
        )
        .into_stmt_lazy(),
    )])
);

stmt_test!(
    local_variable_series_ending_without_marker,
    "{ var i = 0 j = 0 }",
    Block::lazy(vec![
        LocalVariableSeries::new(vec![OptionalInitilization::Initialized(
            Assignment::new(
                Identifier::lazy("i").into_expr_lazy(),
                AssignmentOp::Identity(Token::lazy(TokenKind::Equal)),
                Literal::Real(0.0).into_expr_lazy(),
            )
            .into_stmt_lazy(),
        )])
        .into_stmt_lazy(),
        Assignment::new(
            Identifier::lazy("j").into_expr_lazy(),
            AssignmentOp::Identity(Token::lazy(TokenKind::Equal)),
            Literal::Real(0.0).into_expr_lazy(),
        )
        .into_stmt_lazy(),
    ])
);

stmt_test!(
    try_catch,
    "try {} catch (e) {}",
    TryCatch::new(
        Block::lazy(vec![]).into_stmt_lazy(),
        Grouping::lazy(Identifier::lazy("e").into_expr_lazy()).into_expr_lazy(),
        Block::lazy(vec![]).into_stmt_lazy(),
    )
);

stmt_test!(
    try_catch_finally,
    "try {} catch (e) {} finally {}",
    TryCatch::new_with_finally(
        Block::lazy(vec![]).into_stmt_lazy(),
        Grouping::lazy(Identifier::lazy("e").into_expr_lazy()).into_expr_lazy(),
        Block::lazy(vec![]).into_stmt_lazy(),
        Block::lazy(vec![]).into_stmt_lazy(),
    )
);

stmt_test!(
    for_loop,
    "for (var i = 0; i < 1; i++) {}",
    ForLoop::new(
        LocalVariableSeries::new(vec![OptionalInitilization::Initialized(
            Assignment::new(
                Identifier::lazy("i").into_expr_lazy(),
                AssignmentOp::Identity(Token::lazy(TokenKind::Equal)),
                Literal::Real(0.0).into_expr_lazy(),
            )
            .into_stmt_lazy(),
        )])
        .into_stmt_lazy(),
        Equality::new(
            Identifier::lazy("i").into_expr_lazy(),
            EqualityOp::LessThan(Token::lazy(TokenKind::LessThan)),
            Literal::Real(1.0).into_expr_lazy(),
        )
        .into_expr_lazy(),
        StmtKind::Expr(
            Postfix::new(
                Identifier::lazy("i").into_expr_lazy(),
                PostfixOp::Increment(Token::lazy(TokenKind::DoublePlus)),
            )
            .into_expr_lazy(),
        )
        .into_stmt_lazy(),
        Block::lazy(vec![]).into_stmt_lazy(),
    )
);

stmt_test!(
    with,
    "with foo {}",
    WithLoop::new(
        Identifier::lazy("foo").into_expr_lazy(),
        Block::lazy(vec![]).into_stmt_lazy(),
    )
);

stmt_test!(
    repeat,
    "repeat 1 {}",
    RepeatLoop::new(
        Literal::Real(1.0).into_expr_lazy(),
        Block::lazy(vec![]).into_stmt_lazy(),
    )
);

stmt_test!(
    do_until,
    "do { foo += 1; } until foo == 1;",
    DoUntil::new(
        Block::lazy(vec![
            Assignment::new(
                Identifier::lazy("foo").into_expr_lazy(),
                AssignmentOp::PlusEqual(Token::lazy(TokenKind::PlusEqual)),
                Literal::Real(1.0).into_expr_lazy(),
            )
            .into_stmt_lazy(),
        ])
        .into_stmt_lazy(),
        Equality::new(
            Identifier::lazy("foo").into_expr_lazy(),
            EqualityOp::Equal(Token::lazy(TokenKind::DoubleEqual)),
            Literal::Real(1.0).into_expr_lazy(),
        )
        .into_expr_lazy(),
    )
);
stmt_test!(
    while_loop,
    "while foo == 1 { foo += 1; }",
    If::new(
        Equality::new(
            Identifier::lazy("foo").into_expr_lazy(),
            EqualityOp::Equal(Token::lazy(TokenKind::DoubleEqual)),
            Literal::Real(1.0).into_expr_lazy(),
        )
        .into_expr_lazy(),
        Block::lazy(vec![
            Assignment::new(
                Identifier::lazy("foo").into_expr_lazy(),
                AssignmentOp::PlusEqual(Token::lazy(TokenKind::PlusEqual)),
                Literal::Real(1.0).into_expr_lazy(),
            )
            .into_stmt_lazy(),
        ])
        .into_stmt_lazy(),
    )
);

stmt_test!(
    if_stmt,
    "if foo == 1 {}",
    If::new(
        Equality::new(
            Identifier::lazy("foo").into_expr_lazy(),
            EqualityOp::Equal(Token::lazy(TokenKind::DoubleEqual)),
            Literal::Real(1.0).into_expr_lazy(),
        )
        .into_expr_lazy(),
        Block::lazy(vec![]).into_stmt_lazy(),
    )
);

stmt_test!(
    if_then,
    "if foo == 1 then {}",
    If::new_with_then_keyword(
        Equality::new(
            Identifier::lazy("foo").into_expr_lazy(),
            EqualityOp::Equal(Token::lazy(TokenKind::DoubleEqual)),
            Literal::Real(1.0).into_expr_lazy(),
        )
        .into_expr_lazy(),
        Block::lazy(vec![]).into_stmt_lazy(),
        None,
    )
);

stmt_test!(
    if_else,
    "if foo == 1 {} else {}",
    If::new_with_else(
        Equality::new(
            Identifier::lazy("foo").into_expr_lazy(),
            EqualityOp::Equal(Token::lazy(TokenKind::DoubleEqual)),
            Literal::Real(1.0).into_expr_lazy(),
        )
        .into_expr_lazy(),
        Block::lazy(vec![]).into_stmt_lazy(),
        Block::lazy(vec![]).into_stmt_lazy(),
    )
);

stmt_test!(
    switch,
    "switch foo {}",
    Switch::new(Identifier::lazy("foo").into_expr_lazy(), vec![], None)
);

stmt_test!(
    switch_with_case,
    "switch foo { case bar: break; }",
    Switch::new(
        Identifier::lazy("foo").into_expr_lazy(),
        vec![SwitchCase::new(
            Identifier::lazy("bar").into_expr_lazy(),
            vec![StmtKind::Break.into_stmt_lazy()],
        )],
        None,
    )
);

stmt_test!(
    switch_case_fallthrough,
    "switch foo { case bar: case baz: break; }",
    Switch::new(
        Identifier::lazy("foo").into_expr_lazy(),
        vec![
            SwitchCase::new(Identifier::lazy("bar").into_expr_lazy(), vec![]),
            SwitchCase::new(
                Identifier::lazy("baz").into_expr_lazy(),
                vec![StmtKind::Break.into_stmt_lazy()],
            ),
        ],
        None,
    )
);

stmt_test!(
    switch_default,
    "switch foo { default: break; }",
    Switch::new(
        Identifier::lazy("foo").into_expr_lazy(),
        vec![],
        Some(vec![StmtKind::Break.into_stmt_lazy()]),
    )
);

stmt_test!(block, "{}", Block::lazy(vec![]));

stmt_test!(
    block_begin_end,
    "begin end",
    Block::new(
        vec![],
        Some((Token::lazy(TokenKind::Begin), Token::lazy(TokenKind::End))),
    )
);

stmt_test!(return_stmt, "return;", Return::new(None));

stmt_test!(
    return_with_value,
    "return 0;",
    Return::new(Some(Literal::Real(0.0).into_expr_lazy()))
);

stmt_test!(
    throw,
    "throw foo;",
    Throw::new(Identifier::lazy("foo").into_expr_lazy())
);

stmt_test!(
    delete,
    "delete foo;",
    Delete::new(Identifier::lazy("foo").into_expr_lazy())
);

stmt_test!(break_stmt, "break;", StmtKind::Break);

stmt_test!(exit, "exit;", StmtKind::Exit);

stmt_test!(excess_semicolons, "exit;;;", StmtKind::Exit);

stmt_test!(
    assign,
    "foo = 1",
    Assignment::new(
        Identifier::lazy("foo").into_expr_lazy(),
        AssignmentOp::Identity(Token::lazy(TokenKind::Equal)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

stmt_test!(
    single_equals_equality,
    "foo = bar = 1",
    Assignment::new(
        Identifier::lazy("foo").into_expr_lazy(),
        AssignmentOp::Identity(Token::lazy(TokenKind::Equal)),
        Equality::new(
            Identifier::lazy("bar").into_expr_lazy(),
            EqualityOp::Equal(Token::lazy(TokenKind::Equal)),
            Literal::Real(1.0).into_expr_lazy(),
        )
        .into_expr_lazy(),
    )
);

stmt_test!(
    function_assignment,
    "foo = function() {}",
    Assignment::new(
        Identifier::lazy("foo").into_expr_lazy(),
        AssignmentOp::Identity(Token::lazy(TokenKind::Equal)),
        Function::new_anonymous(vec![], Block::lazy(vec![]).into_stmt_lazy()).into_expr_lazy(),
    )
);

stmt_test!(
    logical_assignment,
    "foo = 1 && 1",
    Assignment::new(
        Identifier::lazy("foo").into_expr_lazy(),
        AssignmentOp::Identity(Token::lazy(TokenKind::Equal)),
        Logical::new(
            Literal::Real(1.0).into_expr_lazy(),
            LogicalOp::And(Token::lazy(TokenKind::DoubleAmpersand)),
            Literal::Real(1.0).into_expr_lazy(),
        )
        .into_expr_lazy(),
    )
);

stmt_test!(
    ternary_assignment,
    "foo = bar ? 1 : 2;",
    Assignment::new(
        Identifier::lazy("foo").into_expr_lazy(),
        AssignmentOp::Identity(Token::lazy(TokenKind::Equal)),
        Ternary::new(
            Identifier::lazy("bar").into_expr_lazy(),
            Literal::Real(1.0).into_expr_lazy(),
            Literal::Real(2.0).into_expr_lazy(),
        )
        .into_expr_lazy(),
    )
);

stmt_test!(
    null_coalecence_assign,
    "foo = bar ?? 0;",
    Assignment::new(
        Identifier::lazy("foo").into_expr_lazy(),
        AssignmentOp::Identity(Token::lazy(TokenKind::Equal)),
        NullCoalecence::new(
            Identifier::lazy("bar").into_expr_lazy(),
            Literal::Real(0.0).into_expr_lazy(),
        )
        .into_expr_lazy(),
    )
);

stmt_test!(
    dot_assign,
    "self.foo = 1",
    Assignment::new(
        Access::Identity {
            right: Identifier::lazy("foo"),
        }
        .into_expr_lazy(),
        AssignmentOp::Identity(Token::lazy(TokenKind::Equal)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

stmt_test!(
    ds_assign,
    "foo[0] = 1",
    Assignment::new(
        Access::Array {
            left: Identifier::lazy("foo").into_expr_lazy(),
            index_one: Literal::Real(0.0).into_expr_lazy(),
            index_two: None,
            using_accessor: false,
        }
        .into_expr_lazy(),
        AssignmentOp::Identity(Token::lazy(TokenKind::Equal)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

// Valid GML, as much as it hurts. See `assignment_to_call`
stmt_test!(
    call_assignment,
    "foo() = 1",
    Assignment::new(
        Call::new(Identifier::lazy("foo").into_expr_lazy(), vec![]).into_expr_lazy(),
        AssignmentOp::Identity(Token::lazy(TokenKind::Equal)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

stmt_test!(
    static_assign,
    "static foo = 1",
    Assignment::new(
        Identifier::lazy("foo").into_expr_lazy(),
        AssignmentOp::Identity(Token::lazy(TokenKind::Equal)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

stmt_test!(
    plus_equal,
    "foo += 1",
    Assignment::new(
        Identifier::lazy("foo").into_expr_lazy(),
        AssignmentOp::PlusEqual(Token::lazy(TokenKind::PlusEqual)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

stmt_test!(
    minus_equal,
    "foo -= 1",
    Assignment::new(
        Identifier::lazy("foo").into_expr_lazy(),
        AssignmentOp::MinusEqual(Token::lazy(TokenKind::MinusEqual)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

stmt_test!(
    star_equal,
    "foo *= 1",
    Assignment::new(
        Identifier::lazy("foo").into_expr_lazy(),
        AssignmentOp::StarEqual(Token::lazy(TokenKind::StarEqual)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

stmt_test!(
    slash_equal,
    "foo /= 1",
    Assignment::new(
        Identifier::lazy("foo").into_expr_lazy(),
        AssignmentOp::SlashEqual(Token::lazy(TokenKind::SlashEqual)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

stmt_test!(
    and_equal,
    "foo &= 1",
    Assignment::new(
        Identifier::lazy("foo").into_expr_lazy(),
        AssignmentOp::AndEqual(Token::lazy(TokenKind::AmpersandEqual)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

stmt_test!(
    or_equal,
    "foo |= 1",
    Assignment::new(
        Identifier::lazy("foo").into_expr_lazy(),
        AssignmentOp::OrEqual(Token::lazy(TokenKind::PipeEqual)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

stmt_test!(
    xor_equal,
    "foo ^= 1",
    Assignment::new(
        Identifier::lazy("foo").into_expr_lazy(),
        AssignmentOp::XorEqual(Token::lazy(TokenKind::CaretEquals)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

stmt_test!(
    mod_equal,
    "foo %= 1",
    Assignment::new(
        Identifier::lazy("foo").into_expr_lazy(),
        AssignmentOp::ModEqual(Token::lazy(TokenKind::PercentEqual)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

stmt_test!(
    general_self_reference,
    "foo = self",
    Assignment::new(
        Identifier::lazy("foo").into_expr_lazy(),
        AssignmentOp::Identity(Token::lazy(TokenKind::Equal)),
        Identifier::lazy("self").into_expr_lazy(),
    )
);

stmt_test!(
    general_other_reference,
    "foo = other",
    Assignment::new(
        Identifier::lazy("foo").into_expr_lazy(),
        AssignmentOp::Identity(Token::lazy(TokenKind::Equal)),
        Identifier::lazy("other").into_expr_lazy(),
    )
);

stmt_test!(
    comment_above_statement,
    "
            // nothing in here!
            foo = bar;    
        ",
    Assignment::new(
        Identifier::lazy("foo").into_expr_lazy(),
        AssignmentOp::Identity(Token::lazy(TokenKind::Equal)),
        Identifier::lazy("bar").into_expr_lazy(),
    )
);

stmt_test!(
    two_macro_declaration,
    "{ \n#macro foo 0\n#macro bar 0\n }",
    Block::lazy(vec![
        Macro::new(Identifier::lazy("foo"), "0")
            .into_expr_lazy()
            .into_stmt_lazy(),
        Macro::new(Identifier::lazy("bar"), "0")
            .into_expr_lazy()
            .into_stmt_lazy(),
    ])
);
