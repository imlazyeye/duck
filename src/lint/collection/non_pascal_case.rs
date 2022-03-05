use crate::lint::{Lint, LintLevel};
use heck::ToUpperCamelCase;

#[derive(Debug, PartialEq)]
pub struct NonPascalCase;
impl Lint for NonPascalCase {
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

/// Returns the given string under duck's definition of PascalCase.
#[allow(dead_code)]
fn pascal_case(string: &str) -> String {
    let output = string.to_upper_camel_case();
    let mut prefix = String::new();
    let mut chars = string.chars();
    while let Some('_') = chars.next() {
        prefix.push('_');
    }
    prefix + &output
}
