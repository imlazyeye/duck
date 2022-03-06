use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyExpressionPass, EarlyStatementPass, Lint, LintLevel},
    parse::{
        Assignment, AssignmentOperator, Equality, EqualityOperator, Evaluation, EvaluationOperator, Expression,
        ExpressionBox, Literal, Logical, LogicalOperator, Statement, StatementBox,
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
    fn report_expr(expression_box: &ExpressionBox, config: &crate::Config, reports: &mut Vec<Diagnostic<FileId>>) {
        reports.push(
            Self::diagnostic(config)
                .with_message("Suspicious constant usage")
                .with_labels(vec![
                    Label::primary(expression_box.file_id(), expression_box.span())
                        .with_message("using this operator..."),
                    Label::primary(expression_box.file_id(), expression_box.span())
                        .with_message("...with this literal, which is not a coherent operation"),
                ]),
        );
    }
}

impl EarlyExpressionPass for SuspicousConstantUsage {
    fn visit_expression_early(
        expression_box: &ExpressionBox,
        config: &crate::Config,
        reports: &mut Vec<Diagnostic<FileId>>,
    ) {
        match expression_box.expression() {
            Expression::Evaluation(Evaluation { operator, right, .. }) => {
                if let Some(literal) = right.expression().as_literal() {
                    if literal_is_suspicous(literal, OperationWrapper::Evaluation(*operator)) {
                        Self::report_expr(right, config, reports);
                    }
                }
            }
            Expression::Logical(Logical { operator, right, .. }) => {
                if let Some(literal) = right.expression().as_literal() {
                    if literal_is_suspicous(literal, OperationWrapper::Logical(*operator)) {
                        Self::report_expr(right, config, reports);
                    }
                }
            }
            Expression::Equality(Equality { operator, right, .. }) => {
                if let Some(literal) = right.expression().as_literal() {
                    if literal_is_suspicous(literal, OperationWrapper::Equality(*operator)) {
                        Self::report_expr(right, config, reports);
                    }
                }
            }
            _ => {}
        }
    }
}
impl EarlyStatementPass for SuspicousConstantUsage {
    fn visit_statement_early(
        statement_box: &StatementBox,
        config: &crate::Config,
        reports: &mut Vec<Diagnostic<FileId>>,
    ) {
        if let Statement::Assignment(Assignment { operator, right, .. }) = statement_box.statement() {
            if !matches!(
                *operator,
                AssignmentOperator::Equal(_) | AssignmentOperator::NullCoalecenceEqual(_)
            ) {
                if let Some(literal) = right.expression().as_literal() {
                    if literal_is_suspicous(literal, OperationWrapper::Assignment(*operator)) {
                        Self::report_expr(right, config, reports);
                    }
                }
            }
        }
    }
}

fn literal_is_suspicous(literal: &Literal, operation_wrapper: OperationWrapper) -> bool {
    match operation_wrapper {
        OperationWrapper::Assignment(AssignmentOperator::Equal(_))
        | OperationWrapper::Equality(EqualityOperator::Equal(_)) => return false,
        _ => {}
    }
    match literal {
        Literal::True
        | Literal::False
        | Literal::Undefined
        | Literal::Noone
        | Literal::Array(_)
        | Literal::Struct(_) => true,
        Literal::String(_) | Literal::Real(_) | Literal::Hex(_) => true,
        Literal::Misc(literal) => {
            match literal.as_str() {
                "tile_index_mask" | "tile_flip" | "tile_mirror" | "tile_rotate" => {
                    // This is intended for bit masking, so only allow it if its an evaluation involving binary...
                    match operation_wrapper {
                        OperationWrapper::Assignment(op) => !matches!(
                            op,
                            AssignmentOperator::XorEqual(_)
                                | AssignmentOperator::OrEqual(_)
                                | AssignmentOperator::AndEqual(_),
                        ),
                        OperationWrapper::Evaluation(op) => !matches!(
                            op,
                            EvaluationOperator::And(_)
                                | EvaluationOperator::Or(_)
                                | EvaluationOperator::Xor(_)
                                | EvaluationOperator::BitShiftLeft(_)
                                | EvaluationOperator::BitShiftRight(_)
                        ),
                        OperationWrapper::Logical(_) => true,
                        OperationWrapper::Equality(_) => true,
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

enum OperationWrapper {
    Assignment(AssignmentOperator),
    Evaluation(EvaluationOperator),
    Logical(LogicalOperator),
    Equality(EqualityOperator),
}
