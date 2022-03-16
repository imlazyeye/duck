use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyExprPass, Lint, LintLevel},
    parse::{Evaluation, EvaluationOp, Expr, ExprType, TokenType},
    Config, FileId,
};

#[derive(Debug, PartialEq)]
pub struct ModPreference;
impl Lint for ModPreference {
    fn explanation() -> &'static str {
        "GML supports both `mod` and `%` to perform modulo division. Consistent use of one over the other yields cleaner code."
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "mod_preference"
    }
}
impl EarlyExprPass for ModPreference {
    fn visit_expr_early(expr: &Expr, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let ExprType::Evaluation(Evaluation {
            op: EvaluationOp::Modulo(token),
            ..
        }) = expr.inner()
        {
            if config.prefer_mod_keyword() && token.token_type != TokenType::Mod {
                reports.push(Self::diagnostic(config).with_message("Use of `%`").with_labels(vec![
                    Label::primary(expr.file_id(), token.span).with_message("use the `mod` keyword instead of `%`"),
                ]));
            } else if token.token_type == TokenType::Mod {
                reports.push(Self::diagnostic(config).with_message("Use of `mod`").with_labels(vec![
                    Label::primary(expr.file_id(), token.span).with_message("use the `%` operator instead of `mod`"),
                ]));
            }
        }
    }
}
