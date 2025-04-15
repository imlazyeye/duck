use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    Config, FileId,
    lint::{EarlyExprPass, Lint, LintLevel},
    parse::{Call, Expr, ExprKind},
};

#[derive(Debug, PartialEq)]
pub struct Todo;
impl Lint for Todo {
    fn explanation() -> &'static str {
        "Todo markers are useful for work-in-progress code, but often are not intended to be permanently in place."
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "todo"
    }
}

impl EarlyExprPass for Todo {
    fn visit_expr_early(expr: &Expr, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let ExprKind::Call(Call { left, .. }) = expr.kind() {
            if let ExprKind::Identifier(identifier) = left.kind() {
                if identifier.lexeme == config.todo_keyword {
                    reports.push(
                        Self::diagnostic(config)
                            .with_message("Use of todo marker")
                            .with_labels(vec![
                                Label::primary(left.file_id(), left.span()).with_message("remove this todo marker"),
                            ]),
                    );
                }
            }
        }
    }
}
