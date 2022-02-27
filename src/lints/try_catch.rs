use crate::{
    lint::EarlyStatementPass, parsing::statement::Statement, utils::Span, Lint, LintReport, LintLevel,
};

#[derive(Debug, PartialEq)]
pub struct TryCatch;
impl Lint for TryCatch {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
			display_name: "Use of `try` / `catch`".into(),
            tag: Self::tag(),
			explanation: "GML's try/catch will collect all errors as opposed to the precise ones wanted, allowing them to accidently catch errors that should not be surpressed.",
			suggestions: vec!["Adjust the architecture to inspect for an issue prior to the crash".into()],
			default_level: Self::default_level(),
			span,
		}
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "try_catch"
    }
}

impl EarlyStatementPass for TryCatch {
    fn visit_statement_early(
        _config: &crate::Config,
        statement: &crate::parsing::statement::Statement,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Statement::TryCatch(..) = statement {
            reports.push(Self::generate_report(span))
        }
    }
}
