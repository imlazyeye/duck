use crate::{LintCategory, XLint};

pub struct NonScreamCase;
impl XLint for NonScreamCase {
    fn tag(&self) -> &str {
        "non_scream_case"
    }

    fn display_name(&self) -> &str {
        "Identifier should be SCREAM_CASE"
    }

    fn explanation(&self) -> &str {
        "Scream case is the ideal casing for constants to distingusih them from other values."
    }

    fn suggestions(&self) -> Vec<&str> {
        vec!["Change your casing to SCREAM_CASE"]
    }

    fn category(&self) -> LintCategory {
        LintCategory::Style
    }
}
