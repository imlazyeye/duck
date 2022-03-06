use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyExpressionPass, Lint, LintLevel},
    parse::{Equality, Expression, ExpressionBox},
    Config, FileId,
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
    fn test_expr(expression_box: &ExpressionBox, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        let is_valid = !matches!(expression_box.expression(), Expression::FunctionDeclaration(_));
        if !is_valid {
            reports.push(
                Self::diagnostic(config)
                    .with_message("Invalid equality")
                    .with_labels(vec![
                        Label::primary(expression_box.file_id(), expression_box.span())
                            .with_message("cannot check for equality with a function declaration"),
                    ]),
            );
        }
    }
}

impl EarlyExpressionPass for InvalidEquality {
    fn visit_expression_early(expression_box: &ExpressionBox, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let Expression::Equality(Equality { left, right, .. }) = expression_box.expression() {
            Self::test_expr(left, config, reports);
            Self::test_expr(right, config, reports);
        }
    }
}
