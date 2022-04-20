use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyStmtPass, Lint, LintLevel},
    parse::{Stmt, StmtKind},
    FileId,
};

#[derive(Debug, PartialEq)]
pub struct Exit;
impl Lint for Exit {
    fn explanation() -> &'static str {
        "`return` can always be used in place of exit, which provides more consistency across your codebase."
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "exit"
    }
}

impl EarlyStmtPass for Exit {
    fn visit_stmt_early(stmt: &Stmt, config: &crate::Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let StmtKind::Exit = stmt.inner() {
            reports.push(Self::diagnostic(config).with_message("Use of `exit`").with_labels(vec![
                Label::primary(stmt.file_id(), stmt.span()).with_message("replace this with `return`"),
            ]));
        }
    }
}
