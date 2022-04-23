use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyStmtPass, Lint, LintLevel},
    parse::{Stmt, StmtKind},
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

impl EarlyStmtPass for TryCatch {
    fn visit_stmt_early(stmt: &Stmt, config: &crate::Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let StmtKind::TryCatch(..) = stmt.kind() {
            reports.push(
                Self::diagnostic(config)
                    .with_message("Use of `try` / `catch`")
                    .with_labels(vec![Label::primary(stmt.file_id(), stmt.span())]),
            );
        }
    }
}
