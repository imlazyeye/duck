use crate::{parsing::Token, Duck, Lint, LintCategory, LintReport};

pub struct TryCatch;
impl Lint for TryCatch {
    fn tag() -> &'static str {
        "try_catch"
    }

    fn display_name() -> &'static str {
        "Use of `try` / `catch`"
    }

    fn explanation() -> &'static str {
        "GML's try/catch will collect all errors as opposed to the precise ones wanted, allowing them to accidently catch errors that should not be surpressed."
    }

    fn suggestions() -> Vec<&'static str> {
        vec!["Adjust the architecture to inspect for an issue prior to the crash"]
    }

    fn category() -> LintCategory {
        LintCategory::Pedantic
    }

    fn run(duck: &Duck) -> Vec<LintReport> {
        let mut reports = vec![];
        for keyword in duck.keywords() {
            if let (Token::Try, position) = keyword {
                reports.push(LintReport {
                    position: position.clone(),
                })
            }
        }
        reports
    }
}
