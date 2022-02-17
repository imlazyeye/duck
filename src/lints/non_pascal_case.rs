use crate::{LintCategory, XLint};

pub struct NonPascalCase;
impl XLint for NonPascalCase {
    fn tag(&self) -> &str {
        "non_pascal_case"
    }

    fn display_name(&self) -> &str {
        "Identifier should be SCREAM_CASE"
    }

    fn explanation(&self) -> &str {
        "Pascal case is the ideal casing for \"types\" to distinguish them from other values."
    }

    fn suggestions(&self) -> Vec<&str> {
        vec!["Change your casing to PascalCase"]
    }

    fn category(&self) -> LintCategory {
        LintCategory::Style
    }
}
