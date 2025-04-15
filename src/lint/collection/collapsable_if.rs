use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    Config, FileId,
    lint::{EarlyStmtPass, Lint, LintLevel},
    parse::{If, Stmt, StmtKind},
};

#[derive(Debug, PartialEq)]
pub struct CollapsableIf;
impl Lint for CollapsableIf {
    fn explanation() -> &'static str {
        "If statements that contain nothing more than another if statement can be collapsed into a single statement."
    }

    fn default_level() -> LintLevel {
        LintLevel::Warn
    }

    fn tag() -> &'static str {
        "collapsable_if"
    }
}

impl EarlyStmtPass for CollapsableIf {
    fn visit_stmt_early(stmt: &Stmt, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let StmtKind::If(If {
            body: first_body,
            else_stmt: None,
            ..
        }) = stmt.kind()
        {
            if let Some(block) = first_body.kind().as_block().filter(|block| block.body.len() == 1) {
                let nested_stmt = block.body.first().unwrap();
                if let StmtKind::If(If { else_stmt: None, .. }) = nested_stmt.kind() {
                    reports.push(
                        Self::diagnostic(config)
                            .with_message("Collapsable if statement")
                            .with_labels(vec![
                                Label::secondary(nested_stmt.file_id(), nested_stmt.span())
                                    .with_message("nested if statement"),
                                Label::primary(stmt.file_id(), stmt.span())
                                    .with_message("this can be combined with the nested if statement"),
                            ]),
                    )
                }
            }
        }
    }
}
