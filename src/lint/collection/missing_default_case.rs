use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyStatementPass, Lint, LintLevel},
    parse::{Statement, StatementBox},
    FileId,
};

#[derive(Debug, PartialEq)]
pub struct MissingDefaultCase;
impl Lint for MissingDefaultCase {
    fn explanation() -> &'static str {
        "Switch statements are often used to express all possible outcomes of a limited data set, but by not implementing a default case, no code will run to handle any alternate or unexpected values."
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "missing_default_case"
    }
}

impl EarlyStatementPass for MissingDefaultCase {
    fn visit_statement_early(
        statement_box: &StatementBox,
        config: &crate::Config,
        reports: &mut Vec<Diagnostic<FileId>>,
    ) {
        if let Statement::Switch(switch) = statement_box.statement() {
            if switch.default_case().is_none() {
                let final_position = switch
                    .cases()
                    .iter()
                    .last()
                    .and_then(|case| case.iter_body_statements().last().map(|stmt| stmt.span().1))
                    .unwrap_or(statement_box.span().1);
                reports.push(
                    Self::diagnostic(config)
                        .with_message("Missing default case")
                        .with_labels(vec![
                            Label::primary(statement_box.file_id(), statement_box.span())
                                .with_message("switch statement is missing a default case"),
                            Label::secondary(statement_box.file_id(), final_position..final_position)
                                .with_message("add a default case here"),
                        ]),
                );
            }
        }
    }
}
