use crate::{parsing::expression::Expression, Duck, Lint, LintCategory, LintReport, Position};

pub struct DrawSprite;
impl Lint for DrawSprite {
    fn tag() -> &'static str {
        "draw_sprite"
    }

    fn display_name() -> &'static str {
        "Use of `draw_sprite*`"
    }

    fn explanation() -> &'static str {
        "Projects that implement their own rendering backend may wish to be restrictive around when and where the `draw_sprite` functions are called."
    }

    fn suggestions() -> Vec<&'static str> {
        vec!["Replace this call with your API's ideal function"]
    }

    fn category() -> crate::LintCategory {
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
                if gm_draw_sprite_functions().contains(&name.as_str()) {
                    reports.push(LintReport {
                        position: position.clone(),
                    })
                }
            }
        }
    }
}

fn gm_draw_sprite_functions() -> &'static [&'static str] {
    &[
        "draw_sprite",
        "draw_sprite_ext",
        "draw_sprite_general",
        "draw_sprite_part",
        "draw_sprite_part_ext",
        "draw_sprite_pos",
        "draw_sprite_stretched",
        "draw_sprite_stretched_ext",
        "draw_sprite_tiled",
        "draw_sprite_tiled_ext",
    ]
}
