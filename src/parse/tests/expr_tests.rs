use crate::parse::*;
use pretty_assertions::assert_eq;

macro_rules! expr_test {
    ($name:ident, $source:expr, $expected:expr) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            let expected: ExprKind = $expected.into();
            let mut parser = Parser::new_with_default_ids($source, 0);
            let outputed = parser.expr().unwrap();
            assert_eq!(*outputed.kind(), expected, "`{}` failed!", $source)
        }
    };
}

expr_test!(
    function,
    "function foo() {}",
    Function::new(Identifier::lazy("foo"), vec![], Block::lazy(vec![]).into_stmt_lazy())
);

expr_test!(
    static_function,
    "static function foo() {}",
    Function::new(Identifier::lazy("foo"), vec![], Block::lazy(vec![]).into_stmt_lazy())
);

expr_test!(
    function_with_parameters,
    "function foo(bar, baz) {}",
    Function::new(
        Identifier::lazy("foo"),
        vec![
            Field::Uninitialized(Identifier::lazy("bar").into_expr_lazy()),
            Field::Uninitialized(Identifier::lazy("baz").into_expr_lazy()),
        ],
        Block::lazy(vec![]).into_stmt_lazy(),
    )
);

expr_test!(
    default_parameters,
    "function foo(bar=1, baz) {}",
    Function::new(
        Identifier::lazy("foo"),
        vec![
            Field::Initialized(
                Assignment::new(
                    Identifier::lazy("bar").into_expr_lazy(),
                    AssignmentOp::Identity(Token::lazy(TokenKind::Equal)),
                    Literal::Real(1.0).into_expr_lazy(),
                )
                .into_stmt_lazy(),
            ),
            Field::Uninitialized(Identifier::lazy("baz").into_expr_lazy()),
        ],
        Block::lazy(vec![]).into_stmt_lazy(),
    )
);

expr_test!(
    anonymous_function,
    "function() {}",
    Function::new_anonymous(vec![], Block::lazy(vec![]).into_stmt_lazy())
);

expr_test!(
    constructor,
    "function foo() constructor {}",
    Function::new_constructor(
        Some(Identifier::lazy("foo")),
        vec![],
        Constructor { inheritance: None },
        Block::lazy(vec![]).into_stmt_lazy(),
    )
);

expr_test!(
    inheritance,
    "function foo() : bar() constructor {}",
    Function::new_constructor(
        Some(Identifier::lazy("foo")),
        vec![],
        Constructor {
            inheritance: Some(Call::new(Identifier::lazy("bar").into_expr_lazy(), vec![]).into_expr_lazy())
        },
        Block::lazy(vec![]).into_stmt_lazy(),
    )
);

expr_test!(
    function_return_no_semi_colon,
    "function foo() { return }",
    Function::new(
        Identifier::lazy("foo"),
        vec![],
        Block::lazy(vec![Return::new(None).into_stmt_lazy()]).into_stmt_lazy(),
    )
);

expr_test!(
    and,
    "1 && 1",
    Logical::new(
        Literal::Real(1.0).into_expr_lazy(),
        LogicalOp::And(Token::lazy(TokenKind::DoubleAmpersand)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

expr_test!(
    and_keyword,
    "1 and 1",
    Logical::new(
        Literal::Real(1.0).into_expr_lazy(),
        LogicalOp::And(Token::lazy(TokenKind::And)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

expr_test!(
    or,
    "1 || 1",
    Logical::new(
        Literal::Real(1.0).into_expr_lazy(),
        LogicalOp::Or(Token::lazy(TokenKind::DoublePipe)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

expr_test!(
    or_keyword,
    "1 or 1",
    Logical::new(
        Literal::Real(1.0).into_expr_lazy(),
        LogicalOp::Or(Token::lazy(TokenKind::Or)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

expr_test!(
    xor,
    "1 xor 1",
    Logical::new(
        Literal::Real(1.0).into_expr_lazy(),
        LogicalOp::Xor(Token::lazy(TokenKind::Xor)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

expr_test!(
    addition,
    "1 + 1",
    Evaluation::new(
        Literal::Real(1.0).into_expr_lazy(),
        EvaluationOp::Plus(Token::lazy(TokenKind::Plus)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

expr_test!(
    subtraction,
    "1 - 1",
    Evaluation::new(
        Literal::Real(1.0).into_expr_lazy(),
        EvaluationOp::Minus(Token::lazy(TokenKind::Minus)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

expr_test!(
    multiplication,
    "1 * 1",
    Evaluation::new(
        Literal::Real(1.0).into_expr_lazy(),
        EvaluationOp::Star(Token::lazy(TokenKind::Star)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

expr_test!(
    division,
    "1 / 1",
    Evaluation::new(
        Literal::Real(1.0).into_expr_lazy(),
        EvaluationOp::Slash(Token::lazy(TokenKind::Slash)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

expr_test!(
    modulo,
    "1 mod 1",
    Evaluation::new(
        Literal::Real(1.0).into_expr_lazy(),
        EvaluationOp::Modulo(Token::lazy(TokenKind::Mod)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

expr_test!(
    modulo_alt,
    "1 % 1",
    Evaluation::new(
        Literal::Real(1.0).into_expr_lazy(),
        EvaluationOp::Modulo(Token::lazy(TokenKind::Percent)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

expr_test!(
    div,
    "1 div 1",
    Evaluation::new(
        Literal::Real(1.0).into_expr_lazy(),
        EvaluationOp::Div(Token::lazy(TokenKind::Div)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

expr_test!(
    bitwise_and,
    "1 & 1",
    Evaluation::new(
        Literal::Real(1.0).into_expr_lazy(),
        EvaluationOp::And(Token::lazy(TokenKind::Ampersand)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

expr_test!(
    bitwise_or,
    "1 | 1",
    Evaluation::new(
        Literal::Real(1.0).into_expr_lazy(),
        EvaluationOp::Or(Token::lazy(TokenKind::Pipe)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

expr_test!(
    bitwise_chain,
    "1 | 1 | 1",
    Evaluation::new(
        Literal::Real(1.0).into_expr_lazy(),
        EvaluationOp::Or(Token::lazy(TokenKind::Pipe)),
        Evaluation::new(
            Literal::Real(1.0).into_expr_lazy(),
            EvaluationOp::Or(Token::lazy(TokenKind::Pipe)),
            Literal::Real(1.0).into_expr_lazy(),
        )
        .into_expr_lazy(),
    )
);

expr_test!(
    bitwise_xor,
    "1 ^ 1",
    Evaluation::new(
        Literal::Real(1.0).into_expr_lazy(),
        EvaluationOp::Xor(Token::lazy(TokenKind::Caret)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

expr_test!(
    dot_access_bitwise,
    "foo.bar | foo.bar",
    Evaluation::new(
        Access::Dot {
            left: Identifier::lazy("foo").into_expr_lazy(),
            right: Identifier::lazy("bar"),
        }
        .into_expr_lazy(),
        EvaluationOp::Or(Token::lazy(TokenKind::Pipe)),
        Access::Dot {
            left: Identifier::lazy("foo").into_expr_lazy(),
            right: Identifier::lazy("bar"),
        }
        .into_expr_lazy(),
    )
);

expr_test!(
    bit_shift_left,
    "1 << 1",
    Evaluation::new(
        Literal::Real(1.0).into_expr_lazy(),
        EvaluationOp::BitShiftLeft(Token::lazy(TokenKind::BitShiftLeft)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

expr_test!(
    bit_shift_right,
    "1 >> 1",
    Evaluation::new(
        Literal::Real(1.0).into_expr_lazy(),
        EvaluationOp::BitShiftRight(Token::lazy(TokenKind::BitShiftRight)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

expr_test!(
    less_than,
    "1 < 1",
    Equality::new(
        Literal::Real(1.0).into_expr_lazy(),
        EqualityOp::LessThan(Token::lazy(TokenKind::LessThan)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

expr_test!(
    combo_math,
    "1 * 1 + 1 >> 1 & 1 == 1",
    Equality::new(
        Evaluation::new(
            Evaluation::new(
                Evaluation::new(
                    Evaluation::new(
                        Literal::Real(1.0).into_expr_lazy(),
                        EvaluationOp::Star(Token::lazy(TokenKind::Star)),
                        Literal::Real(1.0).into_expr_lazy(),
                    )
                    .into_expr_lazy(),
                    EvaluationOp::Plus(Token::lazy(TokenKind::Plus)),
                    Literal::Real(1.0).into_expr_lazy(),
                )
                .into_expr_lazy(),
                EvaluationOp::BitShiftRight(Token::lazy(TokenKind::BitShiftRight)),
                Literal::Real(1.0).into_expr_lazy(),
            )
            .into_expr_lazy(),
            EvaluationOp::And(Token::lazy(TokenKind::Ampersand)),
            Literal::Real(1.0).into_expr_lazy(),
        )
        .into_expr_lazy(),
        EqualityOp::Equal(Token::lazy(TokenKind::DoubleEqual)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

expr_test!(
    less_than_or_equal,
    "1 <= 1",
    Equality::new(
        Literal::Real(1.0).into_expr_lazy(),
        EqualityOp::LessThanOrEqual(Token::lazy(TokenKind::LessThanOrEqual)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

expr_test!(
    greater_than,
    "1 > 1",
    Equality::new(
        Literal::Real(1.0).into_expr_lazy(),
        EqualityOp::GreaterThan(Token::lazy(TokenKind::GreaterThan)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

expr_test!(
    greater_than_or_equal,
    "1 >= 1",
    Equality::new(
        Literal::Real(1.0).into_expr_lazy(),
        EqualityOp::GreaterThanOrEqual(Token::lazy(TokenKind::GreaterThanOrEqual)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

expr_test!(
    equal,
    "1 == 1",
    Equality::new(
        Literal::Real(1.0).into_expr_lazy(),
        EqualityOp::Equal(Token::lazy(TokenKind::DoubleEqual)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

expr_test!(
    colon_equal,
    "1 := 1",
    Equality::new(
        Literal::Real(1.0).into_expr_lazy(),
        EqualityOp::Equal(Token::lazy(TokenKind::ColonEqual)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

expr_test!(
    bang_equal,
    "1 != 1",
    Equality::new(
        Literal::Real(1.0).into_expr_lazy(),
        EqualityOp::NotEqual(Token::lazy(TokenKind::BangEqual)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

expr_test!(
    greater_than_less_than,
    "1 <> 1",
    Equality::new(
        Literal::Real(1.0).into_expr_lazy(),
        EqualityOp::NotEqual(Token::lazy(TokenKind::LessThanGreaterThan)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

expr_test!(
    null_coalecence,
    "foo ?? 1",
    NullCoalecence::new(
        Identifier::lazy("foo").into_expr_lazy(),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

expr_test!(
    ternary,
    "foo ? 1 : 2",
    Ternary::new(
        Identifier::lazy("foo").into_expr_lazy(),
        Literal::Real(1.0).into_expr_lazy(),
        Literal::Real(2.0).into_expr_lazy(),
    )
);

expr_test!(
    ternary_order_of_ops,
    "foo && bar ? 1 : 2",
    Ternary::new(
        Logical::new(
            Identifier::lazy("foo").into_expr_lazy(),
            LogicalOp::And(Token::lazy(TokenKind::DoubleAmpersand)),
            Identifier::lazy("bar").into_expr_lazy(),
        )
        .into_expr_lazy(),
        Literal::Real(1.0).into_expr_lazy(),
        Literal::Real(2.0).into_expr_lazy(),
    )
);

expr_test!(
    not,
    "!foo",
    Unary::new(
        UnaryOp::Not(Token::lazy(TokenKind::Bang)),
        Identifier::lazy("foo").into_expr_lazy(),
    )
);

expr_test!(
    not_keyword,
    "not foo",
    Unary::new(
        UnaryOp::Not(Token::lazy(TokenKind::Not)),
        Identifier::lazy("foo").into_expr_lazy(),
    )
);

expr_test!(
    positive,
    "+1",
    Unary::new(
        UnaryOp::Positive(Token::lazy(TokenKind::Plus)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

expr_test!(
    neagtive,
    "-1",
    Unary::new(
        UnaryOp::Negative(Token::lazy(TokenKind::Minus)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

expr_test!(
    dot_unary,
    "!self.foo",
    Unary::new(
        UnaryOp::Not(Token::lazy(TokenKind::Bang)),
        Access::Identity {
            right: Identifier::lazy("foo"),
        }
        .into_expr_lazy(),
    )
);

expr_test!(
    ds_unary,
    "!foo[bar]",
    Unary::new(
        UnaryOp::Not(Token::lazy(TokenKind::Bang)),
        Access::Array {
            left: Identifier::lazy("foo").into_expr_lazy(),
            index_one: Identifier::lazy("bar").into_expr_lazy(),
            index_two: None,
            using_accessor: false,
        }
        .into_expr_lazy(),
    )
);

expr_test!(
    prefix_increment,
    "++1",
    Unary::new(
        UnaryOp::Increment(Token::lazy(TokenKind::DoublePlus)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

expr_test!(
    prefix_decrement,
    "--1",
    Unary::new(
        UnaryOp::Decrement(Token::lazy(TokenKind::DoubleMinus)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

expr_test!(
    bitwise_not,
    "~1",
    Unary::new(
        UnaryOp::BitwiseNot(Token::lazy(TokenKind::Tilde)),
        Literal::Real(1.0).into_expr_lazy(),
    )
);

expr_test!(
    postfix_increment,
    "1++",
    Postfix::new(
        Literal::Real(1.0).into_expr_lazy(),
        PostfixOp::Increment(Token::lazy(TokenKind::DoublePlus)),
    )
);

expr_test!(
    postfix_decrement,
    "1--",
    Postfix::new(
        Literal::Real(1.0).into_expr_lazy(),
        PostfixOp::Decrement(Token::lazy(TokenKind::DoubleMinus)),
    )
);

expr_test!(
    dot_postfix,
    "self.foo++",
    Postfix::new(
        Access::Identity {
            right: Identifier::lazy("foo"),
        }
        .into_expr_lazy(),
        PostfixOp::Increment(Token::lazy(TokenKind::DoublePlus)),
    )
);

expr_test!(
    ds_postfix,
    "foo[bar]++",
    Postfix::new(
        Access::Array {
            left: Identifier::lazy("foo").into_expr_lazy(),
            index_one: Identifier::lazy("bar").into_expr_lazy(),
            index_two: None,
            using_accessor: false,
        }
        .into_expr_lazy(),
        PostfixOp::Increment(Token::lazy(TokenKind::DoublePlus)),
    )
);

expr_test!(
    call,
    "foo()",
    Call::new(Identifier::lazy("foo").into_expr_lazy(), vec![])
);

expr_test!(
    call_with_args,
    "foo(0, 1, 2)",
    Call::new(
        Identifier::lazy("foo").into_expr_lazy(),
        vec![
            Literal::Real(0.0).into_expr_lazy(),
            Literal::Real(1.0).into_expr_lazy(),
            Literal::Real(2.0).into_expr_lazy(),
        ],
    )
);

expr_test!(
    call_trailing_commas,
    "foo(0, 1, 2,)",
    Call::new(
        Identifier::lazy("foo").into_expr_lazy(),
        vec![
            Literal::Real(0.0).into_expr_lazy(),
            Literal::Real(1.0).into_expr_lazy(),
            Literal::Real(2.0).into_expr_lazy(),
        ],
    )
);

expr_test!(
    construction,
    "new foo()",
    Call::new_with_new_operator(Identifier::lazy("foo").into_expr_lazy(), vec![])
);

expr_test!(empty_array, "[]", ExprKind::Literal(Literal::Array(vec![])));

expr_test!(
    simple_array,
    "[0, 1, 2]",
    ExprKind::Literal(Literal::Array(vec![
        Literal::Real(0.0).into_expr_lazy(),
        Literal::Real(1.0).into_expr_lazy(),
        Literal::Real(2.0).into_expr_lazy(),
    ]))
);

expr_test!(empty_struct, "{}", ExprKind::Literal(Literal::Struct(vec![])));

expr_test!(
    struct_begin_end,
    "begin end",
    ExprKind::Literal(Literal::Struct(vec![]))
);

expr_test!(
    simple_struct,
    "{ foo: bar, fizz: buzz }",
    ExprKind::Literal(Literal::Struct(vec![
        (Identifier::lazy("foo"), Identifier::lazy("bar").into_expr_lazy()),
        (Identifier::lazy("fizz"), Identifier::lazy("buzz").into_expr_lazy()),
    ]))
);

expr_test!(
    array_access,
    "foo[bar]",
    Access::Array {
        left: Identifier::lazy("foo").into_expr_lazy(),
        index_one: Identifier::lazy("bar").into_expr_lazy(),
        index_two: None,
        using_accessor: false,
    }
);

expr_test!(
    array_direct_access,
    "foo[@ bar]",
    Access::Array {
        left: Identifier::lazy("foo").into_expr_lazy(),
        index_one: Identifier::lazy("bar").into_expr_lazy(),
        index_two: None,
        using_accessor: true,
    }
);

expr_test!(
    array_access_2d,
    "foo[bar, buzz]",
    Access::Array {
        left: Identifier::lazy("foo").into_expr_lazy(),
        index_one: Identifier::lazy("bar").into_expr_lazy(),
        index_two: Some(Identifier::lazy("buzz").into_expr_lazy()),
        using_accessor: false,
    }
);

expr_test!(
    ds_map_access,
    "foo[? bar]",
    Access::Map {
        left: Identifier::lazy("foo").into_expr_lazy(),
        key: Identifier::lazy("bar").into_expr_lazy(),
    }
);

expr_test!(
    ds_list_access,
    "foo[| bar]",
    Access::List {
        left: Identifier::lazy("foo").into_expr_lazy(),
        index: Identifier::lazy("bar").into_expr_lazy(),
    }
);

expr_test!(
    ds_grid_access,
    "foo[# bar, buzz]",
    Access::Grid {
        left: Identifier::lazy("foo").into_expr_lazy(),
        index_one: Identifier::lazy("bar").into_expr_lazy(),
        index_two: Identifier::lazy("buzz").into_expr_lazy(),
    }
);

expr_test!(
    ds_grid_access_no_space,
    "foo[#bar, buzz]",
    Access::Grid {
        left: Identifier::lazy("foo").into_expr_lazy(),
        index_one: Identifier::lazy("bar").into_expr_lazy(),
        index_two: Identifier::lazy("buzz").into_expr_lazy(),
    }
);

expr_test!(
    struct_access,
    "foo[$ bar]",
    Access::Struct {
        left: Identifier::lazy("foo").into_expr_lazy(),
        key: Identifier::lazy("bar").into_expr_lazy(),
    }
);

expr_test!(
    chained_ds_accesses,
    "foo[bar][buzz]",
    Access::Array {
        left: Access::Array {
            left: Identifier::lazy("foo").into_expr_lazy(),
            index_one: Identifier::lazy("bar").into_expr_lazy(),
            index_two: None,
            using_accessor: false,
        }
        .into_expr_lazy(),
        index_one: Identifier::lazy("buzz").into_expr_lazy(),
        index_two: None,
        using_accessor: false,
    }
);

expr_test!(
    ds_access_call,
    "foo[bar]()",
    Call::new(
        Access::Array {
            left: Identifier::lazy("foo").into_expr_lazy(),
            index_one: Identifier::lazy("bar").into_expr_lazy(),
            index_two: None,
            using_accessor: false,
        }
        .into_expr_lazy(),
        vec![],
    )
);

expr_test!(
    dot_access,
    "foo.bar",
    Access::Dot {
        left: Identifier::lazy("foo").into_expr_lazy(),
        right: Identifier::lazy("bar"),
    }
);

expr_test!(
    chained_dot_access,
    "foo.bar.buzz",
    Access::Dot {
        left: Access::Dot {
            left: Identifier::lazy("foo").into_expr_lazy(),
            right: Identifier::lazy("bar"),
        }
        .into_expr_lazy(),
        right: Identifier::lazy("buzz"),
    }
);

expr_test!(
    dot_access_to_call,
    "foo.bar()",
    Call::new(
        Access::Dot {
            left: Identifier::lazy("foo").into_expr_lazy(),
            right: Identifier::lazy("bar"),
        }
        .into_expr_lazy(),
        vec![],
    )
);

expr_test!(
    dot_access_to_ds_access,
    "foo.bar[0]",
    Access::Array {
        left: Access::Dot {
            left: Identifier::lazy("foo").into_expr_lazy(),
            right: Identifier::lazy("bar"),
        }
        .into_expr_lazy(),
        index_one: Literal::Real(0.0).into_expr_lazy(),
        index_two: None,
        using_accessor: false,
    }
);

expr_test!(
    dot_access_from_call,
    "foo().bar",
    Access::Dot {
        left: Call::new(Identifier::lazy("foo").into_expr_lazy(), vec![]).into_expr_lazy(),
        right: Identifier::lazy("bar"),
    }
);

expr_test!(
    chained_calls,
    "foo().bar()",
    Call::new(
        Access::Dot {
            left: Call::new(Identifier::lazy("foo").into_expr_lazy(), vec![]).into_expr_lazy(),
            right: Identifier::lazy("bar"),
        }
        .into_expr_lazy(),
        vec![],
    )
);

expr_test!(
    chain_calls_with_call_parameter,
    "foo().bar(buzz())",
    Call::new(
        Access::Dot {
            left: Call::new(Identifier::lazy("foo").into_expr_lazy(), vec![]).into_expr_lazy(),
            right: Identifier::lazy("bar"),
        }
        .into_expr_lazy(),
        vec![Call::new(Identifier::lazy("buzz").into_expr_lazy(), vec![]).into_expr_lazy()],
    )
);

expr_test!(
    global_dot_access,
    "global.bar",
    Access::Global {
        right: Identifier::lazy("bar"),
    }
);

expr_test!(
    self_dot_access,
    "self.bar",
    Access::Identity {
        right: Identifier::lazy("bar"),
    }
);

expr_test!(
    other_dot_access,
    "other.bar",
    Access::Other {
        right: Identifier::lazy("bar"),
    }
);

expr_test!(
    ds_dot_access,
    "foo[0].bar",
    Access::Dot {
        left: Access::Array {
            left: Identifier::lazy("foo").into_expr_lazy(),
            index_one: Literal::Real(0.0).into_expr_lazy(),
            index_two: None,
            using_accessor: false,
        }
        .into_expr_lazy(),
        right: Identifier::lazy("bar"),
    }
);

expr_test!(grouping, "(0)", Grouping::lazy(Literal::Real(0.0).into_expr_lazy()));

expr_test!(
    nested_grouping,
    "((0) * 0)",
    Grouping::lazy(
        Evaluation::new(
            Grouping::lazy(Literal::Real(0.0).into_expr_lazy()).into_expr_lazy(),
            EvaluationOp::Star(Token::lazy(TokenKind::Star)),
            Literal::Real(0.0).into_expr_lazy(),
        )
        .into_expr_lazy(),
    )
);

expr_test!(identifier, "foo", Identifier::lazy("foo"));

expr_test!(number, "0", Literal::Real(0.0));

expr_test!(float, "0.01", Literal::Real(0.01));

expr_test!(float_no_prefix, ".01", Literal::Real(0.01));

expr_test!(constant, "true", ExprKind::Literal(Literal::True));
expr_test!(constant_bool, "false", ExprKind::Literal(Literal::False));
expr_test!(undefined, "undefined", ExprKind::Literal(Literal::Undefined));
expr_test!(noone, "noone", ExprKind::Literal(Literal::Noone));
expr_test!(
    misc_literal,
    "browser_not_a_browser",
    ExprKind::Literal(Literal::Misc("browser_not_a_browser".into()))
);

expr_test!(string, "\"foo\"", ExprKind::Literal(Literal::String("foo".into())));

expr_test!(
    multi_line_string,
    "@\"\nfoo\nfoo\"",
    ExprKind::Literal(Literal::String("\nfoo\nfoo".into()))
);

expr_test!(
    multi_line_string_single_quote,
    "@'\nfoo\nfoo'",
    ExprKind::Literal(Literal::String("\nfoo\nfoo".into()))
);

// I hate gamemaker.
expr_test!(
    multi_line_string_single_quote_with_inner_double_quote,
    "@'\nfoo\"\nfoo'",
    ExprKind::Literal(Literal::String("\nfoo\"\nfoo".into()))
);

expr_test!(dollar_hex, "$a0f9a0", ExprKind::Literal(Literal::Hex("a0f9a0".into())));

expr_test!(short_hex, "$20", ExprKind::Literal(Literal::Hex("20".into())));

expr_test!(oh_x_hex, "0xa0f9a0", ExprKind::Literal(Literal::Hex("a0f9a0".into())));

expr_test!(
    logically_joined_expressions,
    "foo == 1 && foo == 1 && foo == 1",
    Logical::new(
        Equality::new(
            Identifier::lazy("foo").into_expr_lazy(),
            EqualityOp::Equal(Token::lazy(TokenKind::DoubleEqual)),
            Literal::Real(1.0).into_expr_lazy(),
        )
        .into_expr_lazy(),
        LogicalOp::And(Token::lazy(TokenKind::DoubleAmpersand)),
        Logical::new(
            Equality::new(
                Identifier::lazy("foo").into_expr_lazy(),
                EqualityOp::Equal(Token::lazy(TokenKind::DoubleEqual)),
                Literal::Real(1.0).into_expr_lazy(),
            )
            .into_expr_lazy(),
            LogicalOp::And(Token::lazy(TokenKind::DoubleAmpersand)),
            Equality::new(
                Identifier::lazy("foo").into_expr_lazy(),
                EqualityOp::Equal(Token::lazy(TokenKind::DoubleEqual)),
                Literal::Real(1.0).into_expr_lazy(),
            )
            .into_expr_lazy(),
        )
        .into_expr_lazy(),
    )
);

expr_test!(
    comment_in_builder_chain,
    "
            foo()
            // nothing in here!
            .bar()
        ",
    Call::new(
        Access::Dot {
            left: Call::new(Identifier::lazy("foo").into_expr_lazy(), vec![]).into_expr_lazy(),
            right: Identifier::lazy("bar"),
        }
        .into_expr_lazy(),
        vec![],
    )
);
