use crate::lint::{Lint, LintLevel};
use heck::ToShoutySnakeCase;

#[derive(Debug, PartialEq)]
pub struct NonScreamCase;
impl Lint for NonScreamCase {
    fn explanation() -> &'static str {
        ""
    }

    fn default_level() -> LintLevel {
        LintLevel::Warn
    }

    fn tag() -> &'static str {
        ""
    }
}

/// Returns the given string under duck's definition of SCREAM_CASE.
#[allow(dead_code)]
fn scream_case(string: &str) -> String {
    let output = string.to_shouty_snake_case();
    let mut prefix = String::new();
    let mut chars = string.chars();
    while let Some('_') = chars.next() {
        prefix.push('_');
    }
    prefix + &output
}
