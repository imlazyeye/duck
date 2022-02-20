use crate::{parsing::expression::Expression, Duck, Lint, LintCategory, LintReport, Position};

pub struct DrawText;
impl Lint for DrawText {
    fn tag() -> &'static str {
        "draw_text"
    }

    fn display_name() -> &'static str {
        "Use of `draw_text_*`"
    }

    fn explanation() -> &'static str {
        "Projects that implement their own UI frameworks / localization may wish to be restrictive around when and where the `draw_text` functions are called."
    }

    fn suggestions() -> Vec<&'static str> {
        vec!["Replace this call with your API's ideal function"]
    }

    fn category() -> LintCategory {
        LintCategory::Pedantic
    }

    fn visit_expression(
        duck: &Duck,
        expression: &Expression,
        position: &Position,
        reports: &mut Vec<LintReport>,
    ) {
        if let Expression::Call(caller, _, _) = expression {
            if let Expression::Identifier(name) = caller.inner() {
                if gm_draw_text_functions().contains(&name.as_str()) {
                    reports.push(LintReport {
                        position: position.clone(),
                    })
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
