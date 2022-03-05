use crate::{
    Config,
    lint::{EarlyStatementPass, Lint, LintLevel, LintReport},
    parse::{DoUntil, Expression, If, RepeatLoop, Statement, WhileLoop, WithLoop},
    parse::Span,
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
    fn visit_statement_early(config: &Config, statement: &Statement, span: Span, reports: &mut Vec<LintReport>) {
        let expression = match statement {
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
                Self::report(
                    "Parenthetical in statement expression",
                    [
                        "Remove the wrapping parenthesis from this expression".into(),
                        "Change your preferences for this lint in .duck.toml".into(),
                    ],
                    span,
                    reports,
                )
            } else if !has_grouping && config.statement_parentheticals {
                Self::report(
                    "Lacking parenthetical in statement expression",
                    [
                        "Add wrapping parenthesis from this expression".into(),
                        "Change your preferences for this lint in .duck.toml".into(),
                    ],
                    span,
                    reports,
                )
            }
        }
    }
}
