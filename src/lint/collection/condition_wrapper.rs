use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyExprPass, EarlyStmtPass, Lint, LintLevel},
    parse::{DoUntil, Expr, ExprType, If, RepeatLoop, Stmt, StmtType, Switch, Ternary, WhileLoop, WithLoop},
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
    pub fn test(expr: &Expr, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let Some(grouping) = expr.inner().as_grouping() {
            let (left_token, right_token) = grouping.parenthesis();
            if !config.statement_parentheticals {
                reports.push(
                    Self::diagnostic(config)
                        .with_message("Unneccesary grouping around condition")
                        .with_labels(vec![
                            Label::primary(expr.file_id(), left_token.span),
                            Label::primary(expr.file_id(), right_token.span),
                        ]),
                );
            }
        } else if config.statement_parentheticals {
            reports.push(
                Self::diagnostic(config)
                    .with_message("Missing outer grouping in statement")
                    .with_labels(vec![
                        Label::primary(expr.file_id(), expr.span()).with_message("wrap this condition in parenthesis"),
                    ]),
            );
        }
    }
}

impl EarlyExprPass for ConditionWrapper {
    fn visit_expr_early(expr: &Expr, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let ExprType::Ternary(Ternary { condition, .. }) = expr.inner() {
            Self::test(condition, config, reports)
        }
    }
}

impl EarlyStmtPass for ConditionWrapper {
    fn visit_stmt_early(stmt: &Stmt, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        match stmt.inner() {
            StmtType::Switch(Switch {
                matching_value: expr, ..
            })
            | StmtType::If(If { condition: expr, .. })
            | StmtType::DoUntil(DoUntil { condition: expr, .. })
            | StmtType::WhileLoop(WhileLoop { condition: expr, .. })
            | StmtType::WithLoop(WithLoop { identity: expr, .. })
            | StmtType::RepeatLoop(RepeatLoop { tick_counts: expr, .. }) => Self::test(expr, config, reports),
            _ => {}
        };
    }
}
