use crate::{Lint, LintCategory, LintReport, Span};

#[derive(Debug, PartialEq)]
pub struct ModKeyword;
impl Lint for ModKeyword {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
			display_name: "Use of `mod`",
			tag: "mod_keyword",
			explanation: "GML supports both `mod` and `%` to perform modulo division -- `%` is more consistent with other languages and is preferred.",
			suggestions: vec!["Use `%` instead of `mod`"],
			category: LintCategory::Style,
			span,
		}
    }

    // fn run(duck: &Duck) -> Vec<LintReport> {
    //     let mut reports = vec![];
    //     for keyword in duck.keywords() {
    //         if let (Token::Mod, span) = keyword {
    //             reports.push(LintReport {
    //                 span: span,
    //             })
    //         }
    //     }
    //     reports
    // }
}
