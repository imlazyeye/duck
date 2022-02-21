use crate::{parsing::statement::Statement, Duck, Lint, LintCategory, LintReport, Span};

#[derive(Debug, PartialEq)]
pub struct SingleSwitchCase;
impl Lint for SingleSwitchCase {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
			display_name: "Single switch case",
			tag: "single_switch_case",
			explanation: "Switch statements that only match on a single element can be reduced to an `if` statement.",
			suggestions: vec!["Use an `if` statement instead of a `switch` statement"],
			category: LintCategory::Style,
			span,
		}
    }

    fn visit_statement(
        _duck: &Duck,
        statement: &crate::parsing::statement::Statement,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Statement::Switch(_, cases, _) = statement {
            if cases.len() == 1 {
                reports.push(Self::generate_report(span));
            }
        }
    }
}
