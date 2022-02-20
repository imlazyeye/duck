use crate::{
    parsing::{statement::Statement},
    Duck, Lint, LintCategory, LintReport, Position,
};

#[derive(Debug, PartialEq)]
pub struct MissingDefaultCase;
impl Lint for MissingDefaultCase {
    fn generate_report(position: Position) -> LintReport {
        LintReport {
			display_name: "Missing default case",
			tag: "missing_default_case",
			explanation: "Switch statements are often used to express all possible outcomes of a limited data set, but by not implementing a default case, no code will run to handle any alternate or unexpected values.",
			suggestions: vec!["Add a default case to the switch statement"],
			category: LintCategory::Pedantic,
			position,
		}
    }

    fn visit_statement(
        _duck: &Duck,
        statement: &Statement,
        position: &Position,
        reports: &mut Vec<LintReport>,
    ) {
        if let Statement::Switch(_, _, None) = statement {
            reports.push(Self::generate_report(position.clone()))
        }
    }
}
