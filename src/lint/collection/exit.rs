use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyStatementPass, Lint, LintLevel},
    parse::{Statement, StatementBox},
    FileId,
};

#[derive(Debug, PartialEq)]
pub struct Exit;
impl Lint for Exit {
    fn explanation() -> &'static str {
        "`return` can always be used in place of exit, which provides more consistency across your codebase."
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "exit"
    }
}

impl EarlyStatementPass for Exit {
    fn visit_statement_early(
        statement_box: &StatementBox,
        config: &crate::Config,
        reports: &mut Vec<Diagnostic<FileId>>,
    ) {
        if let Statement::Exit = statement_box.statement() {
            reports.push(Self::diagnostic(config).with_message("Use of `exit`").with_labels(vec![
                    Label::primary(statement_box.file_id(), statement_box.span())
                        .with_message("replace this with `return`"),
                ]));
        }
    }
}
