use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyExpressionPass, Lint, LintLevel},
    parse::{Expression, ExpressionBox, Logical},
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
    fn test_expr(expression_box: &ExpressionBox, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        let is_valid = !matches!(expression_box.expression(), Expression::FunctionDeclaration(_));
        if !is_valid {
            reports.push(
                Self::diagnostic(config)
                    .with_message("Invalid comparison")
                    .with_labels(vec![
                        Label::primary(expression_box.file_id(), expression_box.span())
                            .with_message("cannot compare a value with a function declaration"),
                    ]),
            );
        }
    }
}

impl EarlyExpressionPass for InvalidComparison {
    fn visit_expression_early(expression_box: &ExpressionBox, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let Expression::Logical(Logical { left, right, .. }) = expression_box.expression() {
            Self::test_expr(left, config, reports);
            Self::test_expr(right, config, reports);
        }
    }
}
