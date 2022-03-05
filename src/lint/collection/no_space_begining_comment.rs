use crate::lint::{Lint, LintLevel};

#[derive(Debug, PartialEq)]
pub struct NoSpaceBeginingComment;
impl Lint for NoSpaceBeginingComment {
    fn explanation() -> &'static str {
        "Comments should begin with a space after them to increase readability and consistency."
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "no_space_begining_comment"
    }
}
