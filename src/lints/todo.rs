use crate::{
    parsing::expression::{Expression},
    Duck, Lint, LintCategory, LintReport, Position,
};

pub struct Todo;
impl Lint for Todo {
    fn tag() -> &'static str {
        "todo"
    }

    fn display_name() -> &'static str {
        "Use of todo marker"
    }

    fn explanation() -> &'static str {
        "Todo markers are useful for work-in-progress code, but often are not intended to be permanently in place."
    }

    fn suggestions() -> Vec<&'static str> {
        vec!["Remove this todo marker"]
    }

    fn category() -> LintCategory {
        LintCategory::Pedantic
    }
}
