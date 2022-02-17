use crate::{Lint, LintCategory};

pub struct NonPascalCase;
impl Lint for NonPascalCase {
    fn tag() -> &'static str {
        "non_pascal_case"
    }

    fn display_name() -> &'static str {
        "Identifier should be SCREAM_CASE"
    }

    fn explanation() -> &'static str {
        "Pascal case is the ideal casing for \"types\" to distinguish them from other values."
    }

    fn suggestions() -> Vec<&'static str> {
        vec!["Change your casing to PascalCase"]
    }

    fn category() -> LintCategory {
        LintCategory::Style
    }
}
