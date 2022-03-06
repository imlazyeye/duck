use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyStatementPass, Lint, LintLevel},
    parse::{Access, Assignment, Expression, Globalvar, Statement, StatementBox},
    FileId,
};

#[derive(Debug, PartialEq)]
pub struct Global;
impl Lint for Global {
    fn explanation() -> &'static str {
        "While useful at times, global variables reduce saftey since they can be accessed or mutated anywhere, and provide no guarentee that they've already been initiailized."
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "global"
    }
}

impl EarlyStatementPass for Global {
    fn visit_statement_early(
        statement_box: &StatementBox,
        config: &crate::Config,
        reports: &mut Vec<Diagnostic<FileId>>,
    ) {
        match statement_box.statement() {
            Statement::Assignment(Assignment { left, .. }) => {
                if let Expression::Access(Access::Global { .. }) = left.expression() {
                    reports.push(
                        Self::diagnostic(config)
                            .with_message("Use of global variable")
                            .with_labels(vec![
                                Label::primary(left.file_id(), left.span())
                                    .with_message("scope this variable to an individual object or struct"),
                            ]),
                    );
                }
            }
            Statement::GlobalvarDeclaration(Globalvar { .. }) => {
                reports.push(
                    Self::diagnostic(config)
                        .with_message("Use of global variable")
                        .with_labels(vec![
                            Label::primary(statement_box.file_id(), statement_box.span())
                                .with_message("scope this variable to an individual object or struct"),
                        ]),
                );
            }
            _ => {}
        }
    }
}
