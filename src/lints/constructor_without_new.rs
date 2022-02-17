use crate::{Lint, LintCategory};

pub struct ConstructorWithoutNew;
impl Lint for ConstructorWithoutNew {
    fn tag() -> &'static str {
        "constructor_without_new"
    }

    fn display_name() -> &'static str {
        "Invokation of constructor without `new`"
    }

    fn explanation() -> &'static str {
        "Constructors invoked without the `new` keyword do not return the newly constructed struct."
    }

    fn suggestions() -> Vec<&'static str> {
        vec!["Add the `new` operator before the call"]
    }

    fn category() -> LintCategory {
        LintCategory::Correctness
    }
}
