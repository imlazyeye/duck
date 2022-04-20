use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyStmtPass, Lint, LintLevel},
    parse::{Stmt, StmtKind, Switch},
    FileId,
};

#[derive(Debug, PartialEq)]
pub struct SingleSwitchCase;
impl Lint for SingleSwitchCase {
    fn explanation() -> &'static str {
        "Switch statements that only match on a single element can be reduced to an `if` statement."
    }

    fn default_level() -> LintLevel {
        LintLevel::Warn
    }

    fn tag() -> &'static str {
        "single_switch_case"
    }
}

impl EarlyStmtPass for SingleSwitchCase {
    fn visit_stmt_early(stmt: &Stmt, config: &crate::Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let StmtKind::Switch(Switch {
            cases, default_case, ..
        }) = stmt.inner()
        {
            if cases.len() == 1 {
                if default_case.is_some() {
                    reports.push(
                        Self::diagnostic(config)
                            .with_message("Switch statement with single case")
                            .with_labels(vec![
                                Label::primary(stmt.file_id(), stmt.span())
                                    .with_message("Use an `if/else` statement instead of a `switch` statement"),
                            ]),
                    );
                } else {
                    reports.push(
                        Self::diagnostic(config)
                            .with_message("Switch statement with single case")
                            .with_labels(vec![
                                Label::primary(stmt.file_id(), stmt.span())
                                    .with_message("Use an `if` statement instead of a `switch` statement"),
                            ]),
                    );
                }
            }
        }
    }
}
