use crate::{parsing::expression::Expression, Duck, Lint, LintCategory, LintReport, Position};

#[derive(Debug, PartialEq)]
pub struct NonPascalCase;
impl Lint for NonPascalCase {
    fn generate_report(position: Position) -> LintReport {
        LintReport {
			display_name: "Identifier should be SCREAM_CASE",
			tag: "non_pascal_case",
			explanation: "Pascal case is the ideal casing for \"types\" to distinguish them from other values.",
			suggestions: vec!["Change your casing to PascalCase"],
			category: LintCategory::Style,
			position,
		}
    }

    // fn run(duck: &Duck) -> Vec<LintReport> {
    //     let mut reports = vec![];
    //     for e in duck.enums() {
    //         let name = e.name();
    //         let ideal_name = Duck::pascal_case(name);
    //         if name != ideal_name {
    //             reports.push(LintReport {
    //                 position: e.position().clone(),
    //             })
    //         }
    //     }
    //     for constructor in duck.constructors() {
    //         if let Some(name) = constructor.name() {
    //             let ideal_name = Duck::pascal_case(name);
    //             if name != &ideal_name {
    //                 reports.push(LintReport {
    //                     position: constructor.position().clone(),
    //                 })
    //             }
    //         }
    //     }
    //     reports
    // }
}
