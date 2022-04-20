use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyExprPass, EarlyStmtPass, Lint, LintLevel},
    parse::{
        Assignment, AssignmentOp, Equality, EqualityOp, Evaluation, EvaluationOp, Expr, ExprKind, Literal, Logical,
        LogicalOp, Stmt, StmtKind,
    },
    FileId,
};

#[derive(Debug, PartialEq)]
pub struct SuspicousConstantUsage;
impl Lint for SuspicousConstantUsage {
    fn explanation() -> &'static str {
        "Using a constant outside of equalities and direct assignments is likely unintended or misunderstood code."
    }

    fn default_level() -> LintLevel {
        LintLevel::Deny
    }

    fn tag() -> &'static str {
        "suspicious_constant_usage"
    }
}

impl SuspicousConstantUsage {
    fn report_expr(expr: &Expr, config: &crate::Config, reports: &mut Vec<Diagnostic<FileId>>) {
        reports.push(
            Self::diagnostic(config)
                .with_message("Suspicious constant usage")
                .with_labels(vec![
                    Label::primary(expr.file_id(), expr.span()).with_message("using this operator..."),
                    Label::primary(expr.file_id(), expr.span())
                        .with_message("...with this literal, which is not a coherent operation"),
                ]),
        );
    }
}

impl EarlyExprPass for SuspicousConstantUsage {
    fn visit_expr_early(expr: &Expr, config: &crate::Config, reports: &mut Vec<Diagnostic<FileId>>) {
        match expr.inner() {
            ExprKind::Evaluation(Evaluation {
                op: operator, right, ..
            }) => {
                if let Some(literal) = right.inner().as_literal() {
                    if literal_is_suspicous(literal, OpWrapper::Evaluation(*operator)) {
                        Self::report_expr(right, config, reports);
                    }
                }
            }
            ExprKind::Logical(Logical {
                op: operator, right, ..
            }) => {
                if let Some(literal) = right.inner().as_literal() {
                    if literal_is_suspicous(literal, OpWrapper::Logical(*operator)) {
                        Self::report_expr(right, config, reports);
                    }
                }
            }
            ExprKind::Equality(Equality {
                op: operator, right, ..
            }) => {
                if let Some(literal) = right.inner().as_literal() {
                    if literal_is_suspicous(literal, OpWrapper::Equality(*operator)) {
                        Self::report_expr(right, config, reports);
                    }
                }
            }
            _ => {}
        }
    }
}
impl EarlyStmtPass for SuspicousConstantUsage {
    fn visit_stmt_early(stmt: &Stmt, config: &crate::Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let StmtKind::Assignment(Assignment {
            op: operator, right, ..
        }) = stmt.inner()
        {
            if !matches!(
                *operator,
                AssignmentOp::Identity(_) | AssignmentOp::NullCoalecenceEqual(_)
            ) {
                if let Some(literal) = right.inner().as_literal() {
                    if literal_is_suspicous(literal, OpWrapper::Assignment(*operator)) {
                        Self::report_expr(right, config, reports);
                    }
                }
            }
        }
    }
}

fn literal_is_suspicous(literal: &Literal, operation_wrapper: OpWrapper) -> bool {
    match operation_wrapper {
        OpWrapper::Assignment(AssignmentOp::Identity(_)) => return false,
        OpWrapper::Equality(op) => {
            if matches!(op, EqualityOp::Equal(_) | EqualityOp::NotEqual(_)) {
                return false;
            }
        }
        _ => {}
    }
    match literal {
        Literal::True
        | Literal::False
        | Literal::Undefined
        | Literal::Noone
        | Literal::Array(_)
        | Literal::Struct(_) => true,
        Literal::String(_) | Literal::Real(_) | Literal::Hex(_) => false,
        Literal::Misc(literal) => {
            match literal.as_str() {
                "tile_index_mask" | "tile_flip" | "tile_mirror" | "tile_rotate" => {
                    // This is intended for bit masking, so only allow it if its an evaluation involving binary...
                    match operation_wrapper {
                        OpWrapper::Assignment(op) => !matches!(
                            op,
                            AssignmentOp::XorEqual(_) | AssignmentOp::OrEqual(_) | AssignmentOp::AndEqual(_),
                        ),
                        OpWrapper::Evaluation(op) => !matches!(
                            op,
                            EvaluationOp::And(_)
                                | EvaluationOp::Or(_)
                                | EvaluationOp::Xor(_)
                                | EvaluationOp::BitShiftLeft(_)
                                | EvaluationOp::BitShiftRight(_)
                        ),
                        OpWrapper::Logical(_) => true,
                        OpWrapper::Equality(_) => true,
                    }
                }
                _ => {
                    // The rest are all sus
                    true
                }
            }
        }
    }
}

enum OpWrapper {
    Assignment(AssignmentOp),
    Evaluation(EvaluationOp),
    Logical(LogicalOp),
    Equality(EqualityOp),
}
