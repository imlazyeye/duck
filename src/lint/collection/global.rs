use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyStmtPass, Lint, LintLevel},
    parse::{Access, Assignment, ExprType, Globalvar, Stmt, StmtType},
    FileId,
};

#[derive(Debug, PartialEq)]
pub struct Global;
impl Lint for Global {
    fn explanation() -> &'static str {
        "While useful at times, global variables reduce saftey since they can be accessed or mutated anywhere, and provide no guarentee that they've already been initiailized."
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "global"
    }
}

impl EarlyStmtPass for Global {
    fn visit_stmt_early(stmt: &Stmt, config: &crate::Config, reports: &mut Vec<Diagnostic<FileId>>) {
        match stmt.inner() {
            StmtType::Assignment(Assignment { left, .. }) => {
                if let ExprType::Access(Access::Global { .. }) = left.inner() {
                    reports.push(
                        Self::diagnostic(config)
                            .with_message("Use of global variable")
                            .with_labels(vec![
                                Label::primary(left.file_id(), left.span())
                                    .with_message("scope this variable to an individual object or struct"),
                            ]),
                    );
                }
            }
            StmtType::GlobalvarDeclaration(Globalvar { .. }) => {
                reports.push(
                    Self::diagnostic(config)
                        .with_message("Use of global variable")
                        .with_labels(vec![
                            Label::primary(stmt.file_id(), stmt.span())
                                .with_message("scope this variable to an individual object or struct"),
                        ]),
                );
            }
            _ => {}
        }
    }
}
