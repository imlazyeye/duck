use crate::{parsing::Token, Duck, Lint, LintCategory, LintReport};

pub struct Exit;
impl Lint for Exit {
    fn tag() -> &'static str {
        "exit"
    }

    fn display_name() -> &'static str {
        "Use of `exit`"
    }

    fn explanation() -> &'static str {
        "`return` can always be used in place of exit, which provides more consistency across your codebase."
    }

    fn suggestions() -> Vec<&'static str> {
        vec!["Use `return` instead of `exit`"]
    }

    fn category() -> LintCategory {
        LintCategory::Style
    }

    fn run(duck: &Duck) -> Vec<LintReport> {
        let mut reports = vec![];
        for keyword in duck.keywords() {
            if let (Token::Exit, position) = keyword {
                reports.push(LintReport {
                    position: position.clone(),
                })
            }
        }
        reports
    }
}
