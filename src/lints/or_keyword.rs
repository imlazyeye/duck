use crate::{parsing::expression::Expression, Duck, Lint, LintCategory, LintReport, Position};

#[derive(Debug, PartialEq)]
pub struct OrKeyword;
impl Lint for OrKeyword {
    fn generate_report(position: Position) -> LintReport {
        LintReport {
			display_name: "Use of `or`",
			tag: "or_keyword",
			explanation: "GML supports both `or` and `||` to refer to logical or -- `||` is more consistent with other languages and is preferred.",
			suggestions: vec!["Use `||` instead of `or`"],
			category: LintCategory::Style,
			position,
		}
    }

    // fn run(duck: &Duck) -> Vec<LintReport> {
    //     let mut reports = vec![];
    //     for keyword in duck.keywords() {
    //         if let (Token::Or, position) = keyword {
    //             reports.push(LintReport {
    //                 position: position.clone(),
    //             })
    //         }
    //     }
    //     reports
    // }
}
