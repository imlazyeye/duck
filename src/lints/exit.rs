use crate::{LintCategory, XLint};

pub struct Exit;
impl XLint for Exit {
    fn tag(&self) -> &str {
        "exit"
    }

    fn display_name(&self) -> &str {
        "Use of `exit`"
    }

    fn explanation(&self) -> &str {
        "`return` can always be used in place of exit, which provides more consistency across your codebase."
    }

    fn suggestions(&self) -> Vec<&str> {
        vec!["Use `return` instead of `exit`"]
    }

    fn category(&self) -> LintCategory {
        LintCategory::Style
    }
}
