use crate::{
    lint::EarlyExpressionPass, parsing::Expression, utils::Span, Lint, LintLevel, LintReport,
};

#[derive(Debug, PartialEq)]
pub struct ShowDebugMessage;
impl Lint for ShowDebugMessage {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
            tag: Self::tag(),
			display_name: "Use of `show_debug_message`".into(),
			explanation: "Projects often implement their own logging framework and wish to avoid unwrapped prints to the console.",
			suggestions: vec![
            "Replace `show_debug_message` with a better logging function".into(),
            "Remove this debug message".into(),
        ],
			default_level: Self::default_level(),
			span,
		}
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "show_debug_message"
    }
}

impl EarlyExpressionPass for ShowDebugMessage {
    fn visit_expression_early(
        _config: &crate::Config,
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
