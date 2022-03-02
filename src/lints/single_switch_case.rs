use crate::{
    lint::{EarlyStatementPass, Lint, LintLevel, LintReport},
    parsing::Statement,
    utils::Span,
};

#[derive(Debug, PartialEq)]
pub struct SingleSwitchCase;
impl Lint for SingleSwitchCase {
    fn explanation() -> &'static str {
        "Switch statements that only match on a single element can be reduced to an `if` statement."
    }

    fn default_level() -> LintLevel {
        LintLevel::Warn
    }

    fn tag() -> &'static str {
        "single_switch_case"
    }
}

impl EarlyStatementPass for SingleSwitchCase {
    fn visit_statement_early(
        _config: &crate::Config,
        statement: &crate::parsing::Statement,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Statement::Switch(switch) = statement {
            if switch.cases().len() == 1 {
                Self::report(
                    "Single switch case",
                    ["Use an `if` statement instead of a `switch` statement".into()],
                    span,
                    reports,
                );
            }
        }
    }
}
