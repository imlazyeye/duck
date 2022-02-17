use crate::{Duck, Lint, LintCategory, LintReport};

pub struct AnonymousConstructor;
impl Lint for AnonymousConstructor {
    fn tag() -> &'static str {
        "anonymous_constructor"
    }

    fn display_name() -> &'static str {
        "Use of an anonymous constructor"
    }

    fn explanation() -> &'static str {
        "Constructors should be reserved for larger, higher scoped types."
    }

    fn suggestions() -> Vec<&'static str> {
        vec![
            "Change this to a named function",
            "Change this to a function that returns a struct literal",
        ]
    }

    fn category() -> crate::LintCategory {
        LintCategory::Style
    }

    fn run(duck: &Duck) -> Vec<LintReport> {
        let mut reports = vec![];
        for constructor in duck.constructors() {
            if constructor.name().is_none() {
                reports.push(LintReport {
                    position: constructor.position().clone(),
                });
            }
        }
        reports
    }
}
