use bimap::BiHashMap;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use once_cell::sync::Lazy;

use crate::{
    EnglishFlavor, FileId,
    lint::{EarlyExprPass, Lint, LintLevel},
    parse::{Call, Expr, ExprKind},
};

#[derive(Debug, PartialEq)]
pub struct EnglishFlavorViolation;
impl Lint for EnglishFlavorViolation {
    fn explanation() -> &'static str {
        "GML has many duplicated function names for the sake of supporting both British and American spelling. For consistency, codebases should stick to one."
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "english_flavor_violation"
    }
}

impl EarlyExprPass for EnglishFlavorViolation {
    fn visit_expr_early(expr: &Expr, config: &crate::Config, reports: &mut Vec<Diagnostic<FileId>>) {
        let english_flavor = &config.english_flavor;
        if let ExprKind::Call(Call { left, .. }) = expr.kind() {
            if let ExprKind::Identifier(identifier) = left.kind() {
                match english_flavor {
                    EnglishFlavor::American => {
                        if let Some(american_spelling) =
                            BRITISH_TO_AMERICAN_KEYWORDS.get_by_left(identifier.lexeme.as_str())
                        {
                            reports.push(
                                Self::diagnostic(config)
                                    .with_message(format!("Use of British spelling `{}`", identifier.lexeme))
                                    .with_labels(vec![
                                        Label::primary(left.file_id(), left.span())
                                            .with_message(format!("replace this with `{}`", american_spelling)),
                                    ]),
                            );
                        }
                    }
                    EnglishFlavor::British => {
                        if let Some(british_spelling) =
                            BRITISH_TO_AMERICAN_KEYWORDS.get_by_right(identifier.lexeme.as_str())
                        {
                            reports.push(
                                Self::diagnostic(config)
                                    .with_message(format!("Use of American spelling `{}`", identifier.lexeme))
                                    .with_labels(vec![
                                        Label::primary(left.file_id(), left.span())
                                            .with_message(format!("replace this with `{}`", british_spelling)),
                                    ]),
                            );
                        }
                    }
                }
            }
        }
    }
}

pub(super) static BRITISH_TO_AMERICAN_KEYWORDS: Lazy<BiHashMap<&'static str, &'static str>> = Lazy::new(|| {
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
    bimap.insert("draw_text_ext_transformed_colour", "draw_text_ext_transformed_color");
    bimap.insert("draw_text_transformed_colour", "draw_text_transformed_color");
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
    bimap.insert("part_particles_create_colour", "part_particles_create_color");
    bimap.insert("part_type_colour_hsv", "part_type_color_hsv");
    bimap.insert("part_type_colour_mix", "part_type_color_mix");
    bimap.insert("part_type_colour_rgb", "part_type_color_rgb");
    bimap.insert("part_type_colour1", "part_type_color1");
    bimap.insert("part_type_colour2", "part_type_color2");
    bimap.insert("part_type_colour3", "part_type_color3");
    bimap.insert("phy_particle_data_flag_colour", "phy_particle_data_flag_color");
    bimap.insert("phy_particle_flag_colourmixing", "phy_particle_flag_colormixing");
    bimap.insert("seqtracktype_colour", "seqtracktype_color");
    bimap.insert("skeleton_attachment_create_colour", "skeleton_attachment_create_color");
    bimap.insert("skeleton_slot_colour_get", "skeleton_slot_color_get");
    bimap.insert("skeleton_slot_colour_set", "skeleton_slot_color_set");
    bimap.insert("vertex_colour", "vertex_color");
    bimap.insert("vertex_format_add_colour", "vertex_format_add_color");
    bimap.insert("vertex_type_colour", "vertex_type_color");
    bimap.insert("vertex_usage_colour", "vertex_usage_color");
    bimap
});
