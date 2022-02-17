use crate::{LintCategory, XLint};

pub struct Todo;
impl XLint for Todo {
    fn tag(&self) -> &str {
        "todo"
    }

    fn display_name(&self) -> &str {
        "Use of todo marker"
    }

    fn explanation(&self) -> &str {
        "Todo markers are useful for work-in-progress code, but often are not intended to be permanently in place."
    }

    fn suggestions(&self) -> Vec<&str> {
        vec!["Remove this todo marker"]
    }

    fn category(&self) -> LintCategory {
        LintCategory::Pedantic
    }
}
