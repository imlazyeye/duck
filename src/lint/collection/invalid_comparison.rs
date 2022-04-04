use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyExprPass, Lint, LintLevel},
    parse::{Expr, ExprType, Logical},
    Config, FileId,
};

#[derive(Debug, PartialEq)]
pub struct InvalidComparison;
impl Lint for InvalidComparison {
    fn explanation() -> &'static str {
        "Certain types allow comparison checks in gml but are undefined behavior and have no valid use cases."
    }

    fn default_level() -> LintLevel {
        LintLevel::Deny
    }

    fn tag() -> &'static str {
        "invalid_comparison"
    }
}
impl InvalidComparison {
    fn test_expr(expr: &Expr, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        let is_valid = !matches!(expr.inner(), ExprType::Function(_));
        if !is_valid {
            reports.push(
                Self::diagnostic(config)
                    .with_message("Invalid comparison")
                    .with_labels(vec![
                        Label::primary(expr.file_id(), expr.span())
                            .with_message("cannot compare a value with a function declaration"),
                    ]),
            );
        }
    }
}

impl EarlyExprPass for InvalidComparison {
    fn visit_expr_early(expr: &Expr, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let ExprType::Logical(Logical { left, right, .. }) = expr.inner() {
            Self::test_expr(left, config, reports);
            Self::test_expr(right, config, reports);
        }
    }
}
