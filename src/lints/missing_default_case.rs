use crate::{parsing::statement::Statement, Duck, Lint, LintCategory, LintReport, Span};

#[derive(Debug, PartialEq)]
pub struct MissingDefaultCase;
impl Lint for MissingDefaultCase {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
			display_name: "Missing default case".into(),
            tag: Self::tag(),
			explanation: "Switch statements are often used to express all possible outcomes of a limited data set, but by not implementing a default case, no code will run to handle any alternate or unexpected values.",
			suggestions: vec!["Add a default case to the switch statement".into()],
			category: Self::category(),
			span,
		}
    }

    fn category() -> LintCategory {
        LintCategory::Strict
    }

    fn tag() -> &'static str {
        "missing_default_case"
    }

    fn visit_statement(
        _duck: &Duck,
        statement: &Statement,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Statement::Switch(_, _, None) = statement {
            reports.push(Self::generate_report(span))
        }
    }
}
