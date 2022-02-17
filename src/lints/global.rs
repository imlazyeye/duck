use crate::{LintCategory, XLint};

pub struct Global;
impl XLint for Global {
    fn tag(&self) -> &str {
        "global"
    }

    fn display_name(&self) -> &str {
        "Use of `global`"
    }

    fn explanation(&self) -> &str {
        "While useful at times, global variables reduce saftey since they can be accessed or mutated anywhere."
    }

    fn suggestions(&self) -> Vec<&str> {
        vec!["Scope this variable to an individual object"]
    }

    fn category(&self) -> LintCategory {
        LintCategory::Pedantic
    }
}
