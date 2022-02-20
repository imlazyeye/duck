use crate::{parsing::expression::Expression, Duck, Lint, LintCategory, LintReport, Position};

pub struct NoSpaceBeginingComment;
impl Lint for NoSpaceBeginingComment {
    fn tag() -> &'static str {
        "no_space_begining_comment"
    }

    fn display_name() -> &'static str {
        "No space begining comment"
    }

    fn explanation() -> &'static str {
        "Comments should begin with a space after them to increase readability and consistency."
    }

    fn suggestions() -> Vec<&'static str> {
        vec!["Add a space to the begining of the comment"]
    }

    fn category() -> LintCategory {
        LintCategory::Style
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
