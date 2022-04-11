use crate::parse::*;
use pretty_assertions::assert_eq;

fn harness_expr(source: &'static str, expected: impl Into<ExprType>) {
    let expected = expected.into();
    let mut parser = Parser::new_with_default_ids(source, 0);
    let outputed = parser.expr().unwrap();
    assert_eq!(*outputed.inner(), expected, "`{}` failed!", source)
}

#[test]
fn enum_declaration() {
    harness_expr(
        "enum Foo { Bar, Baz }",
        Enum::new_with_members(
            Identifier::lazy("Foo"),
            vec![
                OptionalInitilization::Uninitialized(Identifier::lazy("Bar").into_expr_lazy()),
                OptionalInitilization::Uninitialized(Identifier::lazy("Baz").into_expr_lazy()),
            ],
        ),
    )
}

#[test]
fn enum_declaration_begin_end() {
    harness_expr(
        "enum Foo begin Bar, Baz end",
        Enum::new_with_members(
            Identifier::lazy("Foo"),
            vec![
                OptionalInitilization::Uninitialized(Identifier::lazy("Bar").into_expr_lazy()),
                OptionalInitilization::Uninitialized(Identifier::lazy("Baz").into_expr_lazy()),
            ],
        ),
    )
}

#[test]
fn enum_with_values() {
    harness_expr(
        "enum Foo { Bar = 20, Baz }",
        Enum::new_with_members(
            Identifier::lazy("Foo"),
            vec![
                OptionalInitilization::Initialized(
                    Assignment::new(
                        Identifier::lazy("Bar").into_expr_lazy(),
                        AssignmentOp::Identity(Token::lazy(TokenType::Equal)),
                        Literal::Real(20.0).into_expr_lazy(),
                    )
                    .into_stmt_lazy(),
                ),
                OptionalInitilization::Uninitialized(Identifier::lazy("Baz").into_expr_lazy()),
            ],
        ),
    )
}

#[test]
fn enum_with_neighbor_values() {
    harness_expr(
        "enum Foo { Bar, Baz = Foo.Bar }",
        Enum::new_with_members(
            Identifier::lazy("Foo"),
            vec![
                OptionalInitilization::Uninitialized(Identifier::lazy("Bar").into_expr_lazy()),
                OptionalInitilization::Initialized(
                    Assignment::new(
                        Identifier::lazy("Baz").into_expr_lazy(),
                        AssignmentOp::Identity(Token::lazy(TokenType::Equal)),
                        Access::Dot {
                            left: Identifier::lazy("Foo").into_expr_lazy(),
                            right: Identifier::lazy("Bar"),
                        }
                        .into_expr_lazy(),
                    )
                    .into_stmt_lazy(),
                ),
            ],
        ),
    )
}

#[test]
fn macro_declaration() {
    harness_expr(
        "#macro foo 0",
        ExprType::Macro(Macro::new(Identifier::lazy("foo"), "0")),
    )
}

#[test]
fn config_macro() {
    harness_expr(
        "#macro bar:foo 0",
        Macro::new_with_config(Identifier::lazy("foo"), "0", "bar"),
    )
}

#[test]
fn function() {
    harness_expr(
        "function foo() {}",
        Function::new(Identifier::lazy("foo"), vec![], Block::lazy(vec![]).into_stmt_lazy()),
    )
}

#[test]
fn static_function() {
    harness_expr(
        "static function foo() {}",
        Function::new(Identifier::lazy("foo"), vec![], Block::lazy(vec![]).into_stmt_lazy()),
    )
}

#[test]
fn function_with_parameters() {
    harness_expr(
        "function foo(bar, baz) {}",
        Function::new(
            Identifier::lazy("foo"),
            vec![
                OptionalInitilization::Uninitialized(Identifier::lazy("bar").into_expr_lazy()),
                OptionalInitilization::Uninitialized(Identifier::lazy("baz").into_expr_lazy()),
            ],
            Block::lazy(vec![]).into_stmt_lazy(),
        ),
    )
}

#[test]
fn default_parameters() {
    harness_expr(
        "function foo(bar=1, baz) {}",
        Function::new(
            Identifier::lazy("foo"),
            vec![
                OptionalInitilization::Initialized(
                    Assignment::new(
                        Identifier::lazy("bar").into_expr_lazy(),
                        AssignmentOp::Identity(Token::lazy(TokenType::Equal)),
                        Literal::Real(1.0).into_expr_lazy(),
                    )
                    .into_stmt_lazy(),
                ),
                OptionalInitilization::Uninitialized(Identifier::lazy("baz").into_expr_lazy()),
            ],
            Block::lazy(vec![]).into_stmt_lazy(),
        ),
    )
}

#[test]
fn anonymous_function() {
    harness_expr(
        "function() {}",
        Function::new_anonymous(vec![], Block::lazy(vec![]).into_stmt_lazy()),
    )
}

#[test]
fn constructor() {
    harness_expr(
        "function foo() constructor {}",
        Function::new_constructor(
            Some(Identifier::lazy("foo")),
            vec![],
            Constructor::WithoutInheritance,
            Block::lazy(vec![]).into_stmt_lazy(),
        ),
    )
}

#[test]
fn inheritance() {
    harness_expr(
        "function foo() : bar() constructor {}",
        Function::new_constructor(
            Some(Identifier::lazy("foo")),
            vec![],
            Constructor::WithInheritance(Call::new(Identifier::lazy("bar").into_expr_lazy(), vec![]).into_expr_lazy()),
            Block::lazy(vec![]).into_stmt_lazy(),
        ),
    )
}

#[test]
fn function_return_no_semi_colon() {
    harness_expr(
        "function foo() { return }",
        Function::new(
            Identifier::lazy("foo"),
            vec![],
            Block::lazy(vec![Return::new(None).into_stmt_lazy()]).into_stmt_lazy(),
        ),
    )
}

#[test]
fn and() {
    harness_expr(
        "1 && 1",
        Logical::new(
            Literal::Real(1.0).into_expr_lazy(),
            LogicalOp::And(Token::lazy(TokenType::DoubleAmpersand)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn and_keyword() {
    harness_expr(
        "1 and 1",
        Logical::new(
            Literal::Real(1.0).into_expr_lazy(),
            LogicalOp::And(Token::lazy(TokenType::And)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn or() {
    harness_expr(
        "1 || 1",
        Logical::new(
            Literal::Real(1.0).into_expr_lazy(),
            LogicalOp::Or(Token::lazy(TokenType::DoublePipe)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn or_keyword() {
    harness_expr(
        "1 or 1",
        Logical::new(
            Literal::Real(1.0).into_expr_lazy(),
            LogicalOp::Or(Token::lazy(TokenType::Or)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn xor() {
    harness_expr(
        "1 xor 1",
        Logical::new(
            Literal::Real(1.0).into_expr_lazy(),
            LogicalOp::Xor(Token::lazy(TokenType::Xor)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn addition() {
    harness_expr(
        "1 + 1",
        Evaluation::new(
            Literal::Real(1.0).into_expr_lazy(),
            EvaluationOp::Plus(Token::lazy(TokenType::Plus)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn subtraction() {
    harness_expr(
        "1 - 1",
        Evaluation::new(
            Literal::Real(1.0).into_expr_lazy(),
            EvaluationOp::Minus(Token::lazy(TokenType::Minus)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn multiplication() {
    harness_expr(
        "1 * 1",
        Evaluation::new(
            Literal::Real(1.0).into_expr_lazy(),
            EvaluationOp::Star(Token::lazy(TokenType::Star)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn division() {
    harness_expr(
        "1 / 1",
        Evaluation::new(
            Literal::Real(1.0).into_expr_lazy(),
            EvaluationOp::Slash(Token::lazy(TokenType::Slash)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn modulo() {
    harness_expr(
        "1 mod 1",
        Evaluation::new(
            Literal::Real(1.0).into_expr_lazy(),
            EvaluationOp::Modulo(Token::lazy(TokenType::Mod)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
    harness_expr(
        "1 % 1",
        Evaluation::new(
            Literal::Real(1.0).into_expr_lazy(),
            EvaluationOp::Modulo(Token::lazy(TokenType::Percent)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn div() {
    harness_expr(
        "1 div 1",
        Evaluation::new(
            Literal::Real(1.0).into_expr_lazy(),
            EvaluationOp::Div(Token::lazy(TokenType::Div)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn bitwise_and() {
    harness_expr(
        "1 & 1",
        Evaluation::new(
            Literal::Real(1.0).into_expr_lazy(),
            EvaluationOp::And(Token::lazy(TokenType::Ampersand)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn bitwise_or() {
    harness_expr(
        "1 | 1",
        Evaluation::new(
            Literal::Real(1.0).into_expr_lazy(),
            EvaluationOp::Or(Token::lazy(TokenType::Pipe)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn bitwise_chain() {
    harness_expr(
        "1 | 1 | 1",
        Evaluation::new(
            Literal::Real(1.0).into_expr_lazy(),
            EvaluationOp::Or(Token::lazy(TokenType::Pipe)),
            Evaluation::new(
                Literal::Real(1.0).into_expr_lazy(),
                EvaluationOp::Or(Token::lazy(TokenType::Pipe)),
                Literal::Real(1.0).into_expr_lazy(),
            )
            .into_expr_lazy(),
        ),
    );
}

#[test]
fn bitwise_xor() {
    harness_expr(
        "1 ^ 1",
        Evaluation::new(
            Literal::Real(1.0).into_expr_lazy(),
            EvaluationOp::Xor(Token::lazy(TokenType::Caret)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn dot_access_bitwise() {
    harness_expr(
        "foo.bar | foo.bar",
        Evaluation::new(
            Access::Dot {
                left: Identifier::lazy("foo").into_expr_lazy(),
                right: Identifier::lazy("bar"),
            }
            .into_expr_lazy(),
            EvaluationOp::Or(Token::lazy(TokenType::Pipe)),
            Access::Dot {
                left: Identifier::lazy("foo").into_expr_lazy(),
                right: Identifier::lazy("bar"),
            }
            .into_expr_lazy(),
        ),
    );
}

#[test]
fn bit_shift_left() {
    harness_expr(
        "1 << 1",
        Evaluation::new(
            Literal::Real(1.0).into_expr_lazy(),
            EvaluationOp::BitShiftLeft(Token::lazy(TokenType::BitShiftLeft)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn bit_shift_right() {
    harness_expr(
        "1 >> 1",
        Evaluation::new(
            Literal::Real(1.0).into_expr_lazy(),
            EvaluationOp::BitShiftRight(Token::lazy(TokenType::BitShiftRight)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn less_than() {
    harness_expr(
        "1 < 1",
        Equality::new(
            Literal::Real(1.0).into_expr_lazy(),
            EqualityOp::LessThan(Token::lazy(TokenType::LessThan)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn combo_math() {
    harness_expr(
        "1 * 1 + 1 >> 1 & 1 == 1",
        Equality::new(
            Evaluation::new(
                Evaluation::new(
                    Evaluation::new(
                        Evaluation::new(
                            Literal::Real(1.0).into_expr_lazy(),
                            EvaluationOp::Star(Token::lazy(TokenType::Star)),
                            Literal::Real(1.0).into_expr_lazy(),
                        )
                        .into_expr_lazy(),
                        EvaluationOp::Plus(Token::lazy(TokenType::Plus)),
                        Literal::Real(1.0).into_expr_lazy(),
                    )
                    .into_expr_lazy(),
                    EvaluationOp::BitShiftRight(Token::lazy(TokenType::BitShiftRight)),
                    Literal::Real(1.0).into_expr_lazy(),
                )
                .into_expr_lazy(),
                EvaluationOp::And(Token::lazy(TokenType::Ampersand)),
                Literal::Real(1.0).into_expr_lazy(),
            )
            .into_expr_lazy(),
            EqualityOp::Equal(Token::lazy(TokenType::DoubleEqual)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    )
}

#[test]
fn less_than_or_equal() {
    harness_expr(
        "1 <= 1",
        Equality::new(
            Literal::Real(1.0).into_expr_lazy(),
            EqualityOp::LessThanOrEqual(Token::lazy(TokenType::LessThanOrEqual)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn greater_than() {
    harness_expr(
        "1 > 1",
        Equality::new(
            Literal::Real(1.0).into_expr_lazy(),
            EqualityOp::GreaterThan(Token::lazy(TokenType::GreaterThan)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn greater_than_or_equal() {
    harness_expr(
        "1 >= 1",
        Equality::new(
            Literal::Real(1.0).into_expr_lazy(),
            EqualityOp::GreaterThanOrEqual(Token::lazy(TokenType::GreaterThanOrEqual)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn equal() {
    harness_expr(
        "1 == 1",
        Equality::new(
            Literal::Real(1.0).into_expr_lazy(),
            EqualityOp::Equal(Token::lazy(TokenType::DoubleEqual)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn colon_equal() {
    harness_expr(
        "1 := 1",
        Equality::new(
            Literal::Real(1.0).into_expr_lazy(),
            EqualityOp::Equal(Token::lazy(TokenType::ColonEqual)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn bang_equal() {
    harness_expr(
        "1 != 1",
        Equality::new(
            Literal::Real(1.0).into_expr_lazy(),
            EqualityOp::NotEqual(Token::lazy(TokenType::BangEqual)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn greater_than_less_than() {
    harness_expr(
        "1 <> 1",
        Equality::new(
            Literal::Real(1.0).into_expr_lazy(),
            EqualityOp::NotEqual(Token::lazy(TokenType::LessThanGreaterThan)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn null_coalecence() {
    harness_expr(
        "foo ?? 1",
        NullCoalecence::new(
            Identifier::lazy("foo").into_expr_lazy(),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn ternary() {
    harness_expr(
        "foo ? 1 : 2",
        Ternary::new(
            Identifier::lazy("foo").into_expr_lazy(),
            Literal::Real(1.0).into_expr_lazy(),
            Literal::Real(2.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn ternary_order_of_ops() {
    harness_expr(
        "foo && bar ? 1 : 2",
        Ternary::new(
            Logical::new(
                Identifier::lazy("foo").into_expr_lazy(),
                LogicalOp::And(Token::lazy(TokenType::DoubleAmpersand)),
                Identifier::lazy("bar").into_expr_lazy(),
            )
            .into_expr_lazy(),
            Literal::Real(1.0).into_expr_lazy(),
            Literal::Real(2.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn not() {
    harness_expr(
        "!foo",
        Unary::new(
            UnaryOp::Not(Token::lazy(TokenType::Bang)),
            Identifier::lazy("foo").into_expr_lazy(),
        ),
    );
}

#[test]
fn not_keyword() {
    harness_expr(
        "not foo",
        Unary::new(
            UnaryOp::Not(Token::lazy(TokenType::Not)),
            Identifier::lazy("foo").into_expr_lazy(),
        ),
    );
}

#[test]
fn positive() {
    harness_expr(
        "+1",
        Unary::new(
            UnaryOp::Positive(Token::lazy(TokenType::Plus)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn neagtive() {
    harness_expr(
        "-1",
        Unary::new(
            UnaryOp::Negative(Token::lazy(TokenType::Minus)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn dot_unary() {
    harness_expr(
        "!self.foo",
        Unary::new(
            UnaryOp::Not(Token::lazy(TokenType::Bang)),
            Access::Current {
                right: Identifier::lazy("foo"),
            }
            .into_expr_lazy(),
        ),
    );
}

#[test]
fn ds_unary() {
    harness_expr(
        "!foo[bar]",
        Unary::new(
            UnaryOp::Not(Token::lazy(TokenType::Bang)),
            Access::Array {
                left: Identifier::lazy("foo").into_expr_lazy(),
                index_one: Identifier::lazy("bar").into_expr_lazy(),
                index_two: None,
                using_accessor: false,
            }
            .into_expr_lazy(),
        ),
    );
}

#[test]
fn prefix_increment() {
    harness_expr(
        "++1",
        Unary::new(
            UnaryOp::Increment(Token::lazy(TokenType::DoublePlus)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn prefix_decrement() {
    harness_expr(
        "--1",
        Unary::new(
            UnaryOp::Decrement(Token::lazy(TokenType::DoubleMinus)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn bitwise_not() {
    harness_expr(
        "~1",
        Unary::new(
            UnaryOp::BitwiseNot(Token::lazy(TokenType::Tilde)),
            Literal::Real(1.0).into_expr_lazy(),
        ),
    );
}

#[test]
fn postfix_increment() {
    harness_expr(
        "1++",
        Postfix::new(
            Literal::Real(1.0).into_expr_lazy(),
            PostfixOp::Increment(Token::lazy(TokenType::DoublePlus)),
        ),
    );
}

#[test]
fn postfix_decrement() {
    harness_expr(
        "1--",
        Postfix::new(
            Literal::Real(1.0).into_expr_lazy(),
            PostfixOp::Decrement(Token::lazy(TokenType::DoubleMinus)),
        ),
    );
}

#[test]
fn dot_postfix() {
    harness_expr(
        "self.foo++",
        Postfix::new(
            Access::Current {
                right: Identifier::lazy("foo"),
            }
            .into_expr_lazy(),
            PostfixOp::Increment(Token::lazy(TokenType::DoublePlus)),
        ),
    );
}

#[test]
fn ds_postfix() {
    harness_expr(
        "foo[bar]++",
        Postfix::new(
            Access::Array {
                left: Identifier::lazy("foo").into_expr_lazy(),
                index_one: Identifier::lazy("bar").into_expr_lazy(),
                index_two: None,
                using_accessor: false,
            }
            .into_expr_lazy(),
            PostfixOp::Increment(Token::lazy(TokenType::DoublePlus)),
        ),
    );
}

#[test]
fn call() {
    harness_expr("foo()", Call::new(Identifier::lazy("foo").into_expr_lazy(), vec![]));
}

#[test]
fn call_with_args() {
    harness_expr(
        "foo(0, 1, 2)",
        Call::new(
            Identifier::lazy("foo").into_expr_lazy(),
            vec![
                Literal::Real(0.0).into_expr_lazy(),
                Literal::Real(1.0).into_expr_lazy(),
                Literal::Real(2.0).into_expr_lazy(),
            ],
        ),
    );
}

#[test]
fn call_trailing_commas() {
    harness_expr(
        "foo(0, 1, 2,)",
        Call::new(
            Identifier::lazy("foo").into_expr_lazy(),
            vec![
                Literal::Real(0.0).into_expr_lazy(),
                Literal::Real(1.0).into_expr_lazy(),
                Literal::Real(2.0).into_expr_lazy(),
            ],
        ),
    );
}

#[test]
fn construction() {
    harness_expr(
        "new foo()",
        Call::new_with_new_operator(Identifier::lazy("foo").into_expr_lazy(), vec![]),
    );
}

#[test]
fn empty_array() {
    harness_expr("[]", ExprType::Literal(Literal::Array(vec![])));
}

#[test]
fn simple_array() {
    harness_expr(
        "[0, 1, 2]",
        ExprType::Literal(Literal::Array(vec![
            Literal::Real(0.0).into_expr_lazy(),
            Literal::Real(1.0).into_expr_lazy(),
            Literal::Real(2.0).into_expr_lazy(),
        ])),
    );
}

#[test]
fn empty_struct() {
    harness_expr("{}", ExprType::Literal(Literal::Struct(vec![])));
}

#[test]
fn struct_begin_end() {
    harness_expr("begin end", ExprType::Literal(Literal::Struct(vec![])));
}

#[test]
fn simple_struct() {
    harness_expr(
        "{ foo: bar, fizz: buzz }",
        ExprType::Literal(Literal::Struct(vec![
            (Identifier::lazy("foo"), Identifier::lazy("bar").into_expr_lazy()),
            (Identifier::lazy("fizz"), Identifier::lazy("buzz").into_expr_lazy()),
        ])),
    );
}

#[test]
fn array_access() {
    harness_expr(
        "foo[bar]",
        Access::Array {
            left: Identifier::lazy("foo").into_expr_lazy(),
            index_one: Identifier::lazy("bar").into_expr_lazy(),
            index_two: None,
            using_accessor: false,
        },
    );
}

#[test]
fn array_direct_access() {
    harness_expr(
        "foo[@ bar]",
        Access::Array {
            left: Identifier::lazy("foo").into_expr_lazy(),
            index_one: Identifier::lazy("bar").into_expr_lazy(),
            index_two: None,
            using_accessor: true,
        },
    );
}

#[test]
fn array_access_2d() {
    harness_expr(
        "foo[bar, buzz]",
        Access::Array {
            left: Identifier::lazy("foo").into_expr_lazy(),
            index_one: Identifier::lazy("bar").into_expr_lazy(),
            index_two: Some(Identifier::lazy("buzz").into_expr_lazy()),
            using_accessor: false,
        },
    );
}

#[test]
fn ds_map_access() {
    harness_expr(
        "foo[? bar]",
        Access::Map {
            left: Identifier::lazy("foo").into_expr_lazy(),
            key: Identifier::lazy("bar").into_expr_lazy(),
        },
    );
}

#[test]
fn ds_list_access() {
    harness_expr(
        "foo[| bar]",
        Access::List {
            left: Identifier::lazy("foo").into_expr_lazy(),
            index: Identifier::lazy("bar").into_expr_lazy(),
        },
    );
}

#[test]
fn ds_grid_access() {
    harness_expr(
        "foo[# bar, buzz]",
        Access::Grid {
            left: Identifier::lazy("foo").into_expr_lazy(),
            index_one: Identifier::lazy("bar").into_expr_lazy(),
            index_two: Identifier::lazy("buzz").into_expr_lazy(),
        },
    );
}

#[test]
fn ds_grid_access_no_space() {
    harness_expr(
        "foo[#bar, buzz]",
        Access::Grid {
            left: Identifier::lazy("foo").into_expr_lazy(),
            index_one: Identifier::lazy("bar").into_expr_lazy(),
            index_two: Identifier::lazy("buzz").into_expr_lazy(),
        },
    );
}

#[test]
fn struct_access() {
    harness_expr(
        "foo[$ bar]",
        Access::Struct {
            left: Identifier::lazy("foo").into_expr_lazy(),
            key: Identifier::lazy("bar").into_expr_lazy(),
        },
    );
}

#[test]
fn chained_ds_accesses() {
    harness_expr(
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
        },
    );
}

#[test]
fn ds_access_call() {
    harness_expr(
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
        ),
    )
}

#[test]
fn dot_access() {
    harness_expr(
        "foo.bar",
        Access::Dot {
            left: Identifier::lazy("foo").into_expr_lazy(),
            right: Identifier::lazy("bar"),
        },
    );
}

#[test]
fn chained_dot_access() {
    harness_expr(
        "foo.bar.buzz",
        Access::Dot {
            left: Access::Dot {
                left: Identifier::lazy("foo").into_expr_lazy(),
                right: Identifier::lazy("bar"),
            }
            .into_expr_lazy(),
            right: Identifier::lazy("buzz"),
        },
    );
}

#[test]
fn dot_access_to_call() {
    harness_expr(
        "foo.bar()",
        Call::new(
            Access::Dot {
                left: Identifier::lazy("foo").into_expr_lazy(),
                right: Identifier::lazy("bar"),
            }
            .into_expr_lazy(),
            vec![],
        ),
    )
}

#[test]
fn dot_access_to_ds_access() {
    harness_expr(
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
        },
    );
}

#[test]
fn dot_access_from_call() {
    harness_expr(
        "foo().bar",
        Access::Dot {
            left: Call::new(Identifier::lazy("foo").into_expr_lazy(), vec![]).into_expr_lazy(),
            right: Identifier::lazy("bar"),
        },
    );
}

#[test]
fn chained_calls() {
    harness_expr(
        "foo().bar()",
        Call::new(
            Access::Dot {
                left: Call::new(Identifier::lazy("foo").into_expr_lazy(), vec![]).into_expr_lazy(),
                right: Identifier::lazy("bar"),
            }
            .into_expr_lazy(),
            vec![],
        ),
    );
}

#[test]
fn chain_calls_with_call_parameter() {
    harness_expr(
        "foo().bar(buzz())",
        Call::new(
            Access::Dot {
                left: Call::new(Identifier::lazy("foo").into_expr_lazy(), vec![]).into_expr_lazy(),
                right: Identifier::lazy("bar"),
            }
            .into_expr_lazy(),
            vec![Call::new(Identifier::lazy("buzz").into_expr_lazy(), vec![]).into_expr_lazy()],
        ),
    )
}

#[test]
fn global_dot_access() {
    harness_expr(
        "global.bar",
        Access::Global {
            right: Identifier::lazy("bar"),
        },
    );
}

#[test]
fn self_dot_access() {
    harness_expr(
        "self.bar",
        Access::Current {
            right: Identifier::lazy("bar"),
        },
    );
}

#[test]
fn other_dot_access() {
    harness_expr(
        "other.bar",
        Access::Other {
            right: Identifier::lazy("bar"),
        },
    );
}

#[test]
fn ds_dot_access() {
    harness_expr(
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
        },
    );
}

#[test]
fn grouping() {
    harness_expr("(0)", Grouping::lazy(Literal::Real(0.0).into_expr_lazy()));
}

#[test]
fn nested_grouping() {
    harness_expr(
        "((0) * 0)",
        Grouping::lazy(
            Evaluation::new(
                Grouping::lazy(Literal::Real(0.0).into_expr_lazy()).into_expr_lazy(),
                EvaluationOp::Star(Token::lazy(TokenType::Star)),
                Literal::Real(0.0).into_expr_lazy(),
            )
            .into_expr_lazy(),
        ),
    );
}

#[test]
fn identifier() {
    harness_expr("foo", Identifier::lazy("foo"));
}

#[test]
fn number() {
    harness_expr("0", Literal::Real(0.0));
}

#[test]
fn float() {
    harness_expr("0.01", Literal::Real(0.01));
}

#[test]
fn float_no_prefix() {
    harness_expr(".01", Literal::Real(0.01));
}

#[test]
fn constant() {
    harness_expr("true", ExprType::Literal(Literal::True));
    harness_expr("false", ExprType::Literal(Literal::False));
    harness_expr("undefined", ExprType::Literal(Literal::Undefined));
    harness_expr("noone", ExprType::Literal(Literal::Noone));
    harness_expr(
        "browser_not_a_browser",
        ExprType::Literal(Literal::Misc("browser_not_a_browser".into())),
    );
}

#[test]
fn string() {
    harness_expr("\"foo\"", ExprType::Literal(Literal::String("foo".into())));
}

#[test]
fn multi_line_string() {
    harness_expr(
        "@\"\nfoo\nfoo\"",
        ExprType::Literal(Literal::String("\nfoo\nfoo".into())),
    );
}

#[test]
fn multi_line_string_single_quote() {
    harness_expr("@'\nfoo\nfoo'", ExprType::Literal(Literal::String("\nfoo\nfoo".into())));
}

// I hate gamemaker.
#[test]
fn multi_line_string_single_quote_with_inner_double_quote() {
    harness_expr(
        "@'\nfoo\"\nfoo'",
        ExprType::Literal(Literal::String("\nfoo\"\nfoo".into())),
    );
}

#[test]
fn dollar_hex() {
    harness_expr("$a0f9a0", ExprType::Literal(Literal::Hex("a0f9a0".into())));
}

#[test]
fn short_hex() {
    harness_expr("$20", ExprType::Literal(Literal::Hex("20".into())));
}

#[test]
fn oh_x_hex() {
    harness_expr("0xa0f9a0", ExprType::Literal(Literal::Hex("a0f9a0".into())));
}

#[test]
fn logically_joined_expressions() {
    harness_expr(
        "foo == 1 && foo == 1 && foo == 1",
        Logical::new(
            Equality::new(
                Identifier::lazy("foo").into_expr_lazy(),
                EqualityOp::Equal(Token::lazy(TokenType::DoubleEqual)),
                Literal::Real(1.0).into_expr_lazy(),
            )
            .into_expr_lazy(),
            LogicalOp::And(Token::lazy(TokenType::DoubleAmpersand)),
            Logical::new(
                Equality::new(
                    Identifier::lazy("foo").into_expr_lazy(),
                    EqualityOp::Equal(Token::lazy(TokenType::DoubleEqual)),
                    Literal::Real(1.0).into_expr_lazy(),
                )
                .into_expr_lazy(),
                LogicalOp::And(Token::lazy(TokenType::DoubleAmpersand)),
                Equality::new(
                    Identifier::lazy("foo").into_expr_lazy(),
                    EqualityOp::Equal(Token::lazy(TokenType::DoubleEqual)),
                    Literal::Real(1.0).into_expr_lazy(),
                )
                .into_expr_lazy(),
            )
            .into_expr_lazy(),
        ),
    );
}

#[test]
fn comment_in_builder_chain() {
    harness_expr(
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
        ),
    );
}
