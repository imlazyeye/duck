use crate::{parsing::Token, Duck, Lint, LintCategory, LintReport};

pub struct WithLoop;
impl Lint for WithLoop {
    fn tag() -> &'static str {
        "with_loop"
    }

    fn display_name() -> &'static str {
        "Use of `with`"
    }

    fn explanation() -> &'static str {
        "The `with` loop allows your code's context to suddenly change, both making it more difficult to read (as a given line of code is no longer promised to be executing in the scope expected from the file), but also making it more difficult to track down all of the places an object is modified."
    }

    fn suggestions() -> Vec<&'static str> {
        vec![
            "Use `instance_find` if looping over objects",
            "Use direct dot reference `foo.bar` to manipulate single objects",
        ]
    }

    fn category() -> LintCategory {
        LintCategory::Pedantic
    }

    fn run(duck: &Duck) -> Vec<LintReport> {
        let mut reports = vec![];
        for keyword in duck.keywords() {
            if let (Token::With, position) = keyword {
                reports.push(LintReport {
                    position: position.clone(),
                })
            }
        }
        reports
    }
}
