use crate::{parsing::expression::Expression, Duck, Lint, LintCategory, LintReport, Position};

#[derive(Debug, PartialEq)]
pub struct SingleSwitchCase;
impl Lint for SingleSwitchCase {
    fn generate_report(position: Position) -> LintReport {
        LintReport {
			display_name: "Single wwitch case",
			tag: "single_switch_case",
			explanation: "Switch statements that only match on a single element can be reduced to an `if` statement.",
			suggestions: vec!["Use an `if` statement instead of a `switch` statement"],
			category: LintCategory::Style,
			position,
		}
    }
}
