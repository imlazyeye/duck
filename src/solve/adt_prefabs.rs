use crate::{adt, array, function, solve::*, var};
use Ty::*;

pub fn enter_new_object_scope() -> Ty {
    use Ty::*;
    adt!(
        id: Real,
        visible: Bool,
        solid: Bool,
        persistent: Bool,
        depth: Real,
        layer: Real,
        alarm: array!(Real),
        direction: Real,
        friction: Real,
        gravity: Real,
        gravity_direction: Real,
        hspeed: Real,
        vspeed: Real,
        speed: Real,
        xstart: Real,
        ystart: Real,
        x: Real,
        y: Real,
        xprevious: Real,
        yprevious: Real,
        object_index: Real,
        sprite_index: Real,
        sprite_width: Real,
        sprite_height: Real,
        sprite_xoffset: Real,
        sprite_yoffset: Real,
        image_alpha: Real,
        image_angle: Real,
        image_blend: Real,
        image_index: Real,
        image_number: Real,
        image_speed: Real,
        image_xscale: Real,
        image_yscale: Real,
        mask_index: Real,
        bbox_bottom: Real,
        bbox_left: Real,
        bbox_right: Real,
        bbox_top: Real,
        path_index: Real,
        path_position: Real,
        path_positionprevious: Real,
        path_speed: Real,
        path_scale: Real,
        path_orientation: Real,
        path_endaction: Real, // todo: its a collection of constants
        timeline_index: Real,
        timeline_running: Bool,
        timeline_speed: Real,
        timeline_position: Real,
        timeline_loop: Bool,
        in_sequence: Bool,
        sequence_instance: Any, /* it's some struct and look I just don't care
                                 * todo: we don't support the physics system */
    )
}

pub fn global_adt() -> Ty {
    adt!(
        // Arrays (missing array_pop and array_sort, as they require unions)
        array_copy: {
            let ty = array!(var!());
            function!((ty.clone(), Real, ty, Real, Real) => Undefined)
        },
        array_delete: function!((array!(var!()), Real, Real) => Undefined),
        array_create: {
            let ty = var!();
            function!((Real, ty.clone()) => array!(ty))
        },
        array_equals: {
            let ty = array!(var!());
            function!((ty.clone(), ty) => Bool)
        },
        array_get: {
            let ty = var!();
            function!((array!(ty.clone()), Real) => ty)
        },
        array_height_2d: {
            function!((array!(var!())) => Real)
        },
        array_insert: {
            let ty = var!();
            function!((array!(ty.clone()), ty, Real) => Undefined)
        },
        array_length: function!((array!(var!())) => Real),
        array_length_1d: function!((array!(var!())) => Real),
        array_length_2d: function!((array!(var!()), Real) => Real),
        array_push: {
            let ty = var!();
            function!((array!(ty.clone()), ty) => Undefined)
        },
        array_resize: function!((array!(var!()), Real) => Undefined),
        array_set: {
            let ty = var!();
            function!((array!(ty.clone()), ty, Real) => Undefined)
        },

        // Maths
        floor: function!((Real) => Real),
        sqrt: function!((Real) => Real)
    )
}
