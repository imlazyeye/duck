use crate::{parsing::statement::Statement, Duck, Lint, LintCategory, LintReport, Span};

#[derive(Debug, PartialEq)]
pub struct WithLoop;
impl Lint for WithLoop {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
			tag: "with_loop",
			display_name: "Use of `with`",
			explanation: "The `with` loop allows your code's context to suddenly change, both making it more difficult to read (as a given line of code is no longer promised to be executing in the scope expected from the file), but also making it more difficult to track down all of the places an object is modified.",
			suggestions: vec![
            "Use `instance_find` if looping over objects",
            "Use direct dot reference `foo.bar` to manipulate single objects",
        ],
			category: LintCategory::Pedantic,
			span,
		}
    }

    fn visit_statement(
        _duck: &Duck,
        statement: &crate::parsing::statement::Statement,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Statement::With(..) = statement {
            reports.push(Self::generate_report(span))
        }
    }
}
