use crate::{
    parsing::expression::{AccessScope, Expression},
    Duck, Lint, LintCategory, LintReport, Position,
};

#[derive(Debug, PartialEq)]
pub struct Global;
impl Lint for Global {
    fn generate_report(position: Position) -> LintReport {
        LintReport {
			display_name: "Use of `global`",
			tag: "global",
			explanation: "While useful at times, global variables reduce saftey since they can be accessed or mutated anywhere.",
			suggestions: vec!["Scope this variable to an individual object"],
			category: LintCategory::Pedantic,
			position,
		}
    }

    fn visit_expression(
        duck: &Duck,
        expression: &Expression,
        position: &Position,
        reports: &mut Vec<LintReport>,
    ) {
        if let Expression::DotAccess(AccessScope::Global, _) = expression {
            reports.push(Self::generate_report(position.clone()))
        }
    }
}
