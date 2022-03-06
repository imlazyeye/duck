use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyExpressionPass, Lint, LintLevel},
    parse::{Call, Expression, ExpressionBox},
    FileId,
};

#[derive(Debug, PartialEq)]
pub struct ShowDebugMessage;
impl Lint for ShowDebugMessage {
    fn explanation() -> &'static str {
        "Projects often implement their own logging framework and wish to avoid unwrapped prints to the console."
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "show_debug_message"
    }
}

impl EarlyExpressionPass for ShowDebugMessage {
    fn visit_expression_early(
        expression_box: &ExpressionBox,
        config: &crate::Config,
        reports: &mut Vec<Diagnostic<FileId>>,
    ) {
        if let Expression::Call(Call { left, .. }) = expression_box.expression() {
            if let Expression::Identifier(identifier) = left.expression() {
                if identifier.lexeme == "show_debug_message" {
                    reports.push(
                        Self::diagnostic(config)
                            .with_message("Use of `show_debug_message`")
                            .with_labels(vec![
                                Label::primary(left.file_id(), left.span())
                                    .with_message("remove this or replace this call with your API's ideal function"),
                            ]),
                    );
                }
            }
        }
    }
}
