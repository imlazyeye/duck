use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyExpressionPass, EarlyStatementPass, Lint, LintLevel},
    parse::{Access, Call, Expression, ExpressionBox, Globalvar, Statement, StatementBox},
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

impl EarlyStatementPass for Deprecated {
    fn visit_statement_early(
        statement_box: &StatementBox,
        config: &crate::Config,
        reports: &mut Vec<Diagnostic<FileId>>,
    ) {
        if let Statement::GlobalvarDeclaration(Globalvar { name }) = statement_box.statement() {
            reports.push(
                Self::diagnostic(config)
                    .with_message("Use of `globalvar`")
                    .with_labels(vec![
                        Label::primary(statement_box.file_id(), statement_box.span())
                            .with_message(format!("Change this to the `global.{}` syntax", name)),
                    ]),
            );
        }
    }
}

impl EarlyExpressionPass for Deprecated {
    fn visit_expression_early(
        expression_box: &ExpressionBox,
        config: &crate::Config,
        reports: &mut Vec<Diagnostic<FileId>>,
    ) {
        if let Expression::Call(Call { left, .. }) = expression_box.expression() {
            if let Expression::Identifier(identifier) = left.expression() {
                if gm_deprecated_functions().contains(&identifier.name.as_str()) {
                    reports.push(
                        Self::diagnostic(config)
                            .with_message(format!("Use of deprecated function: {}", identifier.name))
                            .with_labels(vec![
                                Label::primary(left.file_id(), left.span()).with_message("this function is deprecated"),
                            ]),
                    );
                }
            }
        } else if let Expression::Access(Access::Array { index_two: Some(_), .. }) = expression_box.expression() {
            reports.push(
                Self::diagnostic(config)
                    .with_message("Use of 2d array")
                    .with_labels(vec![
                        Label::primary(expression_box.file_id(), expression_box.span())
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
    ]
}
