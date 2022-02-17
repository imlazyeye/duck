use crate::{LintCategory, XLint};

pub struct ModKeyword;
impl XLint for ModKeyword {
    fn tag(&self) -> &str {
        "mod_keyword"
    }

    fn display_name(&self) -> &str {
        "Use of `mod`"
    }

    fn explanation(&self) -> &str {
        "GML supports both `mod` and `%` to perform modulo division -- `%` is more consistent with other languages and is preferred."
    }

    fn suggestions(&self) -> Vec<&str> {
        vec!["Use `%` instead of `mod`"]
    }

    fn category(&self) -> LintCategory {
        LintCategory::Style
    }
}
