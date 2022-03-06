use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyExpressionPass, Lint, LintLevel},
    parse::{Expression, ExpressionBox, Logical, LogicalOperator, TokenType},
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
impl EarlyExpressionPass for OrPreference {
    fn visit_expression_early(expression_box: &ExpressionBox, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let Expression::Logical(Logical {
            operator: LogicalOperator::Or(token),
            ..
        }) = expression_box.expression()
        {
            if config.prefer_or_keyword() && token.token_type != TokenType::Or {
                reports.push(Self::diagnostic(config).with_message("Use of `||`").with_labels(vec![
                    Label::primary(expression_box.file_id(), expression_box.span())
                        .with_message("use the `or` keyword instead of `||`"),
                ]));
            } else if token.token_type == TokenType::Or {
                reports.push(Self::diagnostic(config).with_message("Use of `or`").with_labels(vec![
                    Label::primary(expression_box.file_id(), expression_box.span())
                        .with_message("use the `||` operator instead of `or`"),
                ]));
            }
        }
    }
}
