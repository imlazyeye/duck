use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyStatementPass, Lint, LintLevel},
    parse::{Statement, StatementBox},
    FileId,
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
        statement_box: &StatementBox,
        config: &crate::Config,
        reports: &mut Vec<Diagnostic<FileId>>,
    ) {
        if let Statement::TryCatch(..) = statement_box.statement() {
            reports.push(
                Self::diagnostic(config)
                    .with_message("Use of `try` / `catch`")
                    .with_labels(vec![Label::primary(statement_box.file_id(), statement_box.span())]),
            );
        }
    }
}
