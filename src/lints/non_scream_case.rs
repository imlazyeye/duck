use crate::{Duck, Lint, LintCategory, LintReport};

pub struct NonScreamCase;
impl Lint for NonScreamCase {
    fn tag() -> &'static str {
        "non_scream_case"
    }

    fn display_name() -> &'static str {
        "Identifier should be SCREAM_CASE"
    }

    fn explanation() -> &'static str {
        "Scream case is the ideal casing for constants to distingusih them from other values."
    }

    fn suggestions() -> Vec<&'static str> {
        vec!["Change your casing to SCREAM_CASE"]
    }

    fn category() -> LintCategory {
        LintCategory::Style
    }

    fn run(duck: &Duck) -> Vec<LintReport> {
        let mut reports = vec![];
        for mac in duck.macros() {
            let name = mac.name();
            let ideal_name = Duck::scream_case(name);
            if name != ideal_name {
                reports.push(LintReport {
                    position: mac.position().clone(),
                })
            }
        }
        reports
    }
}
