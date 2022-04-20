use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyExprPass, Lint, LintLevel},
    parse::{Expr, ExprKind, Logical, LogicalOp, TokenType},
    Config, FileId,
};

#[derive(Debug, PartialEq)]
pub struct AndPreference;
impl Lint for AndPreference {
    fn explanation() -> &'static str {
        "GML supports both `and` and `&&` to refer to logical \"and\". Consistent use of one over the other yields cleaner code."
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "and_preference"
    }
}
impl EarlyExprPass for AndPreference {
    fn visit_expr_early(expr: &Expr, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let ExprKind::Logical(Logical {
            op: LogicalOp::And(token),
            ..
        }) = expr.inner()
        {
            if config.prefer_and_keyword() && token.token_type != TokenType::And {
                reports.push(Self::diagnostic(config).with_message("Use of `&&`").with_labels(vec![
                    Label::primary(expr.file_id(), token.span).with_message("use the `and` keyword instead of `&&`"),
                ]));
            } else if token.token_type == TokenType::And {
                reports.push(Self::diagnostic(config).with_message("Use of `and`").with_labels(vec![
                    Label::primary(expr.file_id(), token.span).with_message("use the `&&` opreator instead of `and`"),
                ]));
            }
        }
    }
}
