use crate::{
    lint::EarlyStatementPass, parsing::statement::Statement, utils::Span, Lint, LintReport, LintLevel,
};

#[derive(Debug, PartialEq)]
pub struct SingleSwitchCase;
impl Lint for SingleSwitchCase {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
			display_name: "Single switch case".into(),
            tag: Self::tag(),
			explanation: "Switch statements that only match on a single element can be reduced to an `if` statement.",
			suggestions: vec!["Use an `if` statement instead of a `switch` statement".into()],
			default_level: Self::default_level(),
			span,
		}
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
        statement: &crate::parsing::statement::Statement,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Statement::Switch(switch) = statement {
            if switch.cases().len() == 1 {
                reports.push(Self::generate_report(span));
            }
        }
    }
}
