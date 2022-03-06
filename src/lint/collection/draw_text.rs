use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyExpressionPass, Lint, LintLevel},
    parse::{Call, Expression, ExpressionBox},
    Config, FileId,
};

#[derive(Debug, PartialEq)]
pub struct DrawText;
impl Lint for DrawText {
    fn explanation() -> &'static str {
        "Projects that implement their own UI frameworks / localization may wish to be restrictive around when and where the `draw_text` functions are called."
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "draw_text"
    }
}

impl EarlyExpressionPass for DrawText {
    fn visit_expression_early(expression_box: &ExpressionBox, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let Expression::Call(Call { left, .. }) = expression_box.expression() {
            if let Expression::Identifier(identifier) = left.expression() {
                if gm_draw_text_functions().contains(&identifier.lexeme.as_str()) {
                    reports.push(
                        Self::diagnostic(config)
                            .with_message(format!("Use of `{}`", identifier.lexeme))
                            .with_labels(vec![
                                Label::primary(left.file_id(), left.span())
                                    .with_message("replace this call with your API's ideal function"),
                            ]),
                    );
                }
            }
        }
    }
}

fn gm_draw_text_functions() -> &'static [&'static str] {
    &[
        "draw_text",
        "draw_text_color",
        "draw_text_colour",
        "draw_text_ext",
        "draw_text_ext_color",
        "draw_text_ext_colour",
        "draw_text_ext_transformed",
        "draw_text_ext_transformed_color",
        "draw_text_ext_transformed_colour",
        "draw_text_transformed",
        "draw_text_transformed_color",
        "draw_text_transformed_colour",
    ]
}
