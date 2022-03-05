use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyStatementPass, Lint, LintLevel},
    parse::{Statement, StatementBox},
    FileId,
};

#[derive(Debug, PartialEq)]
pub struct WithLoop;
impl Lint for WithLoop {
    fn explanation() -> &'static str {
        "The `with` loop allows your code's context to suddenly change, both making it more difficult to read (as a given line of code is no longer promised to be executing in the scope expected from the file), but also making it more difficult to track down all of the places an object is modified."
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "with_loop"
    }
}

impl EarlyStatementPass for WithLoop {
    fn visit_statement_early(
        statement_box: &StatementBox,
        config: &crate::Config,
        reports: &mut Vec<Diagnostic<FileId>>,
    ) {
        if let Statement::WithLoop(..) = statement_box.statement() {
            reports.push(
                Self::diagnostic(config)
                    .with_message("Use of `with`")
                    .with_labels(vec![Label::primary(statement_box.file_id(), statement_box.span())]),
            );
        }
    }
}
