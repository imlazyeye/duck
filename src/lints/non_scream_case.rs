use crate::{Lint, LintCategory};

pub struct NonScreamCase;
impl Lint for NonScreamCase {
    fn tag() -> &'static str {
        "non_scream_case"
    }

    fn display_name() -> &'static str {
        "Identifier should be SCREAM_CASE"
    }

    fn explanation() -> &'static str {
        "Scream case is the ideal casing for constants to distingusih them from other values."
    }

    fn suggestions() -> Vec<&'static str> {
        vec!["Change your casing to SCREAM_CASE"]
    }

    fn category() -> LintCategory {
        LintCategory::Style
    }
}
