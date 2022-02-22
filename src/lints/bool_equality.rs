use crate::{
    parsing::expression::{EqualityOperator, Expression, Literal},
    Duck, Lint, LintCategory, LintReport, Span,
};

#[derive(Debug, PartialEq)]
pub struct BoolEquality;
impl Lint for BoolEquality {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
            display_name: "Equality check on bool".into(),
            tag: "bool_equality",
            explanation: "Comparing a bool with a bool literal is more verbose than neccesary.",
            suggestions: vec![],
            category: LintCategory::Style,
            span,
        }
    }

    fn visit_expression(
        _duck: &Duck,
        expression: &Expression,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Expression::Equality(_, EqualityOperator::Equal, right) = expression {
            if let Expression::Literal(literal) = right.expression() {
                match literal {
                    Literal::True => reports.push(Self::generate_report_with(
                        span,
                        "Equality check with `true`",
                        ["Remove the `== true`".into()],
                    )),
                    Literal::False => reports.push(Self::generate_report_with(
                        span,
                        "Equality check with `false`",
                        ["Remove the `== false` and se `!foo` syntax instead".into()],
                    )),
                    _ => {}
                }
            }
        }
    }
}
