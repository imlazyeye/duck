use crate::{lint::EarlyExpressionPass, parsing::Expression, utils::Span, Lint, LintLevel, LintReport};

#[derive(Debug, PartialEq)]
pub struct DrawSprite;
impl Lint for DrawSprite {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
            display_name: "Use of `draw_sprite`".into(),
            tag: Self::tag(),
            explanation: "Projects that implement their own rendering backend may wish to be restrictive around when and where the `draw_sprite` functions are called.",
            suggestions: vec!["Replace this call with your API's ideal function".into()],
            default_level: Self::default_level(),
            span,
        }
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "draw_sprite"
    }
}

impl EarlyExpressionPass for DrawSprite {
    fn visit_expression_early(
        _config: &crate::Config,
        expression: &Expression,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Expression::Call(caller, _, _) = expression {
            if let Expression::Identifier(identifier) = caller.expression() {
                if gm_draw_sprite_functions().contains(&identifier.name.as_str()) {
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
