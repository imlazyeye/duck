use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyStatementPass, Lint, LintLevel},
    parse::{Assignment, Expression, Statement, StatementBox},
    Config, FileId,
};

#[derive(Debug, PartialEq)]
pub struct AssignmentToCall;
impl Lint for AssignmentToCall {
    fn explanation() -> &'static str {
        "While possible to compile, assigning a value to the call of a function does not do anything."
    }

    fn default_level() -> LintLevel {
        LintLevel::Deny
    }

    fn tag() -> &'static str {
        "assignment_to_call"
    }
}

impl EarlyStatementPass for AssignmentToCall {
    fn visit_statement_early(statement_box: &StatementBox, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let Statement::Assignment(Assignment { left, right, .. }) = statement_box.statement() {
            if let Expression::Call(..) = left.expression() {
                reports.push(
                    Self::diagnostic(config)
                        .with_message("Assignment to call")
                        .with_labels(vec![
                            Label::secondary(right.file_id(), right.span()).with_message("assigning this value..."),
                            Label::primary(left.file_id(), left.span())
                                .with_message("...to this function call, which does not do anything."),
                        ]),
                );
            }
        }
    }
}
