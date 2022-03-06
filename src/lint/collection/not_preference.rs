use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyExpressionPass, Lint, LintLevel},
    parse::{Expression, ExpressionBox, TokenType, Unary, UnaryOperator},
    Config, FileId,
};

#[derive(Debug, PartialEq)]
pub struct NotPreference;
impl Lint for NotPreference {
    fn explanation() -> &'static str {
        "GML supports both `not` and `!` to refer to unary \"not\". Consistent use of one over the other yields cleaner code."
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "not_preference"
    }
}
impl EarlyExpressionPass for NotPreference {
    fn visit_expression_early(expression_box: &ExpressionBox, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let Expression::Unary(Unary {
            operator: UnaryOperator::Not(token),
            ..
        }) = expression_box.expression()
        {
            if config.prefer_not_keyword() && token.token_type != TokenType::Not {
                reports.push(Self::diagnostic(config).with_message("Use of `!`").with_labels(vec![
                    Label::primary(expression_box.file_id(), token.span)
                        .with_message("use the `not` keyword instead of `!`"),
                ]));
            } else if token.token_type == TokenType::Not {
                reports.push(Self::diagnostic(config).with_message("Use of `not`").with_labels(vec![
                    Label::primary(expression_box.file_id(), token.span)
                        .with_message("use the `!` operator instead of `not`"),
                ]));
            }
        }
    }
}
