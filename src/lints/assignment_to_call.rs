use crate::{parsing::expression::Expression, Duck, Lint, LintCategory, LintReport, Span};

#[derive(Debug, PartialEq)]
pub struct AssignmentToCall;
impl Lint for AssignmentToCall {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
            display_name: "Assignment to call".into(),
            tag: "assignment_to_call",
            explanation: "While possible to compile, assigning a value to the call of a function does not do anything.",
            suggestions: vec!["Re-evaluate this code -- this assignment does not do anything.".into()],
            category: LintCategory::Suspicious,
            span,
        }
    }

    fn visit_expression(
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
