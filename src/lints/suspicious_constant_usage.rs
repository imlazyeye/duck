use crate::{
    lint::EarlyExpressionPass,
    parsing::expression::{AssignmentOperator, Expression, Literal},
    utils::Span,
    Lint, LintLevel, LintReport,
};

#[derive(Debug, PartialEq)]
pub struct SuspicousConstantUsage;
impl Lint for SuspicousConstantUsage {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
            tag: Self::tag(),
			display_name: "Susipcious constant usage".into(),
			explanation: "Using a constant outside of equalities and direct assignments is likely unintended or misunderstood code.",
			suggestions: vec![],
			default_level: Self::default_level(),
			span,
		}
    }

    fn default_level() -> LintLevel {
        LintLevel::Deny
    }

    fn tag() -> &'static str {
        "suspicious_constant_usage"
    }
}

impl EarlyExpressionPass for SuspicousConstantUsage {
    fn visit_expression_early(
        _config: &crate::Config,
        expression: &Expression,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        match expression {
            Expression::Evaluation(_, _, right) => {
                if let Expression::Literal(literal) = right.expression() {
                    if literal_is_suspicous(literal) {
                        reports.push(Self::generate_report(span))
                    }
                }
            }
            Expression::Assignment(_, operator, right) => {
                if !matches!(
                    *operator,
                    AssignmentOperator::Equal | AssignmentOperator::NullCoalecenceEqual
                ) {
                    if let Expression::Literal(literal) = right.expression() {
                        if literal_is_suspicous(literal) {
                            reports.push(Self::generate_report(span))
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn literal_is_suspicous(literal: &Literal) -> bool {
    match literal {
        Literal::True
        | Literal::False
        | Literal::Undefined
        | Literal::Noone
        | Literal::Misc(_)
        | Literal::Array(_)
        | Literal::Struct(_) => true,
        Literal::String(_) | Literal::Real(_) | Literal::Hex(_) => false,
    }
}
