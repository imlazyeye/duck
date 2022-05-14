use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyExprPass, EarlyStmtPass, Lint, LintLevel},
    parse::{DoUntil, Expr, ExprKind, If, Repeat, Stmt, StmtKind, Switch, Ternary, While, With},
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
        if let Some(grouping) = expr.kind().as_grouping() {
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
        if let ExprKind::Ternary(Ternary { condition, .. }) = expr.kind() {
            Self::test(condition, config, reports)
        }
    }
}

impl EarlyStmtPass for ConditionWrapper {
    fn visit_stmt_early(stmt: &Stmt, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        match stmt.kind() {
            StmtKind::Switch(Switch {
                matching_value: expr, ..
            })
            | StmtKind::If(If { condition: expr, .. })
            | StmtKind::DoUntil(DoUntil { condition: expr, .. })
            | StmtKind::While(While { condition: expr, .. })
            | StmtKind::With(With { identity: expr, .. })
            | StmtKind::Repeat(Repeat { tick_counts: expr, .. }) => Self::test(expr, config, reports),
            _ => {}
        };
    }
}
