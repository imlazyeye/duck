use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    Config, FileId,
    lint::{EarlyExprPass, Lint, LintLevel},
    parse::{Equality, Expr, ExprKind},
};

#[derive(Debug, PartialEq)]
pub struct InvalidEquality;
impl Lint for InvalidEquality {
    fn explanation() -> &'static str {
        "Certain types allow equality checks in gml but are undefined behavior and have no valid use cases."
    }

    fn default_level() -> LintLevel {
        LintLevel::Deny
    }

    fn tag() -> &'static str {
        "invalid_equality"
    }
}

impl InvalidEquality {
    fn test_expr(expr: &Expr, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        let is_valid = !matches!(expr.kind(), ExprKind::Function(_));
        if !is_valid {
            reports.push(
                Self::diagnostic(config)
                    .with_message("Invalid equality")
                    .with_labels(vec![
                        Label::primary(expr.file_id(), expr.span())
                            .with_message("cannot check for equality with a function declaration"),
                    ]),
            );
        }
    }
}

impl EarlyExprPass for InvalidEquality {
    fn visit_expr_early(expr: &Expr, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let ExprKind::Equality(Equality { left, right, .. }) = expr.kind() {
            Self::test_expr(left, config, reports);
            Self::test_expr(right, config, reports);
        }
    }
}
