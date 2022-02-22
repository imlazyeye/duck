use crate::{parsing::statement::Statement, Duck, Lint, LintCategory, LintReport, Span};

#[derive(Debug, PartialEq)]
pub struct TryCatch;
impl Lint for TryCatch {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
			display_name: "Use of `try` / `catch`".into(),
            tag: Self::tag(),
			explanation: "GML's try/catch will collect all errors as opposed to the precise ones wanted, allowing them to accidently catch errors that should not be surpressed.",
			suggestions: vec!["Adjust the architecture to inspect for an issue prior to the crash".into()],
			category: Self::category(),
			span,
		}
    }

    fn category() -> LintCategory {
        LintCategory::Pedantic
    }

    fn tag() -> &'static str {
        "try_catch"
    }

    fn visit_statement(
        _duck: &Duck,
        statement: &crate::parsing::statement::Statement,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Statement::TryCatch(..) = statement {
            reports.push(Self::generate_report(span))
        }
    }
}
