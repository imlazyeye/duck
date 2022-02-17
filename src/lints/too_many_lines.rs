use crate::{Lint, LintCategory};

pub struct TooManyLines;
impl Lint for TooManyLines {
    fn tag() -> &'static str {
        "too_many_lines"
    }

    fn display_name() -> &'static str {
        "Too many lines"
    }

    fn explanation() -> &'static str {
        "Functions with lots of lines are harder to work with due to the volume of code that must be read to understand them."
    }

    fn suggestions() -> Vec<&'static str> {
        vec!["Split this into multiple functions"]
    }

    fn category() -> LintCategory {
        LintCategory::Style
    }
}
