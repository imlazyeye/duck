use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyStmtPass, Lint, LintLevel},
    parse::{Stmt, StmtKind},
    FileId,
};

#[derive(Debug, PartialEq)]
pub struct WithLoop;
impl Lint for WithLoop {
    fn explanation() -> &'static str {
        "The `with` loop allows your code's context to suddenly change, both making it more difficult to read (as a given line of code is no longer promised to be executing in the scope expected from the file), but also making it more difficult to track down all of the places an object is modified."
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "with_loop"
    }
}

impl EarlyStmtPass for WithLoop {
    fn visit_stmt_early(stmt: &Stmt, config: &crate::Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let StmtKind::WithLoop(..) = stmt.inner() {
            reports.push(
                Self::diagnostic(config)
                    .with_message("Use of `with`")
                    .with_labels(vec![Label::primary(stmt.file_id(), stmt.span())]),
            );
        }
    }
}
