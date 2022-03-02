use crate::{
    lint::{EarlyExpressionPass, Lint, LintLevel, LintReport},
    parsing::{Call, Expression},
    utils::Span,
};

#[derive(Debug, PartialEq)]
pub struct ShowDebugMessage;
impl Lint for ShowDebugMessage {
    fn explanation() -> &'static str {
        "Projects often implement their own logging framework and wish to avoid unwrapped prints to the console."
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
        if let Expression::Call(Call { left, .. }) = expression {
            if let Expression::Identifier(identifier) = left.expression() {
                if identifier.name == "show_debug_message" {
                    Self::report(
                        "Use of `show_debug_message`",
                        [
                            "Replace `show_debug_message` with a better logging function".into(),
                            "Remove this debug message".into(),
                        ],
                        span,
                        reports,
                    )
                }
            }
        }
    }
}
