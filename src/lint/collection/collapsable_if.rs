use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyStatementPass, Lint, LintLevel},
    parse::{If, Statement, StatementBox},
    Config, FileId,
};

#[derive(Debug, PartialEq)]
pub struct CollapsableIf;
impl Lint for CollapsableIf {
    fn explanation() -> &'static str {
        "If statements that contain nothing more than another if statement can be collapsed into a single statement."
    }

    fn default_level() -> LintLevel {
        LintLevel::Warn
    }

    fn tag() -> &'static str {
        "collapsable_if"
    }
}

impl EarlyStatementPass for CollapsableIf {
    fn visit_statement_early(statement_box: &StatementBox, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let Statement::If(If { body: first_body, .. }) = statement_box.statement() {
            if let Some(block) = first_body.statement().as_block().filter(|block| block.body.len() == 1) {
                let nested_statement = block.body.first().unwrap();
                if let Statement::If(If {
                    else_statement: None, ..
                }) = nested_statement.statement()
                {
                    reports.push(
                        Self::diagnostic(config)
                            .with_message("Collapsable if statement")
                            .with_labels(vec![
                                Label::secondary(nested_statement.file_id(), nested_statement.span())
                                    .with_message("nested if statement"),
                                Label::primary(statement_box.file_id(), statement_box.span())
                                    .with_message("this can be combined with the nested if statement"),
                            ]),
                    )
                }
            }
        }
    }
}
