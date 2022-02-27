use crate::{utils::Span, Lint, LintReport, LintLevel};

#[derive(Debug, PartialEq)]
pub struct TooManyLines;
impl Lint for TooManyLines {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
			display_name: "Too many lines".into(),
            tag: Self::tag(),
			explanation: "Functions with lots of lines are harder to work with due to the volume of code that must be read to understand them.",
			suggestions: vec!["Split this into multiple functions".into()],
			default_level: Self::default_level(),
			span,
		}
    }

    fn default_level() -> LintLevel {
        LintLevel::Warn
    }

    fn tag() -> &'static str {
        "too_many_lines"
    }
}
