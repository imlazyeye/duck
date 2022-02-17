use crate::{LintCategory, XLint};

pub struct DrawText;
impl XLint for DrawText {
    fn tag(&self) -> &str {
        "draw_sprite"
    }

    fn display_name(&self) -> &str {
        "Use of `draw_sprite*`"
    }

    fn explanation(&self) -> &str {
        "Projects that implement their own rendering backend may wish to be restrictive around when and where the `draw_sprite` functions are called."
    }

    fn suggestions(&self) -> Vec<&str> {
        vec!["Replace this call with your API's ideal function"]
    }

    fn category(&self) -> crate::LintCategory {
        LintCategory::Pedantic
    }
}
