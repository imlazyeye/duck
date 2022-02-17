use crate::{Lint, LintCategory};

pub struct Globalvar;
impl Lint for Globalvar {
    fn tag() -> &'static str {
        "globalvar"
    }

    fn display_name() -> &'static str {
        "Use of `globalvar`"
    }

    fn explanation() -> &'static str {
        "Globalvars are depricated and reduce readability."
    }

    fn suggestions() -> Vec<&'static str> {
        vec![
            "Use the `global` keyword",
            "Scope this variable to an individual object",
        ]
    }

    fn category() -> LintCategory {
        LintCategory::Correctness
    }
}
