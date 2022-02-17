use crate::{Duck, GmlSwitchStatementDefault, Lint, LintCategory, LintReport};

pub struct MissingDefaultCase;
impl Lint for MissingDefaultCase {
    fn tag() -> &'static str {
        "missing_default_case"
    }

    fn display_name() -> &'static str {
        "Missing default case"
    }

    fn explanation() -> &'static str {
        "Switch statements are often used to express all possible outcomes of a limited data set, but by not implementing a default case, no code will run to handle any alternate or unexpected values."
    }

    fn suggestions() -> Vec<&'static str> {
        vec!["Add a default case to the switch statement"]
    }

    fn category() -> LintCategory {
        LintCategory::Pedantic
    }

    fn run(duck: &Duck) -> Vec<LintReport> {
        let mut reports = vec![];
        for switch in duck.switches() {
            if switch.default_case() == &GmlSwitchStatementDefault::None {
                reports.push(LintReport {
                    position: switch.position().clone(),
                })
            }
        }
        reports
    }
}
