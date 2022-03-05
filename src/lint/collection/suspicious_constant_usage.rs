use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyExpressionPass, EarlyStatementPass, Lint, LintLevel},
    parse::{
        Assignment, AssignmentOperator, Evaluation, EvaluationOperator, Expression, ExpressionBox, Literal, Statement,
        StatementBox,
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

impl EarlyExpressionPass for SuspicousConstantUsage {
    fn visit_expression_early(
        expression_box: &ExpressionBox,
        config: &crate::Config,
        reports: &mut Vec<Diagnostic<FileId>>,
    ) {
        if let Expression::Evaluation(Evaluation { operator, right, .. }) = expression_box.expression() {
            if let Some(literal) = right.expression().as_literal() {
                if literal_is_suspicous(literal, OperationWrapper::Evaluation(*operator)) {
                    reports.push(
                        Self::diagnostic(config)
                            .with_message("Suspicious constant usage")
                            .with_labels(vec![
                                Label::primary(right.file_id(), right.span()).with_message("using this operator..."),
                                Label::primary(right.file_id(), right.span())
                                    .with_message("...with this literal, which is not a coherent operation"),
                            ]),
                    );
                }
            }
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
                        reports.push(
                            Self::diagnostic(config)
                                .with_message("Suspicious constant usage")
                                .with_labels(vec![
                                    Label::primary(right.file_id(), right.span())
                                        .with_message("using this operator..."),
                                    Label::primary(right.file_id(), right.span())
                                        .with_message("...with this literal, which is not a coherent operation"),
                                ]),
                        );
                    }
                }
            }
        }
    }
}

fn literal_is_suspicous(literal: &Literal, operation_wrapper: OperationWrapper) -> bool {
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
                "tile_index_mask" => {
                    // This is intended for bit masking, so only allow it if its an evaluation involving binary...
                    if let OperationWrapper::Evaluation(op) = operation_wrapper {
                        !matches!(
                            op,
                            EvaluationOperator::And(_)
                                | EvaluationOperator::Or(_)
                                | EvaluationOperator::Xor(_)
                                | EvaluationOperator::BitShiftLeft(_)
                                | EvaluationOperator::BitShiftRight(_)
                        )
                    } else {
                        true
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
}
