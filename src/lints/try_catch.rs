use crate::{
    parsing::{statement::Statement},
    Duck, Lint, LintCategory, LintReport, Position,
};

#[derive(Debug, PartialEq)]
pub struct TryCatch;
impl Lint for TryCatch {
    fn generate_report(position: Position) -> LintReport {
        LintReport {
			display_name: "Use of `try` / `catch`",
			tag: "try_catch",
			explanation: "GML's try/catch will collect all errors as opposed to the precise ones wanted, allowing them to accidently catch errors that should not be surpressed.",
			suggestions: vec!["Adjust the architecture to inspect for an issue prior to the crash"],
			category: LintCategory::Pedantic,
			position,
		}
    }

    fn visit_statement(
        _duck: &Duck,
        statement: &crate::parsing::statement::Statement,
        position: &Position,
        reports: &mut Vec<LintReport>,
    ) {
        if let Statement::TryCatch(..) = statement {
            reports.push(Self::generate_report(position.clone()))
        }
    }
}
