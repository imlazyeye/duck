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

    fn visit_expression(
        duck: &Duck,
        expression: &Expression,
        position: &Position,
        reports: &mut Vec<LintReport>,
    ) {
        if let Some(todo_keyword) = duck.config().todo_keyword() {
            if let Expression::Call(caller, _, _) = expression {
                if let Expression::Identifier(name) = caller.inner() {
                    if name == todo_keyword {
                        reports.push(Self::generate_report(position.clone()))
                    }
                }
            }
        }
    }
}
