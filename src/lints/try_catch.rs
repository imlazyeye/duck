use crate::{LintCategory, XLint};

pub struct TryCatch;
impl XLint for TryCatch {
    fn tag(&self) -> &str {
        "try_catch"
    }

    fn display_name(&self) -> &str {
        "Use of `try` / `catch`"
    }

    fn explanation(&self) -> &str {
        "GML's try/catch will collect all errors as opposed to the precise ones wanted, allowing them to accidently catch errors that should not be surpressed."
    }

    fn suggestions(&self) -> Vec<&str> {
        vec!["Adjust the architecture to inspect for an issue prior to the crash"]
    }

    fn category(&self) -> LintCategory {
        LintCategory::Pedantic
    }
}
