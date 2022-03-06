use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyStatementPass, Lint, LintLevel},
    parse::{Statement, StatementBox, Switch},
    FileId,
};

#[derive(Debug, PartialEq)]
pub struct SingleSwitchCase;
impl Lint for SingleSwitchCase {
    fn explanation() -> &'static str {
        "Switch statements that only match on a single element can be reduced to an `if` statement."
    }

    fn default_level() -> LintLevel {
        LintLevel::Warn
    }

    fn tag() -> &'static str {
        "single_switch_case"
    }
}

impl EarlyStatementPass for SingleSwitchCase {
    fn visit_statement_early(
        statement_box: &StatementBox,
        config: &crate::Config,
        reports: &mut Vec<Diagnostic<FileId>>,
    ) {
        if let Statement::Switch(Switch {
            cases, default_case, ..
        }) = statement_box.statement()
        {
            if cases.len() == 1 {
                if default_case.is_some() {
                    reports.push(
                        Self::diagnostic(config)
                            .with_message("Switch statement with single case")
                            .with_labels(vec![
                                Label::primary(statement_box.file_id(), statement_box.span())
                                    .with_message("Use an `if/else` statement instead of a `switch` statement"),
                            ]),
                    );
                } else {
                    reports.push(
                        Self::diagnostic(config)
                            .with_message("Switch statement with single case")
                            .with_labels(vec![
                                Label::primary(statement_box.file_id(), statement_box.span())
                                    .with_message("Use an `if` statement instead of a `switch` statement"),
                            ]),
                    );
                }
            }
        }
    }
}
