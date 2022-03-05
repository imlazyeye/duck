use crate::{
    lint::{EarlyStatementPass, Lint, LintLevel, LintReport},
    parsing::{Assignment, Expression, Statement},
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

impl EarlyStatementPass for AssignmentToCall {
    fn visit_statement_early(
        _config: &crate::Config,
        expression: &Statement,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Statement::Assignment(Assignment { left, .. }) = expression {
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
