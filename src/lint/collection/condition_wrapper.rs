use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyExpressionPass, EarlyStatementPass, Lint, LintLevel},
    parse::{
        DoUntil, Expression, ExpressionBox, If, RepeatLoop, Statement, StatementBox, Switch, Ternary, WhileLoop,
        WithLoop,
    },
    Config, FileId,
};

#[derive(Debug, PartialEq)]
pub struct ConditionWrapper;
impl Lint for ConditionWrapper {
    fn explanation() -> &'static str {
        "Parenthesis surrounding certain statement expressions are optional in GML, resulting in differing opinions on whether or not to use them. You can select either option via the config."
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "condition_wrapper"
    }
}

impl ConditionWrapper {
    pub fn test(expression_box: &ExpressionBox, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let Some(grouping) = expression_box.expression().as_grouping() {
            let (left_token, right_token) = grouping.parenthesis();
            if !config.statement_parentheticals {
                reports.push(
                    Self::diagnostic(config)
                        .with_message("Unneccesary grouping around condition")
                        .with_labels(vec![
                            Label::primary(expression_box.file_id(), left_token.span),
                            Label::primary(expression_box.file_id(), right_token.span),
                        ]),
                );
            }
        } else if config.statement_parentheticals {
            reports.push(
                Self::diagnostic(config)
                    .with_message("Missing outer grouping in statement")
                    .with_labels(vec![
                        Label::primary(expression_box.file_id(), expression_box.span())
                            .with_message("wrap this condition in parenthesis"),
                    ]),
            );
        }
    }
}

impl EarlyExpressionPass for ConditionWrapper {
    fn visit_expression_early(expression_box: &ExpressionBox, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let Expression::Ternary(Ternary { condition, .. }) = expression_box.expression() {
            Self::test(condition, config, reports)
        }
    }
}

impl EarlyStatementPass for ConditionWrapper {
    fn visit_statement_early(statement_box: &StatementBox, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        match statement_box.statement() {
            Statement::Switch(Switch {
                matching_value: expression,
                ..
            })
            | Statement::If(If {
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
            }) => Self::test(expression, config, reports),
            _ => {}
        };
    }
}
