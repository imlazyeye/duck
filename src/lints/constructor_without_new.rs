use crate::lint::{Lint, LintLevel};

#[derive(Debug, PartialEq)]
pub struct ConstructorWithoutNew;
impl Lint for ConstructorWithoutNew {
    fn explanation() -> &'static str {
        "Constructors invoked without the `new` keyword do not return the newly constructed struct."
    }

    fn default_level() -> LintLevel {
        LintLevel::Deny
    }

    fn tag() -> &'static str {
        "constructor_without_new"
    }
}
