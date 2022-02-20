use crate::{parsing::expression::Expression, Duck, Lint, LintCategory, LintReport, Position};

#[derive(Debug, PartialEq)]
pub struct TooManyLines;
impl Lint for TooManyLines {
    fn generate_report(position: Position) -> LintReport {
        LintReport {
			display_name: "Too many lines",
			tag: "too_many_lines",
			explanation: "Functions with lots of lines are harder to work with due to the volume of code that must be read to understand them.",
			suggestions: vec!["Split this into multiple functions"],
			category: LintCategory::Style,
			position,
		}
    }
}
