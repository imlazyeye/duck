use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyExprPass, Lint, LintLevel},
    parse::{Expr, ExprKind, Logical, LogicalOp, TokenType},
    Config, FileId,
};

#[derive(Debug, PartialEq)]
pub struct OrPreference;
impl Lint for OrPreference {
    fn explanation() -> &'static str {
        "GML supports both `or` and `||` to refer to logical \"or\" -- `||` is more consistent with other languages and is preferred."
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "or_preference"
    }
}
impl EarlyExprPass for OrPreference {
    fn visit_expr_early(expr: &Expr, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let ExprKind::Logical(Logical {
            op: LogicalOp::Or(token),
            ..
        }) = expr.inner()
        {
            if config.prefer_or_keyword() && token.token_type != TokenType::Or {
                reports.push(Self::diagnostic(config).with_message("Use of `||`").with_labels(vec![
                    Label::primary(expr.file_id(), token.span).with_message("use the `or` keyword instead of `||`"),
                ]));
            } else if token.token_type == TokenType::Or {
                reports.push(Self::diagnostic(config).with_message("Use of `or`").with_labels(vec![
                    Label::primary(expr.file_id(), token.span).with_message("use the `||` operator instead of `or`"),
                ]));
            }
        }
    }
}
