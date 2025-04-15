use codespan_reporting::diagnostic::{Diagnostic, Label};
use colored::Colorize;

use crate::{
    Config, FileId,
    lint::{EarlyStmtPass, Lint, LintLevel},
    parse::{ExprKind, Function, Stmt, StmtKind},
};

#[derive(Debug, PartialEq)]
pub struct UselessFunction;
impl Lint for UselessFunction {
    fn explanation() -> &'static str {
        "Anonymous functions that are not assigned to a variable can never be referenced."
    }

    fn default_level() -> LintLevel {
        LintLevel::Deny
    }

    fn tag() -> &'static str {
        "useless_function"
    }
}

impl EarlyStmtPass for UselessFunction {
    fn visit_stmt_early(stmt: &Stmt, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let StmtKind::Expr(expr) = stmt.kind() {
            if let ExprKind::Function(Function { name: None, .. }) = expr.kind() {
                reports.push(
                    Self::diagnostic(config)
                        .with_message("Useless function")
                        .with_labels(vec![
                            Label::primary(expr.file_id(), expr.span())
                                .with_message("this function can never be referenced"),
                        ])
                        .with_notes(vec![format!(
                            "{}: turn this into a named function or save it into a variable",
                            "help".bold()
                        )]),
                );
            }
        }
    }
}
