use crate::{utils::Span, Lint, LintCategory, LintReport};

#[derive(Debug, PartialEq)]
pub struct TooManyLines;
impl Lint for TooManyLines {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
			display_name: "Too many lines".into(),
            tag: Self::tag(),
			explanation: "Functions with lots of lines are harder to work with due to the volume of code that must be read to understand them.",
			suggestions: vec!["Split this into multiple functions".into()],
			category: Self::category(),
			span,
		}
    }

    fn category() -> LintCategory {
        LintCategory::Style
    }

    fn tag() -> &'static str {
        "too_many_lines"
    }
}
