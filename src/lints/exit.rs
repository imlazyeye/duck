use crate::{
    parsing::{statement::Statement},
    Duck, Lint, LintCategory, LintReport, Position,
};

#[derive(Debug, PartialEq)]
pub struct Exit;
impl Lint for Exit {
    fn generate_report(position: Position) -> LintReport {
        LintReport {
			display_name: "Use of `exit`",
			tag: "exit",
			explanation: "`return` can always be used in place of exit, which provides more consistency across your codebase.",
			suggestions: vec!["Use `return` instead of `exit`"],
			category: LintCategory::Style,
			position,
		}
    }

    fn visit_statement(
        _duck: &Duck,
        statement: &Statement,
        position: &Position,
        reports: &mut Vec<LintReport>,
    ) {
        if *statement == Statement::Exit {
            reports.push(Self::generate_report(position.clone()))
        }
    }
}
