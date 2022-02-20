use crate::{parsing::expression::Expression, Duck, Lint, LintCategory, LintReport, Position};

#[derive(Debug, PartialEq)]
pub struct Todo;
impl Lint for Todo {
    fn generate_report(position: Position) -> LintReport {
        LintReport {
			display_name: "Use of todo marker",
			tag: "todo",
			explanation: "Todo markers are useful for work-in-progress code, but often are not intended to be permanently in place.",
			suggestions: vec!["Remove this todo marker"],
			category: LintCategory::Pedantic,
			position,
		}
    }
}
