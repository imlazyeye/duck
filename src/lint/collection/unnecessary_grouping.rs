use crate::{
    lint::{EarlyExprPass, EarlyStmtPass, Lint, LintLevel},
    parse::{Expr, ExprType, ParseVisitor, Stmt, StmtType},
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
        if let ExprType::Grouping(grouping) = expr.inner() {
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
            ExprType::Logical(_) | ExprType::Equality(_) | ExprType::Evaluation(_) | ExprType::Unary(_) => {}

            // This is style preference, which instead is linted by `condition_wrapper`.
            ExprType::Ternary(_) => {}

            // These should not directly own groupings
            ExprType::Function(_)
            | ExprType::NullCoalecence(_)
            | ExprType::Postfix(_)
            | ExprType::Access(_)
            | ExprType::Call(_)
            | ExprType::Grouping(_)
            | ExprType::Literal(_)
            | ExprType::Identifier(_) => expr.visit_child_exprs(|expr| Self::test(expr, config, reports)),
        }
    }
}

impl EarlyStmtPass for UnnecessaryGrouping {
    fn visit_stmt_early(stmt: &Stmt, config: &crate::Config, reports: &mut Vec<Diagnostic<FileId>>) {
        match stmt.inner() {
            // These are a style preference, which instead is linted by `condition_wrapper`.
            StmtType::TryCatch(_)
            | StmtType::ForLoop(_)
            | StmtType::WithLoop(_)
            | StmtType::RepeatLoop(_)
            | StmtType::DoUntil(_)
            | StmtType::WhileLoop(_)
            | StmtType::If(_)
            | StmtType::Switch(_) => {}

            // These should not directly own groupings
            StmtType::Return(_) | StmtType::Throw(_) | StmtType::Delete(_) | StmtType::Assignment(_) => {
                stmt.visit_child_exprs(|expr| Self::test(expr, config, reports))
            }
            _ => {}
        };
    }
}
