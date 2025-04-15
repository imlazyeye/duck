use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    FileId,
    lint::{EarlyStmtPass, Lint, LintLevel},
    parse::{Stmt, StmtKind, Switch},
};

#[derive(Debug, PartialEq)]
pub struct SwitchWithoutCase;
impl Lint for SwitchWithoutCase {
    fn explanation() -> &'static str {
        "A switch statement is unncessary if it contains now cases."
    }

    fn default_level() -> LintLevel {
        LintLevel::Warn
    }

    fn tag() -> &'static str {
        "switch_without_case"
    }
}

impl EarlyStmtPass for SwitchWithoutCase {
    fn visit_stmt_early(stmt: &Stmt, config: &crate::Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let StmtKind::Switch(Switch { cases, .. }) = stmt.kind() {
            if cases.is_empty() {
                reports.push(
                    Self::diagnostic(config)
                        .with_message("Switch statement with no cases")
                        .with_labels(vec![
                            Label::primary(stmt.file_id(), stmt.span()).with_message("Remove this switch statement"),
                        ]),
                );
            }
        }
    }
}
