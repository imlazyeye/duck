use codespan_reporting::diagnostic::{Diagnostic, Label};
use colored::Colorize;

use crate::{
    lint::{EarlyStatementPass, Lint, LintLevel},
    parse::{Expression, Function, Statement, StatementBox},
    Config, FileId,
};

#[derive(Debug, PartialEq)]
pub struct UselessFunction;
impl Lint for UselessFunction {
    fn explanation() -> &'static str {
        "Anonymous functions that are not assigned to a variable can never be referenced."
    }

    fn default_level() -> LintLevel {
        LintLevel::Deny
    }

    fn tag() -> &'static str {
        "useless_function"
    }
}

impl EarlyStatementPass for UselessFunction {
    fn visit_statement_early(statement_box: &StatementBox, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let Statement::Expression(expression_box) = statement_box.statement() {
            if let Expression::FunctionDeclaration(Function { name: None, .. }) = expression_box.expression() {
                reports.push(
                    Self::diagnostic(config)
                        .with_message("Useless function")
                        .with_labels(vec![
                            Label::primary(expression_box.file_id(), expression_box.span())
                                .with_message("this function can never be referenced"),
                        ])
                        .with_notes(vec![format!(
                            "{}: turn this into a named function or save it into a variable",
                            "help".bold()
                        )]),
                );
            }
        }
    }
}
