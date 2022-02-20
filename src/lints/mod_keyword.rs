use crate::{Lint, LintCategory, LintReport, Position};

#[derive(Debug, PartialEq)]
pub struct ModKeyword;
impl Lint for ModKeyword {
    fn generate_report(position: Position) -> LintReport {
        LintReport {
			display_name: "Use of `mod`",
			tag: "mod_keyword",
			explanation: "GML supports both `mod` and `%` to perform modulo division -- `%` is more consistent with other languages and is preferred.",
			suggestions: vec!["Use `%` instead of `mod`"],
			category: LintCategory::Style,
			position,
		}
    }

    // fn run(duck: &Duck) -> Vec<LintReport> {
    //     let mut reports = vec![];
    //     for keyword in duck.keywords() {
    //         if let (Token::Mod, position) = keyword {
    //             reports.push(LintReport {
    //                 position: position.clone(),
    //             })
    //         }
    //     }
    //     reports
    // }
}
