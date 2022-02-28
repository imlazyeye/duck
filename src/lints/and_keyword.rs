use crate::{utils::Span, Lint, LintLevel, LintReport};

#[derive(Debug, PartialEq)]
pub struct AndKeyword;
impl Lint for AndKeyword {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
            display_name: "Use of `and`".into(),
            tag: Self::tag(),
            explanation: "GML supports both `and` and `&&` to refer to logical and -- `&&` is more consistent with other languages and is preferred.",
            suggestions: vec!["Use `&&` instead of `and`".into()],
            default_level: Self::default_level(),
            span,
        }
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "and_keyword"
    }
}
