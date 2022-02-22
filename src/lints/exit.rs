use crate::{parsing::statement::Statement, Duck, Lint, LintCategory, LintReport, Span};

#[derive(Debug, PartialEq)]
pub struct Exit;
impl Lint for Exit {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
			display_name: "Use of `exit`".into(),
            tag: Self::tag(),
			explanation: "`return` can always be used in place of exit, which provides more consistency across your codebase.",
			suggestions: vec!["Use `return` instead of `exit`".into()],
			category: Self::category(),
			span,
		}
    }

    fn category() -> LintCategory {
        LintCategory::Style
    }

    fn tag() -> &'static str {
        "exit"
    }

    fn visit_statement(
        _duck: &Duck,
        statement: &Statement,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if *statement == Statement::Exit {
            reports.push(Self::generate_report(span))
        }
    }
}
