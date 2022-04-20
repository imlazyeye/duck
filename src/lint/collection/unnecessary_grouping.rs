use crate::{
    lint::{EarlyExprPass, EarlyStmtPass, Lint, LintLevel},
    parse::{Expr, ExprKind, ParseVisitor, Stmt, StmtKind},
    FileId,
};
use codespan_reporting::diagnostic::{Diagnostic, Label};

#[derive(Debug, PartialEq)]
pub struct UnnecessaryGrouping;
impl Lint for UnnecessaryGrouping {
    fn explanation() -> &'static str {
        "Parenthesis around an expression that do not change how the logic is executed are redundant and can be removed."
    }

    fn default_level() -> LintLevel {
        LintLevel::Warn
    }

    fn tag() -> &'static str {
        "unnecessary_grouping"
    }
}

impl UnnecessaryGrouping {
    fn test(expr: &Expr, config: &crate::Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let ExprKind::Grouping(grouping) = expr.inner() {
            let (left_token, right_token) = grouping.parenthesis();
            reports.push(
                Self::diagnostic(config)
                    .with_message("Unnecessary grouping")
                    .with_labels(vec![
                        Label::primary(expr.file_id(), left_token.span),
                        Label::primary(expr.file_id(), right_token.span),
                    ]),
            );
        }
    }
}

impl EarlyExprPass for UnnecessaryGrouping {
    fn visit_expr_early(expr: &Expr, config: &crate::Config, reports: &mut Vec<Diagnostic<FileId>>) {
        match expr.inner() {
            // These are the blessed expressions that utilize groupings in meaningful ways
            ExprKind::Logical(_) | ExprKind::Equality(_) | ExprKind::Evaluation(_) | ExprKind::Unary(_) => {}

            // This is style preference, which instead is linted by `condition_wrapper`.
            ExprKind::Ternary(_) => {}

            // These should not directly own groupings
            ExprKind::Enum(_)
            | ExprKind::Macro(_)
            | ExprKind::Function(_)
            | ExprKind::NullCoalecence(_)
            | ExprKind::Postfix(_)
            | ExprKind::Access(_)
            | ExprKind::Call(_)
            | ExprKind::Grouping(_)
            | ExprKind::Literal(_)
            | ExprKind::Identifier(_) => expr.visit_child_exprs(|expr| Self::test(expr, config, reports)),
        }
    }
}

impl EarlyStmtPass for UnnecessaryGrouping {
    fn visit_stmt_early(stmt: &Stmt, config: &crate::Config, reports: &mut Vec<Diagnostic<FileId>>) {
        match stmt.inner() {
            // These are a style preference, which instead is linted by `condition_wrapper`.
            StmtKind::TryCatch(_)
            | StmtKind::ForLoop(_)
            | StmtKind::WithLoop(_)
            | StmtKind::RepeatLoop(_)
            | StmtKind::DoUntil(_)
            | StmtKind::WhileLoop(_)
            | StmtKind::If(_)
            | StmtKind::Switch(_) => {}

            // These should not directly own groupings
            StmtKind::Return(_) | StmtKind::Throw(_) | StmtKind::Delete(_) | StmtKind::Assignment(_) => {
                stmt.visit_child_exprs(|expr| Self::test(expr, config, reports))
            }
            _ => {}
        };
    }
}
