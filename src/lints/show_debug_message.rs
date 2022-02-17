use crate::{Lint, LintCategory};

pub struct ShowDebugMessage;
impl Lint for ShowDebugMessage {
    fn tag() -> &'static str {
        "show_debug_message"
    }

    fn display_name() -> &'static str {
        "Use of `show_debug_message`"
    }

    fn explanation() -> &'static str {
        "Projects often implement their own logging framework and wish to avoid unwrapped prints to the console."
    }

    fn suggestions() -> Vec<&'static str> {
        vec![
            "Replace `show_debug_message` with a better logging function",
            "Remove this debug message",
        ]
    }

    fn category() -> LintCategory {
        LintCategory::Pedantic
    }
}
