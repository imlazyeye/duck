use crate::{Lint, LintCategory};

pub struct Global;
impl Lint for Global {
    fn tag() -> &'static str {
        "global"
    }

    fn display_name() -> &'static str {
        "Use of `global`"
    }

    fn explanation() -> &'static str {
        "While useful at times, global variables reduce saftey since they can be accessed or mutated anywhere."
    }

    fn suggestions() -> Vec<&'static str> {
        vec!["Scope this variable to an individual object"]
    }

    fn category() -> LintCategory {
        LintCategory::Pedantic
    }
}
