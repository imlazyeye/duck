use crate::{LintCategory, XLint};

pub struct SingleSwitchCase;
impl XLint for SingleSwitchCase {
    fn tag(&self) -> &str {
        "single_switch_case"
    }

    fn display_name(&self) -> &str {
        "Single wwitch case"
    }

    fn explanation(&self) -> &str {
        "Switch statements that only match on a single element can be reduced to an `if` statement."
    }

    fn suggestions(&self) -> Vec<&str> {
        vec!["Use an `if` statement instead of a `switch` statement"]
    }

    fn category(&self) -> LintCategory {
        LintCategory::Style
    }
}
