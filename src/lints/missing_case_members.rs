use crate::{XLint, LintCategory};

pub struct MissingCaseMember;
impl XLint for MissingCaseMember {
    fn tag(&self) -> &str {
        "missing_case_member"
    }

    fn display_name(&self) -> &str {
        "Missing case member"
    }

    fn explanation(&self) -> &str {
        "Switch statements matching over an enum typically want to cover all possible cases if they do not implement a default case."
    }

    fn suggestions(&self) -> Vec<&str> {
        vec![
            "Add cases for the missing members",
            "Remove the imtentional crash from your default case",
        ]
    }

    fn category(&self) -> LintCategory {
        LintCategory::Correctness
    }
}
