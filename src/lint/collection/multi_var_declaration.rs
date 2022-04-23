use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyStmtPass, Lint, LintLevel},
    parse::{LocalVariableSeries, Stmt, StmtKind},
    FileId,
};

#[derive(Debug, PartialEq)]
pub struct MultiVarDeclaration;
impl Lint for MultiVarDeclaration {
    fn explanation() -> &'static str {
        "While GML allows you to create multiple local variables at once, it can often lead to confusing syntax that would read better with each variable seperated."
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "multi_var_declaration"
    }
}

impl EarlyStmtPass for MultiVarDeclaration {
    fn visit_stmt_early(stmt: &Stmt, config: &crate::Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let StmtKind::LocalVariableSeries(LocalVariableSeries { declarations }) = stmt.kind() {
            if declarations.len() > 1 {
                reports.push(
                    Self::diagnostic(config)
                        .with_message("Multiple local variables declared at once")
                        .with_labels(vec![
                            Label::primary(stmt.file_id(), stmt.span())
                                .with_message("seperate these into different declarations"),
                        ]),
                );
            }
        }
    }
}
