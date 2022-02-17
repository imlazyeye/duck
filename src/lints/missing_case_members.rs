use crate::{Lint, LintCategory};

pub struct MissingCaseMember;
impl Lint for MissingCaseMember {
    fn tag() -> &'static str {
        "missing_case_member"
    }

    fn display_name() -> &'static str {
        "Missing case member"
    }

    fn explanation() -> &'static str {
        "Switch statements matching over an enum typically want to cover all possible cases if they do not implement a default case."
    }

    fn suggestions() -> Vec<&'static str> {
        vec![
            "Add cases for the missing members",
            "Remove the imtentional crash from your default case",
        ]
    }

    fn category() -> LintCategory {
        LintCategory::Correctness
    }
}
