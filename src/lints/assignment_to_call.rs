use crate::{parsing::expression::Expression, utils::Span, Duck, Lint, LintCategory, LintReport, lint::EarlyExpressionPass};

#[derive(Debug, PartialEq)]
pub struct AssignmentToCall;
impl Lint for AssignmentToCall {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
            display_name: "Assignment to call".into(),
            tag: Self::tag(),
            explanation: "While possible to compile, assigning a value to the call of a function does not do anything.",
            suggestions: vec!["Re-evaluate this code -- this assignment does not do anything.".into()],
            category: Self::category(),
            span,
        }
    }

    fn category() -> LintCategory {
        LintCategory::Suspicious
    }

    fn tag() -> &'static str {
        "assignment_to_call"
    }
}

impl EarlyExpressionPass for AssignmentToCall {
    fn visit_expression_early(
        _duck: &Duck,
        expression: &Expression,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Expression::Assignment(left, ..) = expression {
            if let Expression::Call(..) = left.expression() {
                reports.push(Self::generate_report(span));
            }
        }
    }
}
