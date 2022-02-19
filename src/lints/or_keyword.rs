use crate::{parsing::Token, Duck, Lint, LintCategory, LintReport};

pub struct OrKeyword;
impl Lint for OrKeyword {
    fn tag() -> &'static str {
        "or_keyword"
    }

    fn display_name() -> &'static str {
        "Use of `or`"
    }

    fn explanation() -> &'static str {
        "GML supports both `or` and `||` to refer to logical or -- `||` is more consistent with other languages and is preferred."
    }

    fn suggestions() -> Vec<&'static str> {
        vec!["Use `||` instead of `or`"]
    }

    fn category() -> LintCategory {
        LintCategory::Style
    }

    fn run(duck: &Duck) -> Vec<LintReport> {
        let mut reports = vec![];
        for keyword in duck.keywords() {
            if let (Token::Or, position) = keyword {
                reports.push(LintReport {
                    position: position.clone(),
                })
            }
        }
        reports
    }
}
