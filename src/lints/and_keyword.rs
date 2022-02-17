use crate::{parsing::Token, Duck, Lint, LintCategory, LintReport};

pub struct AndKeyword;
impl Lint for AndKeyword {
    fn tag() -> &'static str {
        "and_keyword"
    }

    fn display_name() -> &'static str {
        "Use of `and`"
    }

    fn explanation() -> &'static str {
        "GML supports both `and` and `&&` to refer to logical and -- `&&` is more consistent with other languages and is preferred."
    }

    fn suggestions() -> Vec<&'static str> {
        vec!["Use `&&` instead of `and`"]
    }

    fn category() -> crate::LintCategory {
        LintCategory::Style
    }

    fn run(duck: &Duck) -> Vec<LintReport> {
        let mut reports = vec![];
        for keyword in duck.keywords() {
            if let (Token::AndKeyword, position) = keyword {
                reports.push(LintReport {
                    position: position.clone(),
                })
            }
        }
        reports
    }
}
