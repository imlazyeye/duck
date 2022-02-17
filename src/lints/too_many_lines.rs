use crate::{LintCategory, XLint};

pub struct TooManyLines;
impl XLint for TooManyLines {
    fn tag(&self) -> &str {
        "too_many_lines"
    }

    fn display_name(&self) -> &str {
        "Too many lines"
    }

    fn explanation(&self) -> &str {
        "Functions with lots of lines are harder to work with due to the volume of code that must be read to understand them."
    }

    fn suggestions(&self) -> Vec<&str> {
        vec!["Split this into multiple functions"]
    }

    fn category(&self) -> LintCategory {
        LintCategory::Style
    }
}
