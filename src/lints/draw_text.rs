use crate::{
    lint::{EarlyExpressionPass, Lint, LintLevel, LintReport},
    parsing::{Call, Expression},
    utils::Span,
    Config,
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
    fn visit_expression_early(_config: &Config, expression: &Expression, span: Span, reports: &mut Vec<LintReport>) {
        if let Expression::Call(Call { left, .. }) = expression {
            if let Expression::Identifier(identifier) = left.expression() {
                if gm_draw_text_functions().contains(&identifier.name.as_str()) {
                    Self::report(
                        format!("Use of `{}`", identifier.name),
                        ["Replace this call with your API's ideal function".into()],
                        span,
                        reports,
                    )
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
