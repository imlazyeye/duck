use crate::{LintCategory, XLint};

pub struct ConstructorWithoutNew;
impl XLint for ConstructorWithoutNew {
    fn tag(&self) -> &str {
        "constructor_without_new"
    }

    fn display_name(&self) -> &str {
        "Invokation of constructor without `new`"
    }

    fn explanation(&self) -> &str {
        "Constructors invoked without the `new` keyword will always return undefined."
    }

    fn suggestions(&self) -> Vec<&str> {
        vec!["Add the `new` operator before the call"]
    }

    fn category(&self) -> LintCategory {
        LintCategory::Correctness
    }
}
