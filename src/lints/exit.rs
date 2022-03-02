use crate::{
    lint::{EarlyStatementPass, Lint, LintLevel, LintReport},
    parsing::Statement,
    utils::Span,
};

#[derive(Debug, PartialEq)]
pub struct Exit;
impl Lint for Exit {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
            display_name: "Use of `exit`".into(),
            tag: Self::tag(),
            explanation: "`return` can always be used in place of exit, which provides more consistency across your codebase.",
            suggestions: vec!["Use `return` instead of `exit`".into()],
            default_level: Self::default_level(),
            span,
        }
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "exit"
    }
}

impl EarlyStatementPass for Exit {
    fn visit_statement_early(
        _config: &crate::Config,
        statement: &Statement,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if *statement == Statement::Exit {
            reports.push(Self::generate_report(span))
        }
    }
}
