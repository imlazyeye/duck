use crate::{LintCategory, XLint};

pub struct TooManyArguments;
impl XLint for TooManyArguments {
    fn tag(&self) -> &str {
        "too_many_arguments"
    }

    fn display_name(&self) -> &str {
        "Too many arguments"
    }

    fn explanation(&self) -> &str {
        "Functions with lots of parameters quickly become confusing and indicate a need for structural change."
    }

    fn suggestions(&self) -> Vec<&str> {
        vec![
            "Split this into multiple functions",
            "Create a struct that holds the fields required by this function",
        ]
    }

    fn category(&self) -> LintCategory {
        LintCategory::Style
    }
}
