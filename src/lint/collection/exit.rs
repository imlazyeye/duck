use crate::{
    lint::{EarlyStatementPass, Lint, LintLevel, LintReport},
    parse::Statement,
    parse::Span,
};

#[derive(Debug, PartialEq)]
pub struct Exit;
impl Lint for Exit {
    fn explanation() -> &'static str {
        "`return` can always be used in place of exit, which provides more consistency across your codebase."
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
            Self::report(
                "Use of `exit`",
                ["Use `return` instead of `exit`".into()],
                span,
                reports,
            );
        }
    }
}
