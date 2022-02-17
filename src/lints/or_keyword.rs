use crate::{LintCategory, XLint};

pub struct OrKeyword;
impl XLint for OrKeyword {
    fn tag(&self) -> &str {
        "or_keyword"
    }

    fn display_name(&self) -> &str {
        "Use of `or`"
    }

    fn explanation(&self) -> &str {
        "GML supports both `or` and `||` to refer to logical or -- `||` is more consistent with other languages and is preferred."
    }

    fn suggestions(&self) -> Vec<&str> {
        vec!["Use `||` instead of `or`"]
    }

    fn category(&self) -> LintCategory {
        LintCategory::Style
    }
}
