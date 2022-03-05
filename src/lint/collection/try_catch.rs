use crate::{
    lint::{EarlyStatementPass, Lint, LintLevel, LintReport},
    parse::Statement,
    parse::Span,
};

#[derive(Debug, PartialEq)]
pub struct TryCatch;
impl Lint for TryCatch {
    fn explanation() -> &'static str {
        "GML's try/catch will collect all errors as opposed to the precise ones wanted, allowing them to accidently catch errors that should not be surpressed."
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
        statement: &crate::parse::Statement,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Statement::TryCatch(..) = statement {
            Self::report(
                "Use of `try` / `catch`",
                ["Adjust the architecture to inspect for an issue prior to the crash".into()],
                span,
                reports,
            )
        }
    }
}
