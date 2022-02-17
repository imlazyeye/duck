use crate::{LintCategory, XLint};

pub struct Globalvar;
impl XLint for Globalvar {
    fn tag(&self) -> &str {
        "globalvar"
    }

    fn display_name(&self) -> &str {
        "Use of `globalvar`"
    }

    fn explanation(&self) -> &str {
        "Globalvars are depricated and reduce readability."
    }

    fn suggestions(&self) -> Vec<&str> {
        vec![
            "Use the `global` keyword",
            "Scope this variable to an individual object",
        ]
    }

    fn category(&self) -> LintCategory {
        LintCategory::Correctness
    }
}
