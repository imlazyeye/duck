use crate::{parsing::expression::Expression, Duck, Lint, LintCategory, LintReport, Position};

#[derive(Debug, PartialEq)]
pub struct DrawText;
impl Lint for DrawText {
    fn generate_report(position: Position) -> LintReport {
        LintReport {
			display_name: "Use of `draw_text_*`",
			tag: "draw_text",
			explanation: "Projects that implement their own UI frameworks / localization may wish to be restrictive around when and where the `draw_text` functions are called.",
			suggestions: vec!["Replace this call with your API's ideal function"],
			category: LintCategory::Pedantic,
			position,
		}
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
                    reports.push(Self::generate_report(position.clone()))
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
