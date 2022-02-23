use crate::{
    parsing::expression::{AssignmentOperator, Expression, Literal},
    Duck, Lint, LintCategory, LintReport, Span,
};

#[derive(Debug, PartialEq)]
pub struct SuspicousBoolUsage;
impl Lint for SuspicousBoolUsage {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
            tag: Self::tag(),
			display_name: "Susipcious bool usage".into(),
			explanation: "Using a bool outside of equalities and direct assignments is likely unintended or misunderstood code.",
			suggestions: vec![],
			category: Self::category(),
			span,
		}
    }

    fn category() -> LintCategory {
        LintCategory::Suspicious
    }

    fn tag() -> &'static str {
        "suspicious_bool_usage"
    }

    fn visit_expression(
        _duck: &Duck,
        expression: &Expression,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        match expression {
            Expression::Evaluation(_, _, right) => {
                if matches!(
                    right.expression(),
                    Expression::Literal(Literal::True) | Expression::Literal(Literal::False)
                ) {
                    reports.push(Self::generate_report(span))
                }
            }
            Expression::Assignment(_, operator, right) => {
                if operator != &AssignmentOperator::Equal
                    && matches!(
                        right.expression(),
                        Expression::Literal(Literal::True) | Expression::Literal(Literal::False)
                    )
                {
                    reports.push(Self::generate_report(span))
                }
            }
            _ => {}
        }
    }
}
