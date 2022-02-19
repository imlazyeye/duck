use duck::parsing::parser::Parser;
use duck::parsing::{
    expression::{
        AssignmentOperator, DsAccess, EvaluationOperator, Expression, Literal, PostfixOperator,
        UnaryOperator,
    },
    OldParser,
};
use pretty_assertions::assert_eq;

fn harness_expr(source: &str, expected: Expression) {
    let parser = OldParser::new(source.into(), "test".into());
    let mut parser = Parser::new(source.into(), "test".into());
    assert_eq!(*parser.expression().unwrap(), expected);
}

#[test]
fn addition() {
    harness_expr(
        "1 + 1",
        Expression::Evaluation(
            Expression::Literal(Literal::Real(1.0)).into(),
            EvaluationOperator::Plus,
            Expression::Literal(Literal::Real(1.0)).into(),
        ),
    );
}

#[test]
fn subtraction() {
    harness_expr(
        "1 - 1",
        Expression::Evaluation(
            Expression::Literal(Literal::Real(1.0)).into(),
            EvaluationOperator::Minus,
            Expression::Literal(Literal::Real(1.0)).into(),
        ),
    );
}

#[test]
fn multiplication() {
    harness_expr(
        "1 * 1",
        Expression::Evaluation(
            Expression::Literal(Literal::Real(1.0)).into(),
            EvaluationOperator::Star,
            Expression::Literal(Literal::Real(1.0)).into(),
        ),
    );
}

#[test]
fn division() {
    harness_expr(
        "1 / 1",
        Expression::Evaluation(
            Expression::Literal(Literal::Real(1.0)).into(),
            EvaluationOperator::Slash,
            Expression::Literal(Literal::Real(1.0)).into(),
        ),
    );
}

#[test]
fn modulo() {
    harness_expr(
        "1 mod 1",
        Expression::Evaluation(
            Expression::Literal(Literal::Real(1.0)).into(),
            EvaluationOperator::Mod,
            Expression::Literal(Literal::Real(1.0)).into(),
        ),
    );
    harness_expr(
        "1 % 1",
        Expression::Evaluation(
            Expression::Literal(Literal::Real(1.0)).into(),
            EvaluationOperator::Mod,
            Expression::Literal(Literal::Real(1.0)).into(),
        ),
    );
}

#[test]
fn div() {
    harness_expr(
        "1 div 1",
        Expression::Evaluation(
            Expression::Literal(Literal::Real(1.0)).into(),
            EvaluationOperator::Div,
            Expression::Literal(Literal::Real(1.0)).into(),
        ),
    );
}

#[test]
fn less_than() {
    harness_expr(
        "1 < 1",
        Expression::Evaluation(
            Expression::Literal(Literal::Real(1.0)).into(),
            EvaluationOperator::LessThan,
            Expression::Literal(Literal::Real(1.0)).into(),
        ),
    );
}

#[test]
fn less_than_or_equal() {
    harness_expr(
        "1 <= 1",
        Expression::Evaluation(
            Expression::Literal(Literal::Real(1.0)).into(),
            EvaluationOperator::LessThanOrEqual,
            Expression::Literal(Literal::Real(1.0)).into(),
        ),
    );
}

#[test]
fn greater_than() {
    harness_expr(
        "1 > 1",
        Expression::Evaluation(
            Expression::Literal(Literal::Real(1.0)).into(),
            EvaluationOperator::GreaterThan,
            Expression::Literal(Literal::Real(1.0)).into(),
        ),
    );
}

#[test]
fn greater_than_or_equal() {
    harness_expr(
        "1 >= 1",
        Expression::Evaluation(
            Expression::Literal(Literal::Real(1.0)).into(),
            EvaluationOperator::GreaterThanOrEqual,
            Expression::Literal(Literal::Real(1.0)).into(),
        ),
    );
}

#[test]
fn and() {
    harness_expr(
        "1 && 1",
        Expression::Evaluation(
            Expression::Literal(Literal::Real(1.0)).into(),
            EvaluationOperator::And,
            Expression::Literal(Literal::Real(1.0)).into(),
        ),
    );
}

#[test]
fn or() {
    harness_expr(
        "1 || 1",
        Expression::Evaluation(
            Expression::Literal(Literal::Real(1.0)).into(),
            EvaluationOperator::Or,
            Expression::Literal(Literal::Real(1.0)).into(),
        ),
    );
}

#[test]
fn equals() {
    harness_expr(
        "1 == 1",
        Expression::Evaluation(
            Expression::Literal(Literal::Real(1.0)).into(),
            EvaluationOperator::Equals,
            Expression::Literal(Literal::Real(1.0)).into(),
        ),
    );
}

#[test]
fn ternary() {
    harness_expr(
        "foo ? 1 : 2",
        Expression::Ternary(
            Expression::Identifier("foo".into()).into(),
            Expression::Literal(Literal::Real(1.0)).into(),
            Expression::Literal(Literal::Real(2.0)).into(),
        ),
    );
}

#[test]
fn assign() {
    harness_expr(
        "foo = 1",
        Expression::Assignment(
            Expression::Identifier("foo".into()).into(),
            AssignmentOperator::Equals,
            Expression::Literal(Literal::Real(1.0)).into(),
        ),
    );
}

#[test]
fn plus_equals() {
    harness_expr(
        "foo += 1",
        Expression::Assignment(
            Expression::Identifier("foo".into()).into(),
            AssignmentOperator::PlusEquals,
            Expression::Literal(Literal::Real(1.0)).into(),
        ),
    );
}

#[test]
fn minus_equals() {
    harness_expr(
        "foo -= 1",
        Expression::Assignment(
            Expression::Identifier("foo".into()).into(),
            AssignmentOperator::MinusEquals,
            Expression::Literal(Literal::Real(1.0)).into(),
        ),
    );
}

#[test]
fn star_equals() {
    harness_expr(
        "foo *= 1",
        Expression::Assignment(
            Expression::Identifier("foo".into()).into(),
            AssignmentOperator::StarEquals,
            Expression::Literal(Literal::Real(1.0)).into(),
        ),
    );
}

#[test]
fn slash_equals() {
    harness_expr(
        "foo /= 1",
        Expression::Assignment(
            Expression::Identifier("foo".into()).into(),
            AssignmentOperator::SlashEquals,
            Expression::Literal(Literal::Real(1.0)).into(),
        ),
    );
}

#[test]
fn not() {
    harness_expr(
        "!foo",
        Expression::Unary(
            UnaryOperator::Not,
            Expression::Identifier("foo".into()).into(),
        ),
    );
}

#[test]
fn neagtive() {
    harness_expr(
        "-1",
        Expression::Unary(
            UnaryOperator::Negative,
            Expression::Literal(Literal::Real(1.0)).into(),
        ),
    );
}

#[test]
fn increment() {
    harness_expr(
        "1++",
        Expression::Postfix(
            Expression::Literal(Literal::Real(1.0)).into(),
            PostfixOperator::Increment,
        ),
    );
}

#[test]
fn decrement() {
    harness_expr(
        "1--",
        Expression::Postfix(
            Expression::Literal(Literal::Real(1.0)).into(),
            PostfixOperator::Decrement,
        ),
    );
}

#[test]
fn call() {
    harness_expr(
        "foo()",
        Expression::Call(Expression::Identifier("foo".into()).into(), vec![]),
    );
}

#[test]
fn call_with_args() {
    harness_expr(
        "foo(0, 1, 2)",
        Expression::Call(
            Expression::Identifier("foo".into()).into(),
            vec![
                Expression::Literal(Literal::Real(0.0)).into(),
                Expression::Literal(Literal::Real(1.0)).into(),
                Expression::Literal(Literal::Real(2.0)).into(),
            ],
        ),
    );
}

#[test]
fn empty_array() {
    harness_expr("[]", Expression::ArrayLiteral(vec![]));
}

#[test]
fn simple_array() {
    harness_expr(
        "[0, 1, 2]",
        Expression::ArrayLiteral(vec![
            Expression::Literal(Literal::Real(0.0)).into(),
            Expression::Literal(Literal::Real(1.0)).into(),
            Expression::Literal(Literal::Real(2.0)).into(),
        ]),
    );
}

#[test]
fn empty_struct() {
    harness_expr("{}", Expression::StructLiteral(vec![]));
}

#[test]
fn simple_struct() {
    harness_expr(
        "{ foo: bar, fizz: buzz }",
        Expression::StructLiteral(vec![
            ("foo".into(), Expression::Identifier("bar".into()).into()),
            ("fizz".into(), Expression::Identifier("buzz".into()).into()),
        ]),
    );
}

#[test]
fn array_access() {
    harness_expr(
        "foo[bar]",
        Expression::DSAccess(
            Expression::Identifier("foo".into()).into(),
            DsAccess::Array(Expression::Identifier("bar".into()).into()),
        ),
    );
}

#[test]
fn ds_map_access() {
    harness_expr(
        "foo[? bar]",
        Expression::DSAccess(
            Expression::Identifier("foo".into()).into(),
            DsAccess::Map(Expression::Identifier("bar".into()).into()),
        ),
    );
}

#[test]
fn ds_list_access() {
    harness_expr(
        "foo[| bar]",
        Expression::DSAccess(
            Expression::Identifier("foo".into()).into(),
            DsAccess::List(Expression::Identifier("bar".into()).into()),
        ),
    );
}

#[test]
fn ds_grid_access() {
    harness_expr(
        "foo[# bar, buzz]",
        Expression::DSAccess(
            Expression::Identifier("foo".into()).into(),
            DsAccess::Grid(
                Expression::Identifier("bar".into()).into(),
                Expression::Identifier("buzz".into()).into(),
            ),
        ),
    );
}

#[test]
fn dot_access() {
    harness_expr(
        "foo.bar",
        Expression::DotAccess(
            Expression::Identifier("foo".into()).into(),
            Expression::Identifier("bar".into()).into(),
        ),
    );
}

#[test]
fn grouping() {
    harness_expr(
        "(0)",
        Expression::Grouping(Expression::Literal(Literal::Real(0.0)).into()),
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
fn constant() {
    harness_expr("true", Expression::Literal(Literal::True));
    harness_expr("false", Expression::Literal(Literal::False));
}

#[test]
fn string() {
    harness_expr(
        "\"foo\"",
        Expression::Literal(Literal::String("foo".into())),
    );
}
