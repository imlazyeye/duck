use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyStmtPass, Lint, LintLevel},
    parse::{LocalVariables, Stmt, StmtKind},
    Config, FileId,
};

#[derive(Debug, PartialEq)]
pub struct VarPrefixViolation;
impl Lint for VarPrefixViolation {
    fn explanation() -> &'static str {
        "It is common practice in GML to prefix local variables (longer than one charcter) with an underscore as it helps to visually distinguish them from instance (or global) variables. You can select either option via the config."
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "var_prefix_violation"
    }
}

impl EarlyStmtPass for VarPrefixViolation {
    fn visit_stmt_early(stmt: &Stmt, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let StmtKind::LocalVariableSeries(LocalVariables { declarations }) = stmt.kind() {
            for local_variable in declarations.iter() {
                let name = local_variable.name();
                if config.var_prefixes && name.len() > 1 && !name.starts_with('_') {
                    reports.push(
                        Self::diagnostic(config)
                            .with_message("Local variable without underscore prefix")
                            .with_labels(vec![
                                Label::primary(local_variable.name_expr().file_id(), local_variable.name_expr().span())
                                    .with_message(format!("Change `{name}` to `_{name}`")),
                            ]),
                    );
                } else if !config.var_prefixes && name.starts_with('_') {
                    reports.push(
                        Self::diagnostic(config)
                            .with_message("Local variable with underscore prefix")
                            .with_labels(vec![
                                Label::primary(local_variable.name_expr().file_id(), local_variable.name_expr().span())
                                    .with_message(format!("Change `{name}` to `{}`", &name[1..])),
                            ]),
                    );
                }
            }
        }
    }
}
