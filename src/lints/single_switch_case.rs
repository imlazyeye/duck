use crate::{Lint, LintCategory};

pub struct SingleSwitchCase;
impl Lint for SingleSwitchCase {
    fn tag() -> &'static str {
        "single_switch_case"
    }

    fn display_name() -> &'static str {
        "Single wwitch case"
    }

    fn explanation() -> &'static str {
        "Switch statements that only match on a single element can be reduced to an `if` statement."
    }

    fn suggestions() -> Vec<&'static str> {
        vec!["Use an `if` statement instead of a `switch` statement"]
    }

    fn category() -> LintCategory {
        LintCategory::Style
    }
}
