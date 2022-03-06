use crate::parse::*;
use pretty_assertions::assert_eq;

fn harness_expr(source: &'static str, expected: impl Into<Expression>) {
    let expected = expected.into();
    let mut parser = Parser::new(source, 0);
    let outputed = parser.expression().unwrap();
    assert_eq!(*outputed.expression(), expected, "`{}` failed!", source)
}

fn harness_valid(source: &'static str) {
    let mut parser = Parser::new(source, 0);
    assert!(parser.expression().is_ok(), "`{}` was invalid!", source)
}

fn harness_invalid(source: &'static str) {
    let mut parser = Parser::new(source, 0);
    assert!(parser.expression().is_err(), "`{}` was valid!", source)
}

#[test]
fn function() {
    harness_expr(
        "function foo() {}",
        Function::new(Identifier::lazy("foo"), vec![], Block::lazy(vec![]).into_lazy_box()),
    )
}

#[test]
fn static_function() {
    harness_expr(
        "static function foo() {}",
        Function::new(Identifier::lazy("foo"), vec![], Block::lazy(vec![]).into_lazy_box()),
    )
}

#[test]
fn function_with_parameters() {
    harness_expr(
        "function foo(bar, baz) {}",
        Function::new(
            Identifier::lazy("foo"),
            vec![
                OptionalInitilization::Uninitialized(Identifier::lazy("bar").into_lazy_box()),
                OptionalInitilization::Uninitialized(Identifier::lazy("baz").into_lazy_box()),
            ],
            Block::lazy(vec![]).into_lazy_box(),
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
                        Identifier::lazy("bar").into_lazy_box(),
                        AssignmentOperator::Equal(Token::lazy(TokenType::Equal)),
                        Literal::Real(1.0).into_lazy_box(),
                    )
                    .into_lazy_box(),
                ),
                OptionalInitilization::Uninitialized(Identifier::lazy("baz").into_lazy_box()),
            ],
            Block::lazy(vec![]).into_lazy_box(),
        ),
    )
}

#[test]
fn anonymous_function() {
    harness_expr(
        "function() {}",
        Function::new_anonymous(vec![], Block::lazy(vec![]).into_lazy_box()),
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
            Block::lazy(vec![]).into_lazy_box(),
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
            Constructor::WithInheritance(Call::new(Identifier::lazy("bar").into_lazy_box(), vec![]).into_lazy_box()),
            Block::lazy(vec![]).into_lazy_box(),
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
            Block::lazy(vec![Return::new(None).into_lazy_box()]).into_lazy_box(),
        ),
    )
}

#[test]
fn and() {
    harness_expr(
        "1 && 1",
        Logical::new(
            Literal::Real(1.0).into_lazy_box(),
            LogicalOperator::And(Token::lazy(TokenType::DoubleAmpersand)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
}

#[test]
fn and_keyword() {
    harness_expr(
        "1 and 1",
        Logical::new(
            Literal::Real(1.0).into_lazy_box(),
            LogicalOperator::And(Token::lazy(TokenType::And)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
}

#[test]
fn or() {
    harness_expr(
        "1 || 1",
        Logical::new(
            Literal::Real(1.0).into_lazy_box(),
            LogicalOperator::Or(Token::lazy(TokenType::DoublePipe)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
}

#[test]
fn or_keyword() {
    harness_expr(
        "1 or 1",
        Logical::new(
            Literal::Real(1.0).into_lazy_box(),
            LogicalOperator::Or(Token::lazy(TokenType::Or)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
}

#[test]
fn xor() {
    harness_expr(
        "1 xor 1",
        Logical::new(
            Literal::Real(1.0).into_lazy_box(),
            LogicalOperator::Xor(Token::lazy(TokenType::Xor)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
}

#[test]
fn addition() {
    harness_expr(
        "1 + 1",
        Evaluation::new(
            Literal::Real(1.0).into_lazy_box(),
            EvaluationOperator::Plus(Token::lazy(TokenType::Plus)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
}

#[test]
fn subtraction() {
    harness_expr(
        "1 - 1",
        Evaluation::new(
            Literal::Real(1.0).into_lazy_box(),
            EvaluationOperator::Minus(Token::lazy(TokenType::Minus)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
}

#[test]
fn multiplication() {
    harness_expr(
        "1 * 1",
        Evaluation::new(
            Literal::Real(1.0).into_lazy_box(),
            EvaluationOperator::Star(Token::lazy(TokenType::Star)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
}

#[test]
fn division() {
    harness_expr(
        "1 / 1",
        Evaluation::new(
            Literal::Real(1.0).into_lazy_box(),
            EvaluationOperator::Slash(Token::lazy(TokenType::Slash)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
}

#[test]
fn modulo() {
    harness_expr(
        "1 mod 1",
        Evaluation::new(
            Literal::Real(1.0).into_lazy_box(),
            EvaluationOperator::Modulo(Token::lazy(TokenType::Mod)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
    harness_expr(
        "1 % 1",
        Evaluation::new(
            Literal::Real(1.0).into_lazy_box(),
            EvaluationOperator::Modulo(Token::lazy(TokenType::Percent)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
}

#[test]
fn div() {
    harness_expr(
        "1 div 1",
        Evaluation::new(
            Literal::Real(1.0).into_lazy_box(),
            EvaluationOperator::Div(Token::lazy(TokenType::Div)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
}

#[test]
fn bitwise_and() {
    harness_expr(
        "1 & 1",
        Evaluation::new(
            Literal::Real(1.0).into_lazy_box(),
            EvaluationOperator::And(Token::lazy(TokenType::Ampersand)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
}

#[test]
fn bitwise_or() {
    harness_expr(
        "1 | 1",
        Evaluation::new(
            Literal::Real(1.0).into_lazy_box(),
            EvaluationOperator::Or(Token::lazy(TokenType::Pipe)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
}

#[test]
fn bitwise_chain() {
    harness_expr(
        "1 | 1 | 1",
        Evaluation::new(
            Literal::Real(1.0).into_lazy_box(),
            EvaluationOperator::Or(Token::lazy(TokenType::Pipe)),
            Evaluation::new(
                Literal::Real(1.0).into_lazy_box(),
                EvaluationOperator::Or(Token::lazy(TokenType::Pipe)),
                Literal::Real(1.0).into_lazy_box(),
            )
            .into_lazy_box(),
        ),
    );
}

#[test]
fn bitwise_xor() {
    harness_expr(
        "1 ^ 1",
        Evaluation::new(
            Literal::Real(1.0).into_lazy_box(),
            EvaluationOperator::Xor(Token::lazy(TokenType::Circumflex)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
}

#[test]
fn dot_access_bitwise() {
    harness_expr(
        "foo.bar | foo.bar",
        Evaluation::new(
            Access::Dot {
                left: Identifier::lazy("foo").into_lazy_box(),
                right: Identifier::lazy("bar").into_lazy_box(),
            }
            .into_lazy_box(),
            EvaluationOperator::Or(Token::lazy(TokenType::Pipe)),
            Access::Dot {
                left: Identifier::lazy("foo").into_lazy_box(),
                right: Identifier::lazy("bar").into_lazy_box(),
            }
            .into_lazy_box(),
        ),
    );
}

#[test]
fn bit_shift_left() {
    harness_expr(
        "1 << 1",
        Evaluation::new(
            Literal::Real(1.0).into_lazy_box(),
            EvaluationOperator::BitShiftLeft(Token::lazy(TokenType::BitShiftLeft)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
}

#[test]
fn bit_shift_right() {
    harness_expr(
        "1 >> 1",
        Evaluation::new(
            Literal::Real(1.0).into_lazy_box(),
            EvaluationOperator::BitShiftRight(Token::lazy(TokenType::BitShiftRight)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
}

#[test]
fn less_than() {
    harness_expr(
        "1 < 1",
        Equality::new(
            Literal::Real(1.0).into_lazy_box(),
            EqualityOperator::LessThan(Token::lazy(TokenType::LessThan)),
            Literal::Real(1.0).into_lazy_box(),
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
                            Literal::Real(1.0).into_lazy_box(),
                            EvaluationOperator::Star(Token::lazy(TokenType::Star)),
                            Literal::Real(1.0).into_lazy_box(),
                        )
                        .into_lazy_box(),
                        EvaluationOperator::Plus(Token::lazy(TokenType::Plus)),
                        Literal::Real(1.0).into_lazy_box(),
                    )
                    .into_lazy_box(),
                    EvaluationOperator::BitShiftRight(Token::lazy(TokenType::BitShiftRight)),
                    Literal::Real(1.0).into_lazy_box(),
                )
                .into_lazy_box(),
                EvaluationOperator::And(Token::lazy(TokenType::Ampersand)),
                Literal::Real(1.0).into_lazy_box(),
            )
            .into_lazy_box(),
            EqualityOperator::Equal(Token::lazy(TokenType::DoubleEqual)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    )
}

#[test]
fn less_than_or_equal() {
    harness_expr(
        "1 <= 1",
        Equality::new(
            Literal::Real(1.0).into_lazy_box(),
            EqualityOperator::LessThanOrEqual(Token::lazy(TokenType::LessThanOrEqual)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
}

#[test]
fn greater_than() {
    harness_expr(
        "1 > 1",
        Equality::new(
            Literal::Real(1.0).into_lazy_box(),
            EqualityOperator::GreaterThan(Token::lazy(TokenType::GreaterThan)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
}

#[test]
fn greater_than_or_equal() {
    harness_expr(
        "1 >= 1",
        Equality::new(
            Literal::Real(1.0).into_lazy_box(),
            EqualityOperator::GreaterThanOrEqual(Token::lazy(TokenType::GreaterThanOrEqual)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
}

#[test]
fn equal() {
    harness_expr(
        "1 == 1",
        Equality::new(
            Literal::Real(1.0).into_lazy_box(),
            EqualityOperator::Equal(Token::lazy(TokenType::DoubleEqual)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
}

#[test]
fn bang_equal() {
    harness_expr(
        "1 != 1",
        Equality::new(
            Literal::Real(1.0).into_lazy_box(),
            EqualityOperator::NotEqual(Token::lazy(TokenType::BangEqual)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
}

#[test]
fn null_coalecence() {
    harness_expr(
        "foo ?? 1",
        NullCoalecence::new(
            Identifier::lazy("foo").into_lazy_box(),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
}

#[test]
fn ternary() {
    harness_expr(
        "foo ? 1 : 2",
        Ternary::new(
            Identifier::lazy("foo").into_lazy_box(),
            Literal::Real(1.0).into_lazy_box(),
            Literal::Real(2.0).into_lazy_box(),
        ),
    );
}

#[test]
fn not() {
    harness_expr(
        "!foo",
        Unary::new(
            UnaryOperator::Not(Token::lazy(TokenType::Bang)),
            Identifier::lazy("foo").into_lazy_box(),
        ),
    );
}

#[test]
fn not_keyword() {
    harness_expr(
        "not foo",
        Unary::new(
            UnaryOperator::Not(Token::lazy(TokenType::Not)),
            Identifier::lazy("foo").into_lazy_box(),
        ),
    );
}

#[test]
fn positive() {
    harness_expr(
        "+1",
        Unary::new(
            UnaryOperator::Positive(Token::lazy(TokenType::Plus)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
}

#[test]
fn neagtive() {
    harness_expr(
        "-1",
        Unary::new(
            UnaryOperator::Negative(Token::lazy(TokenType::Minus)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
}

#[test]
fn dot_unary() {
    harness_expr(
        "!self.foo",
        Unary::new(
            UnaryOperator::Not(Token::lazy(TokenType::Bang)),
            Access::Current {
                right: Identifier::lazy("foo").into_lazy_box(),
            }
            .into_lazy_box(),
        ),
    );
}

#[test]
fn ds_unary() {
    harness_expr(
        "!foo[bar]",
        Unary::new(
            UnaryOperator::Not(Token::lazy(TokenType::Bang)),
            Access::Array {
                left: Identifier::lazy("foo").into_lazy_box(),
                index_one: Identifier::lazy("bar").into_lazy_box(),
                index_two: None,
                using_accessor: false,
            }
            .into_lazy_box(),
        ),
    );
}

#[test]
fn prefix_increment() {
    harness_expr(
        "++1",
        Unary::new(
            UnaryOperator::Increment(Token::lazy(TokenType::DoublePlus)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
}

#[test]
fn prefix_decrement() {
    harness_expr(
        "--1",
        Unary::new(
            UnaryOperator::Decrement(Token::lazy(TokenType::DoubleMinus)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
}

#[test]
fn bitwise_not() {
    harness_expr(
        "~1",
        Unary::new(
            UnaryOperator::BitwiseNot(Token::lazy(TokenType::Tilde)),
            Literal::Real(1.0).into_lazy_box(),
        ),
    );
}

#[test]
fn postfix_increment() {
    harness_expr(
        "1++",
        Postfix::new(
            Literal::Real(1.0).into_lazy_box(),
            PostfixOperator::Increment(Token::lazy(TokenType::DoublePlus)),
        ),
    );
}

#[test]
fn postfix_decrement() {
    harness_expr(
        "1--",
        Postfix::new(
            Literal::Real(1.0).into_lazy_box(),
            PostfixOperator::Decrement(Token::lazy(TokenType::DoubleMinus)),
        ),
    );
}

#[test]
fn dot_postfix() {
    harness_expr(
        "self.foo++",
        Postfix::new(
            Access::Current {
                right: Identifier::lazy("foo").into_lazy_box(),
            }
            .into_lazy_box(),
            PostfixOperator::Increment(Token::lazy(TokenType::DoublePlus)),
        ),
    );
}

#[test]
fn ds_postfix() {
    harness_expr(
        "foo[bar]++",
        Postfix::new(
            Access::Array {
                left: Identifier::lazy("foo").into_lazy_box(),
                index_one: Identifier::lazy("bar").into_lazy_box(),
                index_two: None,
                using_accessor: false,
            }
            .into_lazy_box(),
            PostfixOperator::Increment(Token::lazy(TokenType::DoublePlus)),
        ),
    );
}

#[test]
fn call() {
    harness_expr("foo()", Call::new(Identifier::lazy("foo").into_lazy_box(), vec![]));
}

#[test]
fn call_with_args() {
    harness_expr(
        "foo(0, 1, 2)",
        Call::new(
            Identifier::lazy("foo").into_lazy_box(),
            vec![
                Literal::Real(0.0).into_lazy_box(),
                Literal::Real(1.0).into_lazy_box(),
                Literal::Real(2.0).into_lazy_box(),
            ],
        ),
    );
}

#[test]
fn call_trailing_commas() {
    harness_expr(
        "foo(0, 1, 2,)",
        Call::new(
            Identifier::lazy("foo").into_lazy_box(),
            vec![
                Literal::Real(0.0).into_lazy_box(),
                Literal::Real(1.0).into_lazy_box(),
                Literal::Real(2.0).into_lazy_box(),
            ],
        ),
    );
}

#[test]
fn construction() {
    harness_expr(
        "new foo()",
        Call::new_with_new_operator(Identifier::lazy("foo").into_lazy_box(), vec![]),
    );
}

#[test]
fn empty_array() {
    harness_expr("[]", Expression::Literal(Literal::Array(vec![])));
}

#[test]
fn simple_array() {
    harness_expr(
        "[0, 1, 2]",
        Expression::Literal(Literal::Array(vec![
            Literal::Real(0.0).into_lazy_box(),
            Literal::Real(1.0).into_lazy_box(),
            Literal::Real(2.0).into_lazy_box(),
        ])),
    );
}

#[test]
fn empty_struct() {
    harness_expr("{}", Expression::Literal(Literal::Struct(vec![])));
}

#[test]
fn struct_begin_end() {
    harness_expr("begin end", Expression::Literal(Literal::Struct(vec![])));
}

#[test]
fn simple_struct() {
    harness_expr(
        "{ foo: bar, fizz: buzz }",
        Expression::Literal(Literal::Struct(vec![
            (Identifier::lazy("foo"), Identifier::lazy("bar").into_lazy_box()),
            (Identifier::lazy("fizz"), Identifier::lazy("buzz").into_lazy_box()),
        ])),
    );
}

#[test]
fn array_access() {
    harness_expr(
        "foo[bar]",
        Access::Array {
            left: Identifier::lazy("foo").into_lazy_box(),
            index_one: Identifier::lazy("bar").into_lazy_box(),
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
            left: Identifier::lazy("foo").into_lazy_box(),
            index_one: Identifier::lazy("bar").into_lazy_box(),
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
            left: Identifier::lazy("foo").into_lazy_box(),
            index_one: Identifier::lazy("bar").into_lazy_box(),
            index_two: Some(Identifier::lazy("buzz").into_lazy_box()),
            using_accessor: false,
        },
    );
}

#[test]
fn ds_map_access() {
    harness_expr(
        "foo[? bar]",
        Access::Map {
            left: Identifier::lazy("foo").into_lazy_box(),
            key: Identifier::lazy("bar").into_lazy_box(),
        },
    );
}

#[test]
fn ds_list_access() {
    harness_expr(
        "foo[| bar]",
        Access::List {
            left: Identifier::lazy("foo").into_lazy_box(),
            index: Identifier::lazy("bar").into_lazy_box(),
        },
    );
}

#[test]
fn ds_grid_access() {
    harness_expr(
        "foo[# bar, buzz]",
        Access::Grid {
            left: Identifier::lazy("foo").into_lazy_box(),
            index_one: Identifier::lazy("bar").into_lazy_box(),
            index_two: Identifier::lazy("buzz").into_lazy_box(),
        },
    );
}

#[test]
fn ds_grid_access_no_space() {
    harness_expr(
        "foo[#bar, buzz]",
        Access::Grid {
            left: Identifier::lazy("foo").into_lazy_box(),
            index_one: Identifier::lazy("bar").into_lazy_box(),
            index_two: Identifier::lazy("buzz").into_lazy_box(),
        },
    );
}

#[test]
fn struct_access() {
    harness_expr(
        "foo[$ bar]",
        Access::Struct {
            left: Identifier::lazy("foo").into_lazy_box(),
            key: Identifier::lazy("bar").into_lazy_box(),
        },
    );
}

#[test]
fn chained_ds_accesses() {
    harness_expr(
        "foo[bar][buzz]",
        Access::Array {
            left: Access::Array {
                left: Identifier::lazy("foo").into_lazy_box(),
                index_one: Identifier::lazy("bar").into_lazy_box(),
                index_two: None,
                using_accessor: false,
            }
            .into_lazy_box(),
            index_one: Identifier::lazy("buzz").into_lazy_box(),
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
                left: Identifier::lazy("foo").into_lazy_box(),
                index_one: Identifier::lazy("bar").into_lazy_box(),
                index_two: None,
                using_accessor: false,
            }
            .into_lazy_box(),
            vec![],
        ),
    )
}

#[test]
fn dot_access() {
    harness_expr(
        "foo.bar",
        Access::Dot {
            left: Identifier::lazy("foo").into_lazy_box(),
            right: Identifier::lazy("bar").into_lazy_box(),
        },
    );
}

#[test]
fn grouping_dot_access() {
    harness_expr(
        "(foo).bar",
        Access::Dot {
            left: Grouping::new(Identifier::lazy("foo").into_lazy_box()).into_lazy_box(),
            right: Identifier::lazy("bar").into_lazy_box(),
        },
    );
}

#[test]
fn chained_dot_access() {
    harness_expr(
        "foo.bar.buzz",
        Access::Dot {
            left: Access::Dot {
                left: Identifier::lazy("foo").into_lazy_box(),
                right: Identifier::lazy("bar").into_lazy_box(),
            }
            .into_lazy_box(),
            right: Identifier::lazy("buzz").into_lazy_box(),
        },
    );
}

#[test]
fn dot_access_to_call() {
    harness_expr(
        "foo.bar()",
        Call::new(
            Access::Dot {
                left: Identifier::lazy("foo").into_lazy_box(),
                right: Identifier::lazy("bar").into_lazy_box(),
            }
            .into_lazy_box(),
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
                left: Identifier::lazy("foo").into_lazy_box(),
                right: Identifier::lazy("bar").into_lazy_box(),
            }
            .into_lazy_box(),
            index_one: Literal::Real(0.0).into_lazy_box(),
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
            left: Call::new(Identifier::lazy("foo").into_lazy_box(), vec![]).into_lazy_box(),
            right: Identifier::lazy("bar").into_lazy_box(),
        },
    );
}

#[test]
fn chained_calls() {
    harness_expr(
        "foo().bar()",
        Call::new(
            Access::Dot {
                left: Call::new(Identifier::lazy("foo").into_lazy_box(), vec![]).into_lazy_box(),
                right: Identifier::lazy("bar").into_lazy_box(),
            }
            .into_lazy_box(),
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
                left: Call::new(Identifier::lazy("foo").into_lazy_box(), vec![]).into_lazy_box(),
                right: Identifier::lazy("bar").into_lazy_box(),
            }
            .into_lazy_box(),
            vec![Call::new(Identifier::lazy("buzz").into_lazy_box(), vec![]).into_lazy_box()],
        ),
    )
}

#[test]
fn global_dot_access() {
    harness_expr(
        "global.bar",
        Access::Global {
            right: Identifier::lazy("bar").into_lazy_box(),
        },
    );
}

#[test]
fn self_dot_access() {
    harness_expr(
        "self.bar",
        Access::Current {
            right: Identifier::lazy("bar").into_lazy_box(),
        },
    );
}

#[test]
fn other_dot_access() {
    harness_expr(
        "other.bar",
        Access::Other {
            right: Identifier::lazy("bar").into_lazy_box(),
        },
    );
}

#[test]
fn ds_dot_access() {
    harness_expr(
        "foo[0].bar",
        Access::Dot {
            left: Access::Array {
                left: Identifier::lazy("foo").into_lazy_box(),
                index_one: Literal::Real(0.0).into_lazy_box(),
                index_two: None,
                using_accessor: false,
            }
            .into_lazy_box(),
            right: Identifier::lazy("bar").into_lazy_box(),
        },
    );
}

#[test]
fn grouping() {
    harness_expr("(0)", Grouping::new(Literal::Real(0.0).into_lazy_box()));
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
    harness_expr("true", Expression::Literal(Literal::True));
    harness_expr("false", Expression::Literal(Literal::False));
    harness_expr("undefined", Expression::Literal(Literal::Undefined));
    harness_expr("noone", Expression::Literal(Literal::Noone));
    harness_expr(
        "browser_not_a_browser",
        Expression::Literal(Literal::Misc("browser_not_a_browser".into())),
    );
}

#[test]
fn string() {
    harness_expr("\"foo\"", Expression::Literal(Literal::String("foo".into())));
}

#[test]
fn multi_line_string() {
    harness_expr(
        "@\"\nfoo\nfoo\"",
        Expression::Literal(Literal::String("\nfoo\nfoo".into())),
    );
}

#[test]
fn multi_line_string_single_quote() {
    harness_expr(
        "@'\nfoo\nfoo'",
        Expression::Literal(Literal::String("\nfoo\nfoo".into())),
    );
}

// I hate gamemaker.
#[test]
fn multi_line_string_single_quote_with_inner_double_quote() {
    harness_expr(
        "@'\nfoo\"\nfoo'",
        Expression::Literal(Literal::String("\nfoo\"\nfoo".into())),
    );
}

#[test]
fn dollar_hex() {
    harness_expr("$a0f9a0", Expression::Literal(Literal::Hex("a0f9a0".into())));
}

#[test]
fn short_hex() {
    harness_expr("$20", Expression::Literal(Literal::Hex("20".into())));
}

#[test]
fn oh_x_hex() {
    harness_expr("0xa0f9a0", Expression::Literal(Literal::Hex("a0f9a0".into())));
}

#[test]
fn logically_joined_expressions() {
    harness_expr(
        "foo == 1 && foo == 1 && foo == 1",
        Logical::new(
            Equality::new(
                Identifier::lazy("foo").into_lazy_box(),
                EqualityOperator::Equal(Token::lazy(TokenType::DoubleEqual)),
                Literal::Real(1.0).into_lazy_box(),
            )
            .into_lazy_box(),
            LogicalOperator::And(Token::lazy(TokenType::DoubleAmpersand)),
            Logical::new(
                Equality::new(
                    Identifier::lazy("foo").into_lazy_box(),
                    EqualityOperator::Equal(Token::lazy(TokenType::DoubleEqual)),
                    Literal::Real(1.0).into_lazy_box(),
                )
                .into_lazy_box(),
                LogicalOperator::And(Token::lazy(TokenType::DoubleAmpersand)),
                Equality::new(
                    Identifier::lazy("foo").into_lazy_box(),
                    EqualityOperator::Equal(Token::lazy(TokenType::DoubleEqual)),
                    Literal::Real(1.0).into_lazy_box(),
                )
                .into_lazy_box(),
            )
            .into_lazy_box(),
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
                left: Call::new(Identifier::lazy("foo").into_lazy_box(), vec![]).into_lazy_box(),
                right: Identifier::lazy("bar").into_lazy_box(),
            }
            .into_lazy_box(),
            vec![],
        ),
    );
}