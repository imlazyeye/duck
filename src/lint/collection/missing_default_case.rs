use crate::{
    lint::{EarlyStatementPass, Lint, LintLevel, LintReport},
    parse::Statement,
    parse::Span,
};

#[derive(Debug, PartialEq)]
pub struct MissingDefaultCase;
impl Lint for MissingDefaultCase {
    fn explanation() -> &'static str {
        "Switch statements are often used to express all possible outcomes of a limited data set, but by not implementing a default case, no code will run to handle any alternate or unexpected values."
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "missing_default_case"
    }
}

impl EarlyStatementPass for MissingDefaultCase {
    fn visit_statement_early(
        _config: &crate::Config,
        statement: &Statement,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Statement::Switch(switch) = statement {
            if switch.default_case().is_none() {
                Self::report(
                    "Missing default case",
                    ["Add a default case to the switch statement".into()],
                    span,
                    reports,
                )
            }
        }
    }
}
