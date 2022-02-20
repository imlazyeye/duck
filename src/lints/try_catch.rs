use crate::{parsing::expression::Expression, Duck, Lint, LintCategory, LintReport, Position};

#[derive(Debug, PartialEq)]
pub struct TryCatch;
impl Lint for TryCatch {
    fn generate_report(position: Position) -> LintReport {
        LintReport {
			display_name: "Use of `try` / `catch`",
			tag: "try_catch",
			explanation: "GML's try/catch will collect all errors as opposed to the precise ones wanted, allowing them to accidently catch errors that should not be surpressed.",
			suggestions: vec!["Adjust the architecture to inspect for an issue prior to the crash"],
			category: LintCategory::Pedantic,
			position,
		}
    }

    // fn run(duck: &Duck) -> Vec<LintReport> {
    //     let mut reports = vec![];
    //     for keyword in duck.keywords() {
    //         if let (Token::Try, position) = keyword {
    //             reports.push(LintReport {
    //                 position: position.clone(),
    //             })
    //         }
    //     }
    //     reports
    // }
}
