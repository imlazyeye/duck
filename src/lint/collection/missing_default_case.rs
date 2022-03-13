use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyStmtPass, Lint, LintLevel},
    parse::{Stmt, StmtType},
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

impl EarlyStmtPass for MissingDefaultCase {
    fn visit_stmt_early(stmt: &Stmt, config: &crate::Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let StmtType::Switch(switch) = stmt.inner() {
            if switch.default_case().is_none() {
                let final_position = switch
                    .cases()
                    .iter()
                    .last()
                    .and_then(|case| case.iter_body_statements().last().map(|stmt| stmt.span().end()))
                    .unwrap_or_else(|| stmt.span().end());
                reports.push(
                    Self::diagnostic(config)
                        .with_message("Missing default case")
                        .with_labels(vec![
                            Label::primary(stmt.file_id(), stmt.span())
                                .with_message("switch statement is missing a default case"),
                            Label::secondary(stmt.file_id(), final_position..final_position)
                                .with_message("add a default case here"),
                        ]),
                );
            }
        }
    }
}
