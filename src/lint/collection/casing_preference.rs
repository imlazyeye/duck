use crate::lint::{Lint, LintLevel};
use heck::{ToShoutySnakeCase, ToUpperCamelCase};

#[derive(Debug, PartialEq)]
pub struct CasingPreference;
impl Lint for CasingPreference {
    fn explanation() -> &'static str {
        "Like any programming language, GML contains many different symbols that all can be styled in different ways. Picking consistent rules for each type creates a cleaner and more consistent codebase."
    }

    fn default_level() -> LintLevel {
        LintLevel::Warn
    }

    fn tag() -> &'static str {
        "casing_preference"
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
