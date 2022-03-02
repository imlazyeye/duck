use crate::{
    lint::{Lint, LintLevel, LintReport},
    utils::Span,
};

#[derive(Debug, PartialEq)]
pub struct ModKeyword;
impl Lint for ModKeyword {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
            display_name: "Use of `mod`".into(),
            tag: Self::tag(),
            explanation: "GML supports both `mod` and `%` to perform modulo division -- `%` is more consistent with other languages and is preferred.",
            suggestions: vec!["Use `%` instead of `mod`".into()],
            default_level: Self::default_level(),
            span,
        }
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "mod_keyword"
    }
}
