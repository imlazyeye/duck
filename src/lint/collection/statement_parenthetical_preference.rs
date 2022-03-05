use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyStatementPass, Lint, LintLevel},
    parse::{DoUntil, Expression, If, RepeatLoop, Statement, StatementBox, WhileLoop, WithLoop},
    Config, FileId,
};

#[derive(Debug, PartialEq)]
pub struct StatementParentheticalPreference;
impl Lint for StatementParentheticalPreference {
    fn explanation() -> &'static str {
        "Parenthesis surrounding statement expressions are optional in GML, resulting in differing opinions on whether or not to use them. You can select either option via the config."
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "statement_parenthetical_preference"
    }
}

impl EarlyStatementPass for StatementParentheticalPreference {
    fn visit_statement_early(statement_box: &StatementBox, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        let expression = match statement_box.statement() {
            Statement::Switch(switch) => Some(switch.matching_value()),
            Statement::If(If {
                condition: expression, ..
            })
            | Statement::DoUntil(DoUntil {
                condition: expression, ..
            })
            | Statement::WhileLoop(WhileLoop {
                condition: expression, ..
            })
            | Statement::WithLoop(WithLoop {
                identity: expression, ..
            })
            | Statement::RepeatLoop(RepeatLoop {
                tick_counts: expression,
                ..
            }) => Some(expression),
            _ => None,
        };
        if let Some(expression) = expression {
            let has_grouping = matches!(expression.expression(), Expression::Grouping(_));
            if has_grouping && !config.statement_parentheticals {
                reports.push(
                    Self::diagnostic(config)
                        .with_message("Unneccesary grouping in statement")
                        .with_labels(vec![
                            Label::primary(expression.file_id(), expression.span())
                                .with_message("the surrounding parenthesis can be omitted"),
                        ]),
                );
            } else if !has_grouping && config.statement_parentheticals {
                reports.push(
                    Self::diagnostic(config)
                        .with_message("Missing outer grouping in statement")
                        .with_labels(vec![
                            Label::primary(expression.file_id(), expression.span())
                                .with_message("wrap this statement's expression in parenthesis"),
                        ]),
                );
            }
        }
    }
}
