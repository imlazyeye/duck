use crate::{adt, array, function, solve::*, var};
use Ty::*;

impl Solver {
    pub fn define_gml_std(&mut self) -> crate::solve::adt::Adt {
        let id = adt!(self => {

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
        });

        self.get_adt(id).clone()
    }
}
