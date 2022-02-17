use crate::{XLint, LintCategory};

pub struct AndKeyword;
impl XLint for AndKeyword {
    fn tag(&self) -> &str {
        "and_keyword"
    }

    fn display_name(&self) -> &str {
        "Use of `and`"
    }

    fn explanation(&self) -> &str {
        "GML supports both `and` and `&&` to refer to logical and -- `&&` is more consistent with other languages and is preferred."
    }

    fn suggestions(&self) -> Vec<&str> {
        vec!["Use `&&` instead of `and`"]
    }

    fn category(&self) -> crate::LintCategory {
        LintCategory::Style
    }
}
