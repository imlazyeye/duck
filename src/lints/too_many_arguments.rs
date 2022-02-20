use crate::{
    parsing::expression::{Expression},
    Duck, Lint, LintCategory, LintReport, Position,
};

pub struct TooManyArguments;
impl Lint for TooManyArguments {
    fn tag() -> &'static str {
        "too_many_arguments"
    }

    fn display_name() -> &'static str {
        "Too many arguments"
    }

    fn explanation() -> &'static str {
        "Functions with lots of parameters quickly become confusing and indicate a need for structural change."
    }

    fn suggestions() -> Vec<&'static str> {
        vec![
            "Split this into multiple functions",
            "Create a struct that holds the fields required by this function",
        ]
    }

    fn category() -> LintCategory {
        LintCategory::Style
    }
}
