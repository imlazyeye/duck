use crate::{parsing::expression::Expression, Duck, Lint, LintCategory, LintReport, Position};

#[derive(Debug, PartialEq)]
pub struct NonScreamCase;
impl Lint for NonScreamCase {
    fn generate_report(position: Position) -> LintReport {
        LintReport {
			display_name: "Identifier should be SCREAM_CASE",
			tag: "non_scream_case",
			explanation: "Scream case is the ideal casing for constants to distingusih them from other values.",
			suggestions: vec!["Change your casing to SCREAM_CASE"],
			category: LintCategory::Style,
			position,
		}
    }

    // fn run(duck: &Duck) -> Vec<LintReport> {
    //     let mut reports = vec![];
    //     for mac in duck.macros() {
    //         let name = mac.name();
    //         let ideal_name = Duck::scream_case(name);
    //         if name != ideal_name {
    //             reports.push(LintReport {
    //                 position: mac.position().clone(),
    //             })
    //         }
    //     }
    //     reports
    // }
}
