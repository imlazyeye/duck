use crate::{parsing::statement::Statement, utils::Span, Duck, Lint, LintCategory, LintReport};

#[derive(Debug, PartialEq)]
pub struct SingleSwitchCase;
impl Lint for SingleSwitchCase {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
			display_name: "Single switch case".into(),
            tag: Self::tag(),
			explanation: "Switch statements that only match on a single element can be reduced to an `if` statement.",
			suggestions: vec!["Use an `if` statement instead of a `switch` statement".into()],
			category: Self::category(),
			span,
		}
    }

    fn category() -> LintCategory {
        LintCategory::Style
    }

    fn tag() -> &'static str {
        "single_switch_case"
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
