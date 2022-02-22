use bimap::BiHashMap;

use crate::{parsing::expression::Expression, Lint, LintCategory, LintReport, Span};

#[derive(Debug, PartialEq)]
pub struct British;
impl Lint for British {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
			display_name: "Use of British spelling".into(),
			tag: "british",
			explanation: "GML has many duplicated function names for the sake of supporting both British and American spelling. For consistency, codebases should stick to one.",
			suggestions: vec![],
			category: LintCategory::Style,
			span,
		}
    }

    fn visit_expression(
        _duck: &crate::Duck,
        expression: &crate::parsing::expression::Expression,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Expression::Call(caller, _, _) = expression {
            if let Expression::Identifier(name) = caller.expression() {
                if let Some(american_spelling) =
                    BRITISH_TO_AMERICAN_KEYWORDS.get_by_left(name.as_str())
                {
                    reports.push(Self::generate_report_with(
                        span,
                        format!("Use of British spelling: {}", name),
                        [format!("Use `{}` instead", american_spelling)],
                    ))
                }
            }
        }
    }
}

lazy_static! {
    pub(super) static ref BRITISH_TO_AMERICAN_KEYWORDS: BiHashMap<&'static str, &'static str> = {
        let mut bimap = BiHashMap::new();
        bimap.insert("bm_dest_colour", "bm_dest_color");
        bimap.insert("bm_inv_dest_colour", "bm_inv_dest_color");
        bimap.insert("bm_src_colour", "bm_src_color");
        bimap.insert("colour_get_blue", "color_get_blue");
        bimap.insert("colour_get_green", "color_get_green");
        bimap.insert("colour_get_hue", "color_get_hue");
        bimap.insert("colour_get_red", "color_get_red");
        bimap.insert("colour_get_saturation", "color_get_saturation");
        bimap.insert("colour_get_value", "color_get_value");
        bimap.insert("draw_circle_colour", "draw_circle_color");
        bimap.insert("draw_ellipse_colour", "draw_ellipse_color");
        bimap.insert("draw_get_colour", "draw_get_color");
        bimap.insert("draw_line_colour", "draw_line_color");
        bimap.insert("draw_line_width_colour", "draw_line_width_color");
        bimap.insert("draw_point_colour", "draw_point_color");
        bimap.insert("draw_rectangle_colour", "draw_rectangle_color");
        bimap.insert("draw_roundrect_colour", "draw_roundrect_color");
        bimap.insert("draw_roundrect_colour_ext", "draw_roundrect_color_ext");
        bimap.insert("draw_set_colour", "draw_set_color");
        bimap.insert("draw_text_colour", "draw_text_color");
        bimap.insert("draw_text_ext_colour", "draw_text_ext_color");
        bimap.insert(
            "draw_text_ext_transformed_colour",
            "draw_text_ext_transformed_color",
        );
        bimap.insert(
            "draw_text_transformed_colour",
            "draw_text_transformed_color",
        );
        bimap.insert("draw_triangle_colour", "draw_triangle_color");
        bimap.insert("draw_vertex_colour", "draw_vertex_color");
        bimap.insert("draw_vertex_texture_colour", "draw_vertex_texture_color");
        bimap.insert("gamepad_set_colour", "gamepad_set_color");
        bimap.insert("gm_FogColour", "gm_Fogcolor");
        bimap.insert("gpu_get_colourwriteenable", "gpu_get_colorwriteenable");
        bimap.insert("gpu_set_colourwriteenable", "gpu_set_colorwriteenable");
        bimap.insert("make_colour_hsv", "make_color_hsv");
        bimap.insert("make_colour_rgb", "make_color_rgb");
        bimap.insert("merge_colour", "merge_color");
        bimap.insert(
            "part_particles_create_colour",
            "part_particles_create_color",
        );
        bimap.insert("part_type_colour_hsv", "part_type_color_hsv");
        bimap.insert("part_type_colour_mix", "part_type_color_mix");
        bimap.insert("part_type_colour_rgb", "part_type_color_rgb");
        bimap.insert("part_type_colour1", "part_type_color1");
        bimap.insert("part_type_colour2", "part_type_color2");
        bimap.insert("part_type_colour3", "part_type_color3");
        bimap.insert(
            "phy_particle_data_flag_colour",
            "phy_particle_data_flag_color",
        );
        bimap.insert(
            "phy_particle_flag_colourmixing",
            "phy_particle_flag_colormixing",
        );
        bimap.insert("seqtracktype_colour", "seqtracktype_color");
        bimap.insert(
            "skeleton_attachment_create_colour",
            "skeleton_attachment_create_color",
        );
        bimap.insert("skeleton_slot_colour_get", "skeleton_slot_color_get");
        bimap.insert("skeleton_slot_colour_set", "skeleton_slot_color_set");
        bimap.insert("vertex_colour", "vertex_color");
        bimap.insert("vertex_format_add_colour", "vertex_format_add_color");
        bimap.insert("vertex_type_colour", "vertex_type_color");
        bimap.insert("vertex_usage_colour", "vertex_usage_color");
        bimap
    };
}
