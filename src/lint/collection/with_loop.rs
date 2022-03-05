use crate::{
    lint::{EarlyStatementPass, Lint, LintLevel, LintReport},
    parse::Statement,
    parse::Span,
};

#[derive(Debug, PartialEq)]
pub struct WithLoop;
impl Lint for WithLoop {
    fn explanation() -> &'static str {
        "The `with` loop allows your code's context to suddenly change, both making it more difficult to read (as a given line of code is no longer promised to be executing in the scope expected from the file), but also making it more difficult to track down all of the places an object is modified."
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "with_loop"
    }
}

impl EarlyStatementPass for WithLoop {
    fn visit_statement_early(
        _config: &crate::Config,
        statement: &crate::parse::Statement,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Statement::WithLoop(..) = statement {
            Self::report(
                "Use of with loop",
                [
                    "Use `instance_find` if looping over objects".into(),
                    "Use direct dot reference `foo.bar` to manipulate single objects".into(),
                ],
                span,
                reports,
            )
        }
    }
}
