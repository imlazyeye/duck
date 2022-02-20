use crate::{parsing::expression::Expression, Duck, Lint, LintCategory, LintReport, Position};

#[derive(Debug, PartialEq)]
pub struct ShowDebugMessage;
impl Lint for ShowDebugMessage {
    fn generate_report(position: Position) -> LintReport {
        LintReport {
			tag: "show_debug_message",
			display_name: "Use of `show_debug_message`",
			explanation: "Projects often implement their own logging framework and wish to avoid unwrapped prints to the console.",
			suggestions: vec![
            "Replace `show_debug_message` with a better logging function",
            "Remove this debug message",
        ],
			category: LintCategory::Pedantic,
			position,
		}
    }

    fn visit_expression(
        _duck: &Duck,
        expression: &Expression,
        position: &Position,
        reports: &mut Vec<LintReport>,
    ) {
        if let Expression::Call(caller, _, _) = expression {
            if let Expression::Identifier(name) = caller.inner() {
                if name == "show_debug_message" {
                    reports.push(Self::generate_report(position.clone()))
                }
            }
        }
    }
}
