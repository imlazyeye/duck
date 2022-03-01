use crate::{
    gml::{
        Access, Assignment, AssignmentOperator, Block, Constructor, Equality, EqualityOperator, Evaluation,
        EvaluationOperator, Function, Identifier, Logical, LogicalOperator, NullCoalecence, Parameter, Postfix,
        PostfixOperator, Return, Ternary, Unary, UnaryOperator,
    },
    parsing::{
        expression::{Expression, Literal},
        parser::Parser,
    },
    prelude::{IntoExpressionBox, IntoStatementBox},
};
use colored::Colorize;
use pretty_assertions::assert_eq;

fn harness_expr(source: &str, expected: impl Into<Expression>) {
    let expected = expected.into();
    let mut parser = Parser::new(source, "test".into());
    let outputed = parser.expression().unwrap();
    println!("{}: {}", "Source".yellow(), source);
    assert_eq!(expected, *outputed.expression())
}

#[test]
fn function() {
    harness_expr(
        "function foo() {}",
        Function::new("foo", vec![], Block::new(vec![]).into_lazy_box()),
    )
}

#[test]
fn static_function() {
    harness_expr(
        "static function foo() {}",
        Function::new("foo", vec![], Block::new(vec![]).into_lazy_box()),
    )
}

#[test]
fn function_with_parameters() {
    harness_expr(
        "function foo(bar, baz) {}",
        Function::new(
            "foo",
            vec![Parameter::new("bar"), Parameter::new("baz")],
            Block::new(vec![]).into_lazy_box(),
        ),
    )
}

#[test]
fn default_parameters() {
    harness_expr(
        "function foo(bar=20, baz) {}",
        Function::new(
            "foo",
            vec![
                Parameter::new_with_default("bar", Expression::Literal(Literal::Real(20.0)).lazy_box()),
                Parameter::new("baz"),
            ],
            Block::new(vec![]).into_lazy_box(),
        ),
    )
}

#[test]
fn anonymous_function() {
    harness_expr(
        "function() {}",
        Function::new_anonymous(vec![], Block::new(vec![]).into_lazy_box()),
    )
}

#[test]
fn constructor() {
    harness_expr(
        "function foo() constructor {}",
        Function::new_constructor(
            Some("foo".into()),
            vec![],
            Constructor::WithoutInheritance,
            Block::new(vec![]).into_lazy_box(),
        ),
    )
}

#[test]
fn inheritance() {
    harness_expr(
        "function foo() : bar() constructor {}",
        Function::new_constructor(
            Some("foo".into()),
            vec![],
            Constructor::WithInheritance(
                Expression::Call(Identifier::new("bar").into_lazy_box(), vec![], false).lazy_box(),
            ),
            Block::new(vec![]).into_lazy_box(),
        ),
    )
}

#[test]
fn function_return_no_semi_colon() {
    harness_expr(
        "function foo() { return }",
        Function::new(
            "foo",
            vec![],
            Block::new(vec![Return::new(None).into_lazy_box()]).into_lazy_box(),
        ),
    )
}

#[test]
fn and() {
    harness_expr(
        "1 && 1",
        Logical::new(
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
            LogicalOperator::And,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn and_keyword() {
    harness_expr(
        "1 and 1",
        Logical::new(
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
            LogicalOperator::And,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn or() {
    harness_expr(
        "1 || 1",
        Logical::new(
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
            LogicalOperator::Or,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn or_keyword() {
    harness_expr(
        "1 or 1",
        Logical::new(
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
            LogicalOperator::Or,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn xor() {
    harness_expr(
        "1 xor 1",
        Logical::new(
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
            LogicalOperator::Xor,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn addition() {
    harness_expr(
        "1 + 1",
        Evaluation::new(
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
            EvaluationOperator::Plus,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn subtraction() {
    harness_expr(
        "1 - 1",
        Evaluation::new(
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
            EvaluationOperator::Minus,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn multiplication() {
    harness_expr(
        "1 * 1",
        Evaluation::new(
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
            EvaluationOperator::Star,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn division() {
    harness_expr(
        "1 / 1",
        Evaluation::new(
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
            EvaluationOperator::Slash,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn modulo() {
    harness_expr(
        "1 mod 1",
        Evaluation::new(
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
            EvaluationOperator::Modulo,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
    harness_expr(
        "1 % 1",
        Evaluation::new(
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
            EvaluationOperator::Modulo,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn div() {
    harness_expr(
        "1 div 1",
        Evaluation::new(
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
            EvaluationOperator::Div,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn bitwise_and() {
    harness_expr(
        "1 & 1",
        Evaluation::new(
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
            EvaluationOperator::And,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn bitwise_or() {
    harness_expr(
        "1 | 1",
        Evaluation::new(
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
            EvaluationOperator::Or,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn bitwise_chain() {
    harness_expr(
        "1 | 1 | 1",
        Evaluation::new(
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
            EvaluationOperator::Or,
            Evaluation::new(
                Expression::Literal(Literal::Real(1.0)).lazy_box(),
                EvaluationOperator::Or,
                Expression::Literal(Literal::Real(1.0)).lazy_box(),
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
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
            EvaluationOperator::Xor,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn dot_access_bitwise() {
    harness_expr(
        "foo.bar | foo.bar",
        Evaluation::new(
            Access::Dot {
                left: Identifier::new("foo").into_lazy_box(),
                right: Identifier::new("bar").into_lazy_box(),
            }
            .into_lazy_box(),
            EvaluationOperator::Or,
            Access::Dot {
                left: Identifier::new("foo").into_lazy_box(),
                right: Identifier::new("bar").into_lazy_box(),
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
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
            EvaluationOperator::BitShiftLeft,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn bit_shift_right() {
    harness_expr(
        "1 >> 1",
        Evaluation::new(
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
            EvaluationOperator::BitShiftRight,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn less_than() {
    harness_expr(
        "1 < 1",
        Equality::new(
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
            EqualityOperator::LessThan,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
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
                            Expression::Literal(Literal::Real(1.0)).lazy_box(),
                            EvaluationOperator::Star,
                            Expression::Literal(Literal::Real(1.0)).lazy_box(),
                        )
                        .into_lazy_box(),
                        EvaluationOperator::Plus,
                        Expression::Literal(Literal::Real(1.0)).lazy_box(),
                    )
                    .into_lazy_box(),
                    EvaluationOperator::BitShiftRight,
                    Expression::Literal(Literal::Real(1.0)).lazy_box(),
                )
                .into_lazy_box(),
                EvaluationOperator::And,
                Expression::Literal(Literal::Real(1.0)).lazy_box(),
            )
            .into_lazy_box(),
            EqualityOperator::Equal,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    )
}

#[test]
fn less_than_or_equal() {
    harness_expr(
        "1 <= 1",
        Equality::new(
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
            EqualityOperator::LessThanOrEqual,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn greater_than() {
    harness_expr(
        "1 > 1",
        Equality::new(
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
            EqualityOperator::GreaterThan,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn greater_than_or_equal() {
    harness_expr(
        "1 >= 1",
        Equality::new(
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
            EqualityOperator::GreaterThanOrEqual,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn equal() {
    harness_expr(
        "1 == 1",
        Equality::new(
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
            EqualityOperator::Equal,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn bang_equal() {
    harness_expr(
        "1 != 1",
        Equality::new(
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
            EqualityOperator::NotEqual,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn null_coalecence() {
    harness_expr(
        "foo ?? 1",
        NullCoalecence::new(
            Identifier::new("foo").into_lazy_box(),
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn ternary() {
    harness_expr(
        "foo ? 1 : 2",
        Ternary::new(
            Identifier::new("foo").into_lazy_box(),
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
            Expression::Literal(Literal::Real(2.0)).lazy_box(),
        ),
    );
}

#[test]
fn assign() {
    harness_expr(
        "foo = 1",
        Assignment::new(
            Identifier::new("foo").into_lazy_box(),
            AssignmentOperator::Equal,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn dot_assign() {
    harness_expr(
        "self.foo = 1",
        Assignment::new(
            Access::Current {
                right: Identifier::new("foo").into_lazy_box(),
            }
            .into_lazy_box(),
            AssignmentOperator::Equal,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn ds_assign() {
    harness_expr(
        "foo[0] = 1",
        Assignment::new(
            Access::Array {
                left: Identifier::new("foo").into_lazy_box(),
                index_one: Expression::Literal(Literal::Real(0.0)).lazy_box(),
                index_two: None,
                using_accessor: false,
            }
            .into_lazy_box(),
            AssignmentOperator::Equal,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
// Valid GML, as much as it hurts. See `assignment_to_call`
fn call_assign() {
    harness_expr(
        "foo() = 1",
        Assignment::new(
            Expression::Call(Identifier::new("foo").into_lazy_box(), vec![], false).lazy_box(),
            AssignmentOperator::Equal,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn static_assign() {
    harness_expr(
        "static foo = 1",
        Assignment::new(
            Identifier::new("foo").into_lazy_box(),
            AssignmentOperator::Equal,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn plus_equal() {
    harness_expr(
        "foo += 1",
        Assignment::new(
            Identifier::new("foo").into_lazy_box(),
            AssignmentOperator::PlusEqual,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn minus_equal() {
    harness_expr(
        "foo -= 1",
        Assignment::new(
            Identifier::new("foo").into_lazy_box(),
            AssignmentOperator::MinusEqual,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn star_equal() {
    harness_expr(
        "foo *= 1",
        Assignment::new(
            Identifier::new("foo").into_lazy_box(),
            AssignmentOperator::StarEqual,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn slash_equal() {
    harness_expr(
        "foo /= 1",
        Assignment::new(
            Identifier::new("foo").into_lazy_box(),
            AssignmentOperator::SlashEqual,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn and_equal() {
    harness_expr(
        "foo &= 1",
        Assignment::new(
            Identifier::new("foo").into_lazy_box(),
            AssignmentOperator::AndEqual,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn or_equal() {
    harness_expr(
        "foo |= 1",
        Assignment::new(
            Identifier::new("foo").into_lazy_box(),
            AssignmentOperator::OrEqual,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn xor_equal() {
    harness_expr(
        "foo ^= 1",
        Assignment::new(
            Identifier::new("foo").into_lazy_box(),
            AssignmentOperator::XorEqual,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn mod_equal() {
    harness_expr(
        "foo %= 1",
        Assignment::new(
            Identifier::new("foo").into_lazy_box(),
            AssignmentOperator::ModEqual,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn not() {
    harness_expr(
        "!foo",
        Unary::new(UnaryOperator::Not, Identifier::new("foo").into_lazy_box()),
    );
}

#[test]
fn not_keyword() {
    harness_expr(
        "not foo",
        Unary::new(UnaryOperator::Not, Identifier::new("foo").into_lazy_box()),
    );
}

#[test]
fn positive() {
    harness_expr(
        "+1",
        Unary::new(
            UnaryOperator::Positive,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn neagtive() {
    harness_expr(
        "-1",
        Unary::new(
            UnaryOperator::Negative,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn dot_unary() {
    harness_expr(
        "!self.foo",
        Unary::new(
            UnaryOperator::Not,
            Access::Current {
                right: Identifier::new("foo").into_lazy_box(),
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
            UnaryOperator::Not,
            Access::Array {
                left: Identifier::new("foo").into_lazy_box(),
                index_one: Identifier::new("bar").into_lazy_box(),
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
            UnaryOperator::Increment,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn prefix_decrement() {
    harness_expr(
        "--1",
        Unary::new(
            UnaryOperator::Decrement,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn bitwise_not() {
    harness_expr(
        "~1",
        Unary::new(
            UnaryOperator::BitwiseNot,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn postfix_increment() {
    harness_expr(
        "1++",
        Postfix::new(
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
            PostfixOperator::Increment,
        ),
    );
}

#[test]
fn postfix_decrement() {
    harness_expr(
        "1--",
        Postfix::new(
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
            PostfixOperator::Decrement,
        ),
    );
}

#[test]
fn dot_postfix() {
    harness_expr(
        "self.foo++",
        Postfix::new(
            Access::Current {
                right: Identifier::new("foo").into_lazy_box(),
            }
            .into_lazy_box(),
            PostfixOperator::Increment,
        ),
    );
}

#[test]
fn ds_postfix() {
    harness_expr(
        "foo[bar]++",
        Postfix::new(
            Access::Array {
                left: Identifier::new("foo").into_lazy_box(),
                index_one: Identifier::new("bar").into_lazy_box(),
                index_two: None,
                using_accessor: false,
            }
            .into_lazy_box(),
            PostfixOperator::Increment,
        ),
    );
}

#[test]
fn call() {
    harness_expr(
        "foo()",
        Expression::Call(Identifier::new("foo").into_lazy_box(), vec![], false),
    );
}

#[test]
fn call_with_args() {
    harness_expr(
        "foo(0, 1, 2)",
        Expression::Call(
            Identifier::new("foo").into_lazy_box(),
            vec![
                Expression::Literal(Literal::Real(0.0)).lazy_box(),
                Expression::Literal(Literal::Real(1.0)).lazy_box(),
                Expression::Literal(Literal::Real(2.0)).lazy_box(),
            ],
            false,
        ),
    );
}

#[test]
fn call_trailing_commas() {
    harness_expr(
        "foo(0, 1, 2,)",
        Expression::Call(
            Identifier::new("foo").into_lazy_box(),
            vec![
                Expression::Literal(Literal::Real(0.0)).lazy_box(),
                Expression::Literal(Literal::Real(1.0)).lazy_box(),
                Expression::Literal(Literal::Real(2.0)).lazy_box(),
            ],
            false,
        ),
    );
}

#[test]
fn construction() {
    harness_expr(
        "new foo()",
        Expression::Call(Identifier::new("foo").into_lazy_box(), vec![], true),
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
            Expression::Literal(Literal::Real(0.0)).lazy_box(),
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
            Expression::Literal(Literal::Real(2.0)).lazy_box(),
        ])),
    );
}

#[test]
fn empty_struct() {
    harness_expr("{}", Expression::Literal(Literal::Struct(vec![])));
}

#[test]
fn simple_struct() {
    harness_expr(
        "{ foo: bar, fizz: buzz }",
        Expression::Literal(Literal::Struct(vec![
            ("foo".into(), Identifier::new("bar").into_lazy_box()),
            ("fizz".into(), Identifier::new("buzz").into_lazy_box()),
        ])),
    );
}

#[test]
fn array_access() {
    harness_expr(
        "foo[bar]",
        Access::Array {
            left: Identifier::new("foo").into_lazy_box(),
            index_one: Identifier::new("bar").into_lazy_box(),
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
            left: Identifier::new("foo").into_lazy_box(),
            index_one: Identifier::new("bar").into_lazy_box(),
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
            left: Identifier::new("foo").into_lazy_box(),
            index_one: Identifier::new("bar").into_lazy_box(),
            index_two: Some(Identifier::new("buzz").into_lazy_box()),
            using_accessor: false,
        },
    );
}

#[test]
fn ds_map_access() {
    harness_expr(
        "foo[? bar]",
        Access::Map {
            left: Identifier::new("foo").into_lazy_box(),
            key: Identifier::new("bar").into_lazy_box(),
        },
    );
}

#[test]
fn ds_list_access() {
    harness_expr(
        "foo[| bar]",
        Access::List {
            left: Identifier::new("foo").into_lazy_box(),
            index: Identifier::new("bar").into_lazy_box(),
        },
    );
}

#[test]
fn ds_grid_access() {
    harness_expr(
        "foo[# bar, buzz]",
        Access::Grid {
            left: Identifier::new("foo").into_lazy_box(),
            index_one: Identifier::new("bar").into_lazy_box(),
            index_two: Identifier::new("buzz").into_lazy_box(),
        },
    );
}

#[test]
fn ds_grid_access_no_space() {
    harness_expr(
        "foo[#bar, buzz]",
        Access::Grid {
            left: Identifier::new("foo").into_lazy_box(),
            index_one: Identifier::new("bar").into_lazy_box(),
            index_two: Identifier::new("buzz").into_lazy_box(),
        },
    );
}

#[test]
fn struct_access() {
    harness_expr(
        "foo[$ bar]",
        Access::Struct {
            left: Identifier::new("foo").into_lazy_box(),
            key: Identifier::new("bar").into_lazy_box(),
        },
    );
}

#[test]
fn chained_ds_accesses() {
    harness_expr(
        "foo[bar][buzz]",
        Access::Array {
            left: Access::Array {
                left: Identifier::new("foo").into_lazy_box(),
                index_one: Identifier::new("bar").into_lazy_box(),
                index_two: None,
                using_accessor: false,
            }
            .into_lazy_box(),
            index_one: Identifier::new("buzz").into_lazy_box(),
            index_two: None,
            using_accessor: false,
        },
    );
}

#[test]
fn ds_access_call() {
    harness_expr(
        "foo[bar]()",
        Expression::Call(
            Access::Array {
                left: Identifier::new("foo").into_lazy_box(),
                index_one: Identifier::new("bar").into_lazy_box(),
                index_two: None,
                using_accessor: false,
            }
            .into_lazy_box(),
            vec![],
            false,
        ),
    )
}

#[test]
fn dot_access() {
    harness_expr(
        "foo.bar",
        Access::Dot {
            left: Identifier::new("foo").into_lazy_box(),
            right: Identifier::new("bar").into_lazy_box(),
        },
    );
}

#[test]
fn grouping_dot_access() {
    harness_expr(
        "(foo).bar",
        Access::Dot {
            left: Expression::Grouping(Identifier::new("foo").into_lazy_box()).lazy_box(),
            right: Identifier::new("bar").into_lazy_box(),
        },
    );
}

#[test]
fn chained_dot_access() {
    harness_expr(
        "foo.bar.buzz",
        Access::Dot {
            left: Access::Dot {
                left: Identifier::new("foo").into_lazy_box(),
                right: Identifier::new("bar").into_lazy_box(),
            }
            .into_lazy_box(),
            right: Identifier::new("buzz").into_lazy_box(),
        },
    );
}

#[test]
fn dot_access_to_call() {
    harness_expr(
        "foo.bar()",
        Expression::Call(
            Access::Dot {
                left: Identifier::new("foo").into_lazy_box(),
                right: Identifier::new("bar").into_lazy_box(),
            }
            .into_lazy_box(),
            vec![],
            false,
        ),
    )
}

#[test]
fn dot_access_to_ds_access() {
    harness_expr(
        "foo.bar[0]",
        Access::Array {
            left: Access::Dot {
                left: Identifier::new("foo").into_lazy_box(),
                right: Identifier::new("bar").into_lazy_box(),
            }
            .into_lazy_box(),
            index_one: Expression::Literal(Literal::Real(0.0)).lazy_box(),
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
            left: Expression::Call(Identifier::new("foo").into_lazy_box(), vec![], false).lazy_box(),
            right: Identifier::new("bar").into_lazy_box(),
        },
    );
}

#[test]
fn chained_calls() {
    harness_expr(
        "foo().bar()",
        Expression::Call(
            Access::Dot {
                left: Expression::Call(Identifier::new("foo").into_lazy_box(), vec![], false).lazy_box(),
                right: Identifier::new("bar").into_lazy_box(),
            }
            .into_lazy_box(),
            vec![],
            false,
        ),
    );
}

#[test]
fn chain_calls_with_call_parameter() {
    harness_expr(
        "foo().bar(buzz())",
        Expression::Call(
            Access::Dot {
                left: Expression::Call(Identifier::new("foo").into_lazy_box(), vec![], false).lazy_box(),
                right: Identifier::new("bar").into_lazy_box(),
            }
            .into_lazy_box(),
            vec![Expression::Call(Identifier::new("buzz").into_lazy_box(), vec![], false).lazy_box()],
            false,
        ),
    )
}

#[test]
fn global_dot_access() {
    harness_expr(
        "global.bar",
        Access::Global {
            right: Identifier::new("bar").into_lazy_box(),
        },
    );
}

#[test]
fn self_dot_access() {
    harness_expr(
        "self.bar",
        Access::Current {
            right: Identifier::new("bar").into_lazy_box(),
        },
    );
}

#[test]
fn general_self_reference() {
    harness_expr(
        "foo = self",
        Assignment::new(
            Identifier::new("foo").into_lazy_box(),
            AssignmentOperator::Equal,
            Identifier::new("self").into_lazy_box(),
        ),
    );
}

#[test]
fn ds_dot_access() {
    harness_expr(
        "foo[0].bar",
        Access::Dot {
            left: Access::Array {
                left: Identifier::new("foo").into_lazy_box(),
                index_one: Expression::Literal(Literal::Real(0.0)).lazy_box(),
                index_two: None,
                using_accessor: false,
            }
            .into_lazy_box(),
            right: Identifier::new("bar").into_lazy_box(),
        },
    );
}

#[test]
fn grouping() {
    harness_expr(
        "(0)",
        Expression::Grouping(Expression::Literal(Literal::Real(0.0)).lazy_box()),
    );
}

#[test]
fn identifier() {
    harness_expr("foo", Identifier::new("foo"));
}

#[test]
fn number() {
    harness_expr("0", Expression::Literal(Literal::Real(0.0)));
}

#[test]
fn float() {
    harness_expr("0.01", Expression::Literal(Literal::Real(0.01)));
}

#[test]
fn float_no_prefix() {
    harness_expr(".01", Expression::Literal(Literal::Real(0.01)));
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
                Identifier::new("foo").into_lazy_box(),
                EqualityOperator::Equal,
                Expression::Literal(Literal::Real(1.0)).lazy_box(),
            )
            .into_lazy_box(),
            LogicalOperator::And,
            Logical::new(
                Equality::new(
                    Identifier::new("foo").into_lazy_box(),
                    EqualityOperator::Equal,
                    Expression::Literal(Literal::Real(1.0)).lazy_box(),
                )
                .into_lazy_box(),
                LogicalOperator::And,
                Equality::new(
                    Identifier::new("foo").into_lazy_box(),
                    EqualityOperator::Equal,
                    Expression::Literal(Literal::Real(1.0)).lazy_box(),
                )
                .into_lazy_box(),
            )
            .into_lazy_box(),
        ),
    );
}
