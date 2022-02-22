use crate::{Lint, LintCategory, LintReport, Span};

#[derive(Debug, PartialEq)]
pub struct NoSpaceBeginingComment;
impl Lint for NoSpaceBeginingComment {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
			display_name: "No space begining comment".into(),
            tag: Self::tag(),
			explanation: "Comments should begin with a space after them to increase readability and consistency.",
			suggestions: vec!["Add a space to the begining of the comment".into()],
			category: Self::category(),
			span,
		}
    }

    fn category() -> LintCategory {
        LintCategory::Style
    }

    fn tag() -> &'static str {
        "no_space_begining_comment"
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
    //                     span: comment.span().clone(),
    //                 }),
    //             }
    //         }
    //     }
    //     reports
    // }
}
