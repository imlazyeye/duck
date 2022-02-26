use crate::parsing::expression::{
    AssignmentOperator, Constructor, EqualityOperator, EvaluationOperator, Expression, Literal,
    LogicalOperator, Parameter, PostfixOperator, Scope, UnaryOperator,
};
use crate::parsing::parser::Parser;
use crate::parsing::statement::Statement;
use colored::Colorize;
//use pretty_assertions::assert_eq;

fn harness_expr(source: &str, expected: Expression) {
    let mut parser = Parser::new(source, "test".into());
    let outputed = parser.expression().unwrap();
    if *outputed.expression() != expected {
        panic!(
            "\n{}\n\n{}\n\n{}: {:?}\n\n{}: {:?}\n",
            "Failed a test on the following gml: ".yellow().bold(),
            source,
            "Expected".green().bold(),
            expected,
            "Outputed".red().bold(),
            *outputed.expression(),
        )
    }
}

#[test]
fn function() {
    harness_expr(
        "function foo() {}",
        Expression::FunctionDeclaration(
            Some("foo".into()),
            vec![],
            None,
            Statement::Block(vec![]).lazy_box(),
            false,
        ),
    )
}

#[test]
fn static_function() {
    harness_expr(
        "static function foo() {}",
        Expression::FunctionDeclaration(
            Some("foo".into()),
            vec![],
            None,
            Statement::Block(vec![]).lazy_box(),
            true,
        ),
    )
}

#[test]
fn function_with_parameters() {
    harness_expr(
        "function foo(bar, baz) {}",
        Expression::FunctionDeclaration(
            Some("foo".into()),
            vec![Parameter("bar".into(), None), Parameter("baz".into(), None)],
            None,
            Statement::Block(vec![]).lazy_box(),
            false,
        ),
    )
}

#[test]
fn default_parameters() {
    harness_expr(
        "function foo(bar=20, baz) {}",
        Expression::FunctionDeclaration(
            Some("foo".into()),
            vec![
                Parameter(
                    "bar".into(),
                    Some(Expression::Literal(Literal::Real(20.0)).lazy_box()),
                ),
                Parameter("baz".into(), None),
            ],
            None,
            Statement::Block(vec![]).lazy_box(),
            false,
        ),
    )
}

#[test]
fn anonymous_function() {
    harness_expr(
        "function() {}",
        Expression::FunctionDeclaration(
            None,
            vec![],
            None,
            Statement::Block(vec![]).lazy_box(),
            false,
        ),
    )
}

#[test]
fn constructor() {
    harness_expr(
        "function foo() constructor {}",
        Expression::FunctionDeclaration(
            Some("foo".into()),
            vec![],
            Some(Constructor(None)),
            Statement::Block(vec![]).lazy_box(),
            false,
        ),
    )
}

#[test]
fn inheritance() {
    harness_expr(
        "function foo() : bar() constructor {}",
        Expression::FunctionDeclaration(
            Some("foo".into()),
            vec![],
            Some(Constructor(Some(
                Expression::Call(
                    Expression::Identifier("bar".into()).lazy_box(),
                    vec![],
                    false,
                )
                .lazy_box(),
            ))),
            Statement::Block(vec![]).lazy_box(),
            false,
        ),
    )
}

#[test]
fn function_return_no_semi_colon() {
    harness_expr(
        "function() { return }",
        Expression::FunctionDeclaration(
            None,
            vec![],
            None,
            Statement::Block(vec![Statement::Return(None).lazy_box()]).lazy_box(),
            false,
        ),
    )
}

#[test]
fn and() {
    harness_expr(
        "1 && 1",
        Expression::Logical(
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
        Expression::Logical(
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
        Expression::Logical(
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
        Expression::Logical(
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
        Expression::Logical(
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
        Expression::Evaluation(
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
        Expression::Evaluation(
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
        Expression::Evaluation(
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
        Expression::Evaluation(
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
        Expression::Evaluation(
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
            EvaluationOperator::Modulo,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
    harness_expr(
        "1 % 1",
        Expression::Evaluation(
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
        Expression::Evaluation(
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
        Expression::Evaluation(
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
        Expression::Evaluation(
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
        Expression::Evaluation(
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
            EvaluationOperator::Or,
            Expression::Evaluation(
                Expression::Literal(Literal::Real(1.0)).lazy_box(),
                EvaluationOperator::Or,
                Expression::Literal(Literal::Real(1.0)).lazy_box(),
            )
            .lazy_box(),
        ),
    );
}

#[test]
fn bitwise_xor() {
    harness_expr(
        "1 ^ 1",
        Expression::Evaluation(
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
        Expression::Evaluation(
            Expression::Access(
                Scope::Dot(Expression::Identifier("foo".into()).lazy_box()),
                Expression::Identifier("bar".into()).lazy_box(),
            )
            .lazy_box(),
            EvaluationOperator::Or,
            Expression::Access(
                Scope::Dot(Expression::Identifier("foo".into()).lazy_box()),
                Expression::Identifier("bar".into()).lazy_box(),
            )
            .lazy_box(),
        ),
    );
}

#[test]
fn bit_shift_left() {
    harness_expr(
        "1 << 1",
        Expression::Evaluation(
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
        Expression::Evaluation(
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
        Expression::Equality(
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
        Expression::Equality(
            Expression::Evaluation(
                Expression::Evaluation(
                    Expression::Evaluation(
                        Expression::Evaluation(
                            Expression::Literal(Literal::Real(1.0)).lazy_box(),
                            EvaluationOperator::Star,
                            Expression::Literal(Literal::Real(1.0)).lazy_box(),
                        )
                        .lazy_box(),
                        EvaluationOperator::Plus,
                        Expression::Literal(Literal::Real(1.0)).lazy_box(),
                    )
                    .lazy_box(),
                    EvaluationOperator::BitShiftRight,
                    Expression::Literal(Literal::Real(1.0)).lazy_box(),
                )
                .lazy_box(),
                EvaluationOperator::And,
                Expression::Literal(Literal::Real(1.0)).lazy_box(),
            )
            .lazy_box(),
            EqualityOperator::Equal,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    )
}

#[test]
fn less_than_or_equal() {
    harness_expr(
        "1 <= 1",
        Expression::Equality(
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
        Expression::Equality(
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
        Expression::Equality(
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
        Expression::Equality(
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
        Expression::Equality(
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
        Expression::NullCoalecence(
            Expression::Identifier("foo".into()).lazy_box(),
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn ternary() {
    harness_expr(
        "foo ? 1 : 2",
        Expression::Ternary(
            Expression::Identifier("foo".into()).lazy_box(),
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
            Expression::Literal(Literal::Real(2.0)).lazy_box(),
        ),
    );
}

#[test]
fn assign() {
    harness_expr(
        "foo = 1",
        Expression::Assignment(
            Expression::Identifier("foo".into()).lazy_box(),
            AssignmentOperator::Equal,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn dot_assign() {
    harness_expr(
        "self.foo = 1",
        Expression::Assignment(
            Expression::Access(
                Scope::Current,
                Expression::Identifier("foo".into()).lazy_box(),
            )
            .lazy_box(),
            AssignmentOperator::Equal,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn ds_assign() {
    harness_expr(
        "foo[0] = 1",
        Expression::Assignment(
            Expression::Access(
                Scope::Array(
                    Expression::Literal(Literal::Real(0.0)).lazy_box(),
                    None,
                    false,
                ),
                Expression::Identifier("foo".into()).lazy_box(),
            )
            .lazy_box(),
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
        Expression::Assignment(
            Expression::Call(
                Expression::Identifier("foo".into()).lazy_box(),
                vec![],
                false,
            )
            .lazy_box(),
            AssignmentOperator::Equal,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn static_assign() {
    harness_expr(
        "static foo = 1",
        Expression::Assignment(
            Expression::Identifier("foo".into()).lazy_box(),
            AssignmentOperator::Equal,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn plus_equal() {
    harness_expr(
        "foo += 1",
        Expression::Assignment(
            Expression::Identifier("foo".into()).lazy_box(),
            AssignmentOperator::PlusEqual,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn minus_equal() {
    harness_expr(
        "foo -= 1",
        Expression::Assignment(
            Expression::Identifier("foo".into()).lazy_box(),
            AssignmentOperator::MinusEqual,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn star_equal() {
    harness_expr(
        "foo *= 1",
        Expression::Assignment(
            Expression::Identifier("foo".into()).lazy_box(),
            AssignmentOperator::StarEqual,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn slash_equal() {
    harness_expr(
        "foo /= 1",
        Expression::Assignment(
            Expression::Identifier("foo".into()).lazy_box(),
            AssignmentOperator::SlashEqual,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn and_equal() {
    harness_expr(
        "foo &= 1",
        Expression::Assignment(
            Expression::Identifier("foo".into()).lazy_box(),
            AssignmentOperator::AndEqual,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn or_equal() {
    harness_expr(
        "foo |= 1",
        Expression::Assignment(
            Expression::Identifier("foo".into()).lazy_box(),
            AssignmentOperator::OrEqual,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn xor_equal() {
    harness_expr(
        "foo ^= 1",
        Expression::Assignment(
            Expression::Identifier("foo".into()).lazy_box(),
            AssignmentOperator::XorEqual,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn mod_equal() {
    harness_expr(
        "foo %= 1",
        Expression::Assignment(
            Expression::Identifier("foo".into()).lazy_box(),
            AssignmentOperator::ModEqual,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn not() {
    harness_expr(
        "!foo",
        Expression::Unary(
            UnaryOperator::Not,
            Expression::Identifier("foo".into()).lazy_box(),
        ),
    );
}

#[test]
fn not_keyword() {
    harness_expr(
        "not foo",
        Expression::Unary(
            UnaryOperator::Not,
            Expression::Identifier("foo".into()).lazy_box(),
        ),
    );
}

#[test]
fn positive() {
    harness_expr(
        "+1",
        Expression::Unary(
            UnaryOperator::Positive,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn neagtive() {
    harness_expr(
        "-1",
        Expression::Unary(
            UnaryOperator::Negative,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn dot_unary() {
    harness_expr(
        "!self.foo",
        Expression::Unary(
            UnaryOperator::Not,
            Expression::Access(
                Scope::Current,
                Expression::Identifier("foo".into()).lazy_box(),
            )
            .lazy_box(),
        ),
    );
}

#[test]
fn ds_unary() {
    harness_expr(
        "!foo[bar]",
        Expression::Unary(
            UnaryOperator::Not,
            Expression::Access(
                Scope::Array(Expression::Identifier("bar".into()).lazy_box(), None, false),
                Expression::Identifier("foo".into()).lazy_box(),
            )
            .lazy_box(),
        ),
    );
}

#[test]
fn prefix_increment() {
    harness_expr(
        "++1",
        Expression::Unary(
            UnaryOperator::Increment,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn prefix_decrement() {
    harness_expr(
        "--1",
        Expression::Unary(
            UnaryOperator::Decrement,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn bitwise_not() {
    harness_expr(
        "~1",
        Expression::Unary(
            UnaryOperator::BitwiseNot,
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
        ),
    );
}

#[test]
fn postfix_increment() {
    harness_expr(
        "1++",
        Expression::Postfix(
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
            PostfixOperator::Increment,
        ),
    );
}

#[test]
fn postfix_decrement() {
    harness_expr(
        "1--",
        Expression::Postfix(
            Expression::Literal(Literal::Real(1.0)).lazy_box(),
            PostfixOperator::Decrement,
        ),
    );
}

#[test]
fn dot_postfix() {
    harness_expr(
        "self.foo++",
        Expression::Postfix(
            Expression::Access(
                Scope::Current,
                Expression::Identifier("foo".into()).lazy_box(),
            )
            .lazy_box(),
            PostfixOperator::Increment,
        ),
    );
}

#[test]
fn ds_postfix() {
    harness_expr(
        "foo[bar]++",
        Expression::Postfix(
            Expression::Access(
                Scope::Array(Expression::Identifier("bar".into()).lazy_box(), None, false),
                Expression::Identifier("foo".into()).lazy_box(),
            )
            .lazy_box(),
            PostfixOperator::Increment,
        ),
    );
}

#[test]
fn call() {
    harness_expr(
        "foo()",
        Expression::Call(
            Expression::Identifier("foo".into()).lazy_box(),
            vec![],
            false,
        ),
    );
}

#[test]
fn call_with_args() {
    harness_expr(
        "foo(0, 1, 2)",
        Expression::Call(
            Expression::Identifier("foo".into()).lazy_box(),
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
            Expression::Identifier("foo".into()).lazy_box(),
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
        Expression::Call(
            Expression::Identifier("foo".into()).lazy_box(),
            vec![],
            true,
        ),
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
            (
                "foo".into(),
                Expression::Identifier("bar".into()).lazy_box(),
            ),
            (
                "fizz".into(),
                Expression::Identifier("buzz".into()).lazy_box(),
            ),
        ])),
    );
}

#[test]
fn array_access() {
    harness_expr(
        "foo[bar]",
        Expression::Access(
            Scope::Array(Expression::Identifier("bar".into()).lazy_box(), None, false),
            Expression::Identifier("foo".into()).lazy_box(),
        ),
    );
}

#[test]
fn array_direct_access() {
    harness_expr(
        "foo[@ bar]",
        Expression::Access(
            Scope::Array(Expression::Identifier("bar".into()).lazy_box(), None, true),
            Expression::Identifier("foo".into()).lazy_box(),
        ),
    );
}

#[test]
fn array_access_2d() {
    harness_expr(
        "foo[bar, buzz]",
        Expression::Access(
            Scope::Array(
                Expression::Identifier("bar".into()).lazy_box(),
                Some(Expression::Identifier("buzz".into()).lazy_box()),
                false,
            ),
            Expression::Identifier("foo".into()).lazy_box(),
        ),
    );
}

#[test]
fn ds_map_access() {
    harness_expr(
        "foo[? bar]",
        Expression::Access(
            Scope::Map(Expression::Identifier("bar".into()).lazy_box()),
            Expression::Identifier("foo".into()).lazy_box(),
        ),
    );
}

#[test]
fn ds_list_access() {
    harness_expr(
        "foo[| bar]",
        Expression::Access(
            Scope::List(Expression::Identifier("bar".into()).lazy_box()),
            Expression::Identifier("foo".into()).lazy_box(),
        ),
    );
}

#[test]
fn ds_grid_access() {
    harness_expr(
        "foo[# bar, buzz]",
        Expression::Access(
            Scope::Grid(
                Expression::Identifier("bar".into()).lazy_box(),
                Expression::Identifier("buzz".into()).lazy_box(),
            ),
            Expression::Identifier("foo".into()).lazy_box(),
        ),
    );
}

#[test]
fn ds_grid_access_no_space() {
    harness_expr(
        "foo[#bar, buzz]",
        Expression::Access(
            Scope::Grid(
                Expression::Identifier("bar".into()).lazy_box(),
                Expression::Identifier("buzz".into()).lazy_box(),
            ),
            Expression::Identifier("foo".into()).lazy_box(),
        ),
    );
}

#[test]
fn struct_access() {
    harness_expr(
        "foo[$ bar]",
        Expression::Access(
            Scope::Struct(Expression::Identifier("bar".into()).lazy_box()),
            Expression::Identifier("foo".into()).lazy_box(),
        ),
    );
}

#[test]
fn chained_ds_accesses() {
    harness_expr(
        "foo[bar][buzz]",
        Expression::Access(
            Scope::Array(
                Expression::Identifier("buzz".into()).lazy_box(),
                None,
                false,
            ),
            Expression::Access(
                Scope::Array(Expression::Identifier("bar".into()).lazy_box(), None, false),
                Expression::Identifier("foo".into()).lazy_box(),
            )
            .lazy_box(),
        ),
    );
}

#[test]
fn ds_access_call() {
    harness_expr(
        "foo[0]()",
        Expression::Call(
            Expression::Access(
                Scope::Array(
                    Expression::Literal(Literal::Real(0.0)).lazy_box(),
                    None,
                    false,
                ),
                Expression::Identifier("foo".into()).lazy_box(),
            )
            .lazy_box(),
            vec![],
            false,
        ),
    )
}

#[test]
fn dot_access() {
    harness_expr(
        "foo.bar",
        Expression::Access(
            Scope::Dot(Expression::Identifier("foo".into()).lazy_box()),
            Expression::Identifier("bar".into()).lazy_box(),
        ),
    );
}

#[test]
fn grouping_dot_access() {
    harness_expr(
        "(foo).bar",
        Expression::Access(
            Scope::Dot(
                Expression::Grouping(Expression::Identifier("foo".into()).lazy_box()).lazy_box(),
            ),
            Expression::Identifier("bar".into()).lazy_box(),
        ),
    );
}

#[test]
fn chained_dot_access() {
    harness_expr(
        "foo.bar.buzz",
        Expression::Access(
            Scope::Dot(
                Expression::Access(
                    Scope::Dot(Expression::Identifier("foo".into()).lazy_box()),
                    Expression::Identifier("bar".into()).lazy_box(),
                )
                .lazy_box(),
            ),
            Expression::Identifier("buzz".into()).lazy_box(),
        ),
    );
}

#[test]
fn dot_access_to_call() {
    harness_expr(
        "foo.bar()",
        Expression::Call(
            Expression::Access(
                Scope::Dot(Expression::Identifier("foo".into()).lazy_box()),
                Expression::Identifier("bar".into()).lazy_box(),
            )
            .lazy_box(),
            vec![],
            false,
        ),
    )
}

#[test]
fn dot_access_to_ds_access() {
    harness_expr(
        "foo.bar[0]",
        Expression::Access(
            Scope::Array(
                Expression::Literal(Literal::Real(0.0)).lazy_box(),
                None,
                false,
            ),
            Expression::Access(
                Scope::Dot(Expression::Identifier("foo".into()).lazy_box()),
                Expression::Identifier("bar".into()).lazy_box(),
            )
            .lazy_box(),
        ),
    );
}

#[test]
fn dot_access_from_call() {
    harness_expr(
        "foo().bar",
        Expression::Access(
            Scope::Dot(
                Expression::Call(
                    Expression::Identifier("foo".into()).lazy_box(),
                    vec![],
                    false,
                )
                .lazy_box(),
            ),
            Expression::Identifier("bar".into()).lazy_box(),
        ),
    );
}

#[test]
fn chained_calls() {
    harness_expr(
        "foo().bar()",
        Expression::Call(
            Expression::Access(
                Scope::Dot(
                    Expression::Call(
                        Expression::Identifier("foo".into()).lazy_box(),
                        vec![],
                        false,
                    )
                    .lazy_box(),
                ),
                Expression::Identifier("bar".into()).lazy_box(),
            )
            .lazy_box(),
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
            Expression::Access(
                Scope::Dot(
                    Expression::Call(
                        Expression::Identifier("foo".into()).lazy_box(),
                        vec![],
                        false,
                    )
                    .lazy_box(),
                ),
                Expression::Identifier("bar".into()).lazy_box(),
            )
            .lazy_box(),
            vec![Expression::Call(
                Expression::Identifier("buzz".into()).lazy_box(),
                vec![],
                false,
            )
            .lazy_box()],
            false,
        ),
    )
}

#[test]
fn global_dot_access() {
    harness_expr(
        "global.bar",
        Expression::Access(
            Scope::Global,
            Expression::Identifier("bar".into()).lazy_box(),
        ),
    );
}

#[test]
fn self_dot_access() {
    harness_expr(
        "self.bar",
        Expression::Access(
            Scope::Current,
            Expression::Identifier("bar".into()).lazy_box(),
        ),
    );
}

#[test]
fn general_self_reference() {
    harness_expr(
        "foo = self",
        Expression::Assignment(
            Expression::Identifier("foo".into()).lazy_box(),
            AssignmentOperator::Equal,
            Expression::Identifier("self".into()).lazy_box(),
        ),
    );
}

#[test]
fn ds_dot_access() {
    harness_expr(
        "foo[0].bar",
        Expression::Access(
            Scope::Dot(
                Expression::Access(
                    Scope::Array(
                        Expression::Literal(Literal::Real(0.0)).lazy_box(),
                        None,
                        false,
                    ),
                    Expression::Identifier("foo".into()).lazy_box(),
                )
                .lazy_box(),
            ),
            Expression::Identifier("bar".into()).lazy_box(),
        ),
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
    harness_expr("foo", Expression::Identifier("foo".into()));
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
    harness_expr("pointer_null", Expression::Literal(Literal::PointerNull));
    harness_expr(
        "pointer_invalid",
        Expression::Literal(Literal::PointerInvalid),
    );
    harness_expr("undefined", Expression::Literal(Literal::Undefined));
    harness_expr("NaN", Expression::Literal(Literal::NaN));
    harness_expr("infinity", Expression::Literal(Literal::Infinity));
    harness_expr("pi", Expression::Literal(Literal::Pi));
}

#[test]
fn string() {
    harness_expr(
        "\"foo\"",
        Expression::Literal(Literal::String("foo".into())),
    );
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
    harness_expr(
        "$a0f9a0",
        Expression::Literal(Literal::Hex("a0f9a0".into())),
    );
}

#[test]
fn short_hex() {
    harness_expr("$20", Expression::Literal(Literal::Hex("20".into())));
}

#[test]
fn oh_x_hex() {
    harness_expr(
        "0xa0f9a0",
        Expression::Literal(Literal::Hex("a0f9a0".into())),
    );
}

#[test]
fn logically_joined_expressions() {
    harness_expr(
        "foo == 1 && foo == 1 && foo == 1",
        Expression::Logical(
            Expression::Equality(
                Expression::Identifier("foo".into()).lazy_box(),
                EqualityOperator::Equal,
                Expression::Literal(Literal::Real(1.0)).lazy_box(),
            )
            .lazy_box(),
            LogicalOperator::And,
            Expression::Logical(
                Expression::Equality(
                    Expression::Identifier("foo".into()).lazy_box(),
                    EqualityOperator::Equal,
                    Expression::Literal(Literal::Real(1.0)).lazy_box(),
                )
                .lazy_box(),
                LogicalOperator::And,
                Expression::Equality(
                    Expression::Identifier("foo".into()).lazy_box(),
                    EqualityOperator::Equal,
                    Expression::Literal(Literal::Real(1.0)).lazy_box(),
                )
                .lazy_box(),
            )
            .lazy_box(),
        ),
    );
}
