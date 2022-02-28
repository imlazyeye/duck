use crate::{
    lint::EarlyExpressionPass,
    parsing::{EqualityOperator, Expression, Literal},
    utils::Span,
    Lint, LintLevel, LintReport,
};

#[derive(Debug, PartialEq)]
pub struct BoolEquality;
impl Lint for BoolEquality {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
            display_name: "Equality check on bool".into(),
            tag: Self::tag(),
            explanation: "Comparing a bool with a bool literal is more verbose than neccesary.",
            suggestions: vec![],
            default_level: Self::default_level(),
            span,
        }
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "bool_equality"
    }
}

impl EarlyExpressionPass for BoolEquality {
    fn visit_expression_early(
        _config: &crate::Config,
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
