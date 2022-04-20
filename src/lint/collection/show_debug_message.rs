use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyExprPass, Lint, LintLevel},
    parse::{Call, Expr, ExprKind},
    FileId,
};

#[derive(Debug, PartialEq)]
pub struct ShowDebugMessage;
impl Lint for ShowDebugMessage {
    fn explanation() -> &'static str {
        "Projects often implement their own logging framework and wish to avoid unwrapped prints to the console."
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "show_debug_message"
    }
}

impl EarlyExprPass for ShowDebugMessage {
    fn visit_expr_early(expr: &Expr, config: &crate::Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let ExprKind::Call(Call { left, .. }) = expr.inner() {
            if let ExprKind::Identifier(identifier) = left.inner() {
                if identifier.lexeme == "show_debug_message" {
                    reports.push(
                        Self::diagnostic(config)
                            .with_message("Use of `show_debug_message`")
                            .with_labels(vec![
                                Label::primary(left.file_id(), left.span())
                                    .with_message("remove this or replace this call with your API's ideal function"),
                            ]),
                    );
                }
            }
        }
    }
}
