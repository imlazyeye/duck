use crate::{parsing::expression::Expression, Duck, Lint, LintCategory, LintReport, Span};

#[derive(Debug, PartialEq)]
pub struct Todo;
impl Lint for Todo {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
			display_name: "Use of todo marker",
			tag: "todo",
			explanation: "Todo markers are useful for work-in-progress code, but often are not intended to be permanently in place.",
			suggestions: vec!["Remove this todo marker"],
			category: LintCategory::Pedantic,
			span,
		}
    }

    fn visit_expression(
        duck: &Duck,
        expression: &Expression,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Some(todo_keyword) = duck.config().todo_keyword() {
            if let Expression::Call(caller, _, _) = expression {
                if let Expression::Identifier(name) = caller.expression() {
                    if name == todo_keyword {
                        reports.push(Self::generate_report(span))
                    }
                }
            }
        }
    }
}
