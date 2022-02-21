use crate::{parsing::expression::Expression, Duck, Lint, LintCategory, LintReport, Span};

#[derive(Debug, PartialEq)]
pub struct ShowDebugMessage;
impl Lint for ShowDebugMessage {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
			tag: "show_debug_message",
			display_name: "Use of `show_debug_message`",
			explanation: "Projects often implement their own logging framework and wish to avoid unwrapped prints to the console.",
			suggestions: vec![
            "Replace `show_debug_message` with a better logging function",
            "Remove this debug message",
        ],
			category: LintCategory::Pedantic,
			span,
		}
    }

    fn visit_expression(
        _duck: &Duck,
        expression: &Expression,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Expression::Call(caller, _, _) = expression {
            if let Expression::Identifier(name) = caller.expression() {
                if name == "show_debug_message" {
                    reports.push(Self::generate_report(span))
                }
            }
        }
    }
}
