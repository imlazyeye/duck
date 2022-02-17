use crate::{LintCategory, XLint};

pub struct DrawText;
impl XLint for DrawText {
    fn tag(&self) -> &str {
        "draw_text"
    }

    fn display_name(&self) -> &str {
        "Use of `draw_text_*`"
    }

    fn explanation(&self) -> &str {
        "Projects that implement their own UI frameworks / localization may wish to be restrictive around when and where the `draw_text` functions are called."
    }

    fn suggestions(&self) -> Vec<&str> {
        vec!["Replace this call with your API's ideal function"]
    }

    fn category(&self) -> LintCategory {
        LintCategory::Pedantic
    }
}
