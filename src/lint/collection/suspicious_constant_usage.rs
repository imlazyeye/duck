use crate::{
    lint::{EarlyExpressionPass, EarlyStatementPass, Lint, LintLevel, LintReport},
    parse::{Assignment, AssignmentOperator, Evaluation, EvaluationOperator, Expression, Literal, Span, Statement},
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
        _config: &crate::Config,
        expression: &Expression,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Expression::Evaluation(Evaluation { operator, right, .. }) = expression {
            if let Some(literal) = right.expression().as_literal() {
                if literal_is_suspicous(literal, OperationWrapper::Evaluation(*operator)) {
                    Self::report("Suspicious constant usage", [], span, reports)
                }
            }
        }
    }
}
impl EarlyStatementPass for SuspicousConstantUsage {
    fn visit_statement_early(
        _config: &crate::Config,
        statement: &Statement,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Statement::Assignment(Assignment { operator, right, .. }) = statement {
            if !matches!(
                *operator,
                AssignmentOperator::Equal(_) | AssignmentOperator::NullCoalecenceEqual(_)
            ) {
                if let Some(literal) = right.expression().as_literal() {
                    if literal_is_suspicous(literal, OperationWrapper::Assignment(*operator)) {
                        Self::report("Suspicious constant usage", [], span, reports)
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
