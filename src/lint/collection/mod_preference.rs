use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyExpressionPass, Lint, LintLevel},
    parse::{Evaluation, EvaluationOperator, Expression, ExpressionBox, Token},
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
impl EarlyExpressionPass for ModPreference {
    fn visit_expression_early(expression_box: &ExpressionBox, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let Expression::Evaluation(Evaluation {
            operator: EvaluationOperator::Modulo(token),
            ..
        }) = expression_box.expression()
        {
            if config.prefer_mod_keyword() && token != &Token::Mod {
                reports.push(Self::diagnostic(config).with_message("Use of `%`").with_labels(vec![
                    Label::primary(expression_box.file_id(), expression_box.span())
                        .with_message("use the `mod` keyword instead of `%`"),
                ]));
            } else if token == &Token::Mod {
                reports.push(Self::diagnostic(config).with_message("Use of `mod`").with_labels(vec![
                    Label::primary(expression_box.file_id(), expression_box.span())
                        .with_message("use the `%` operator instead of `mod`"),
                ]));
            }
        }
    }
}
