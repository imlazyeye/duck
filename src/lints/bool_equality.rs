use crate::{
    lint::{EarlyExpressionPass, Lint, LintLevel, LintReport},
    parsing::{Equality, EqualityOperator, Expression, Literal},
    utils::Span,
};

#[derive(Debug, PartialEq)]
pub struct BoolEquality;
impl Lint for BoolEquality {
    fn explanation() -> &'static str {
        "Comparing a bool with a bool literal is more verbose than neccesary."
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "bool_equality"
    }
}

impl EarlyExpressionPass for BoolEquality {
    fn visit_expression_early(
        _config: &crate::Config,
        expression: &Expression,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Expression::Equality(Equality {
            operator: EqualityOperator::Equal(_),
            right,
            ..
        }) = expression
        {
            if let Some(literal) = right.expression().as_literal() {
                match literal {
                    Literal::True => Self::report(
                        "Equality check with `true`",
                        ["Remove the `== true`".into()],
                        span,
                        reports,
                    ),
                    Literal::False => Self::report(
                        "Equality check with `false`",
                        ["Remove the `== false` and se `!foo` syntax instead".into()],
                        span,
                        reports,
                    ),
                    _ => {}
                }
            }
        }
    }
}
