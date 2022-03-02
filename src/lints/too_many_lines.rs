use crate::lint::{Lint, LintLevel};

#[derive(Debug, PartialEq)]
pub struct TooManyLines;
impl Lint for TooManyLines {
    fn explanation() -> &'static str {
        "Functions with lots of lines are harder to work with due to the volume of code that must be read to understand them."
    }

    fn default_level() -> LintLevel {
        LintLevel::Warn
    }

    fn tag() -> &'static str {
        "too_many_lines"
    }
}
