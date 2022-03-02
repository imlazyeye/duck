use crate::{
    lint::{EarlyExpressionPass, Lint, LintLevel, LintReport},
    parsing::{Assignment, Expression},
    utils::Span,
};

#[derive(Debug, PartialEq)]
pub struct AssignmentToCall;
impl Lint for AssignmentToCall {
    fn explanation() -> &'static str {
        "While possible to compile, assigning a value to the call of a function does not do anything."
    }

    fn default_level() -> LintLevel {
        LintLevel::Deny
    }

    fn tag() -> &'static str {
        "assignment_to_call"
    }
}

impl EarlyExpressionPass for AssignmentToCall {
    fn visit_expression_early(
        _config: &crate::Config,
        expression: &Expression,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Expression::Assignment(Assignment { left, .. }) = expression {
            if let Expression::Call(..) = left.expression() {
                Self::report(
                    "Assignment to call",
                    ["Re-evaluate this code -- this assignment does not do anything.".into()],
                    span,
                    reports,
                );
            }
        }
    }
}
