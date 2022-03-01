use crate::{lint::EarlyExpressionPass, parsing::Expression, utils::Span, Config, Lint, LintLevel, LintReport};

#[derive(Debug, PartialEq)]
pub struct DrawText;
impl Lint for DrawText {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
            display_name: "Use of `draw_text`".into(),
            tag: Self::tag(),
            explanation: "Projects that implement their own UI frameworks / localization may wish to be restrictive around when and where the `draw_text` functions are called.",
            suggestions: vec!["Replace this call with your API's ideal function".into()],
            default_level: Self::default_level(),
            span,
        }
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
        if let Expression::Call(caller, _, _) = expression {
            if let Expression::Identifier(identifier) = caller.expression() {
                if gm_draw_text_functions().contains(&identifier.name.as_str()) {
                    reports.push(Self::generate_report_with(
                        span,
                        format!("Use of `{}`", identifier.name),
                        [],
                    ))
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
