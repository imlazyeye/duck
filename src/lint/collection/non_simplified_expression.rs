use crate::{
    FileId,
    lint::{EarlyExprPass, Lint, LintLevel},
    parse::{Evaluation, EvaluationOp, Expr, ExprKind, Grouping, Literal},
};
use codespan_reporting::diagnostic::{Diagnostic, Label};

#[derive(Debug, PartialEq)]
pub struct NonSimplifiedExpression;
impl Lint for NonSimplifiedExpression {
    fn explanation() -> &'static str {
        "Operating on two constant numbers can be reduced for brevity."
    }

    fn default_level() -> LintLevel {
        LintLevel::Warn
    }

    fn tag() -> &'static str {
        "non_simplified_expression"
    }
}

impl NonSimplifiedExpression {
    fn filter_groups(expr: &Expr) -> Option<&Expr> {
        match expr.kind() {
            ExprKind::Grouping(Grouping { inner, .. }) => Self::filter_groups(inner),
            ExprKind::Literal(_) => Some(expr),
            _ => None,
        }
    }
}

impl EarlyExprPass for NonSimplifiedExpression {
    fn visit_expr_early(expr: &Expr, config: &crate::Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let ExprKind::Evaluation(Evaluation { left, op, right }) = expr.kind() {
            let left = Self::filter_groups(left).unwrap_or(left);
            let right = Self::filter_groups(right).unwrap_or(right);

            // Only continue if the two expressions are both reals or both strings
            if !matches!(
                (left.kind(), right.kind()),
                (
                    ExprKind::Literal(Literal::String(_)),
                    ExprKind::Literal(Literal::String(_))
                ) | (ExprKind::Literal(Literal::Real(_)), ExprKind::Literal(Literal::Real(_)))
            ) {
                return;
            }

            let message = match op {
                EvaluationOp::And(_)
                | EvaluationOp::Or(_)
                | EvaluationOp::Xor(_)
                | EvaluationOp::BitShiftLeft(_)
                | EvaluationOp::BitShiftRight(_)
                    if config.simplification_rules.check_bitwise =>
                {
                    "Binary operation can be simplified"
                }
                EvaluationOp::Plus(_) if config.simplification_rules.check_addition => "Addition can be simplified",
                EvaluationOp::Minus(_) if config.simplification_rules.check_addition => "Subtraction can be simplified",
                EvaluationOp::Star(_) if config.simplification_rules.check_multiplication => {
                    "Multiplication can be simplified"
                }
                EvaluationOp::Slash(_) | EvaluationOp::Div(_) | EvaluationOp::Modulo(_)
                    if config.simplification_rules.check_division =>
                {
                    "Division can be simplified"
                }
                _ => return,
            };

            reports.push(Self::diagnostic(config).with_message(message).with_labels(vec![
                Label::primary(expr.file_id(), expr.span()).with_message("this expression can be reduced"),
            ]));
        }
    }
}
