use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyExprPass, EarlyStmtPass, Lint, LintLevel},
    parse::{Access, Call, Expr, ExprKind, Globalvar, Stmt, StmtKind},
    FileId,
};

#[derive(Debug, PartialEq)]
pub struct Deprecated;
impl Lint for Deprecated {
    fn explanation() -> &'static str {
        "Deprecated features are liable to be removed at any time and should be avoided."
    }

    fn default_level() -> LintLevel {
        LintLevel::Warn
    }

    fn tag() -> &'static str {
        "deprecated"
    }
}

impl EarlyStmtPass for Deprecated {
    fn visit_stmt_early(stmt: &Stmt, config: &crate::Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let StmtKind::Globalvar(Globalvar { name }) = stmt.kind() {
            reports.push(
                Self::diagnostic(config)
                    .with_message("Use of `globalvar`")
                    .with_labels(vec![
                        Label::primary(stmt.file_id(), stmt.span())
                            .with_message(format!("Change this to the `global.{}` syntax", name.lexeme)),
                    ]),
            );
        }
    }
}

impl EarlyExprPass for Deprecated {
    fn visit_expr_early(expr: &Expr, config: &crate::Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let ExprKind::Call(Call { left, .. }) = expr.kind() {
            if let ExprKind::Identifier(identifier) = left.kind() {
                if gm_deprecated_functions().contains(&identifier.lexeme.as_str()) {
                    reports.push(
                        Self::diagnostic(config)
                            .with_message(format!("Use of deprecated function: {}", identifier.lexeme))
                            .with_labels(vec![
                                Label::primary(left.file_id(), left.span()).with_message("this function is deprecated"),
                            ]),
                    );
                }
            }
        } else if let ExprKind::Access(Access::Array { index_two: Some(_), .. }) = expr.kind() {
            reports.push(
                Self::diagnostic(config)
                    .with_message("Use of 2d array")
                    .with_labels(vec![
                        Label::primary(expr.file_id(), expr.span())
                            .with_message("use chained arrays instead (`foo[0][0]`)"),
                    ]),
            );
        }
    }
}

fn gm_deprecated_functions() -> &'static [&'static str] {
    &[
        "array_height_2d",
        "array_length_2d",
        "array_length_2d",
        "buffer_surface_copy",
        "variable_struct_get",
        "variable_struct_set",
        "variable_struct_get_names",
        "variable_struct_names_count",
    ]
}
