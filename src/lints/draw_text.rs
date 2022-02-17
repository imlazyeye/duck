use crate::{Lint, LintCategory};

pub struct DrawText;
impl Lint for DrawText {
    fn tag() -> &'static str {
        "draw_text"
    }

    fn display_name() -> &'static str {
        "Use of `draw_text_*`"
    }

    fn explanation() -> &'static str {
        "Projects that implement their own UI frameworks / localization may wish to be restrictive around when and where the `draw_text` functions are called."
    }

    fn suggestions() -> Vec<&'static str> {
        vec!["Replace this call with your API's ideal function"]
    }

    fn category() -> LintCategory {
        LintCategory::Pedantic
    }
}
