use crate::{parsing::expression::Expression, Duck, Lint, LintCategory, LintReport, Position};

#[derive(Debug, PartialEq)]
pub struct NoSpaceBeginingComment;
impl Lint for NoSpaceBeginingComment {
    fn generate_report(position: Position) -> LintReport {
        LintReport {
			display_name: "No space begining comment",
			tag: "no_space_begining_comment",
			explanation: "Comments should begin with a space after them to increase readability and consistency.",
			suggestions: vec!["Add a space to the begining of the comment"],
			category: LintCategory::Style,
			position,
		}
    }

    // fn run(duck: &Duck) -> Vec<LintReport> {
    //     let mut reports = vec![];
    //     for comment in duck.comments() {
    //         // Seek out that space
    //         for c in comment.body().chars() {
    //             match c {
    //                 '/' | '*' => {}
    //                 ' ' => {
    //                     break;
    //                 }
    //                 _ => reports.push(LintReport {
    //                     position: comment.position().clone(),
    //                 }),
    //             }
    //         }
    //     }
    //     reports
    // }
}
