use crate::{LintCategory, XLint};

pub struct ShowDebugMessage;
impl XLint for ShowDebugMessage {
    fn tag(&self) -> &str {
        "show_debug_message"
    }

    fn display_name(&self) -> &str {
        "Use of `show_debug_message`"
    }

    fn explanation(&self) -> &str {
        "Projects often implement their own logging framework and wish to avoid unwrapped prints to the console."
    }

    fn suggestions(&self) -> Vec<&str> {
        vec![
            "Replace `show_debug_message` with a better logging function",
            "Remove this debug message",
        ]
    }

    fn category(&self) -> LintCategory {
        LintCategory::Pedantic
    }
}
