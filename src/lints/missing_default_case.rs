use crate::{XLint, LintCategory};

pub struct MissingDefaultCase;
impl XLint for MissingDefaultCase {
    fn tag(&self) -> &str {
        "missing_default_case"
    }

    fn display_name(&self) -> &str {
        "Missing default case"
    }

    fn explanation(&self) -> &str {
        "Switch statements are often used to express all possible outcomes of a limited data set, but by not implementing a default case, no code will run to handle any alternate or unexpected values."
    }

    fn suggestions(&self) -> Vec<&str> {
        vec!["Add a default case to the switch statement"]
    }

    fn category(&self) -> LintCategory {
        LintCategory::Pedantic
    }
}
