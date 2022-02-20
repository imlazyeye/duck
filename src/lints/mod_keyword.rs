use crate::{parsing::expression::Expression, Duck, Lint, LintCategory, LintReport, Position};

pub struct ModKeyword;
impl Lint for ModKeyword {
    fn tag() -> &'static str {
        "mod_keyword"
    }

    fn display_name() -> &'static str {
        "Use of `mod`"
    }

    fn explanation() -> &'static str {
        "GML supports both `mod` and `%` to perform modulo division -- `%` is more consistent with other languages and is preferred."
    }

    fn suggestions() -> Vec<&'static str> {
        vec!["Use `%` instead of `mod`"]
    }

    fn category() -> LintCategory {
        LintCategory::Style
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
