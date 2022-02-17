use crate::{Lint, LintCategory};

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
}
