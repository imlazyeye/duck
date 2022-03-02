use crate::{
    lint::{EarlyExpressionPass, Lint, LintLevel, LintReport},
    parsing::{Expression, Function},
    utils::Span,
};

#[derive(Debug, PartialEq)]
pub struct NonConstantDefaultParameter;
impl Lint for NonConstantDefaultParameter {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
            tag: Self::tag(),
            display_name: "Non constant default parameter".into(),
            explanation: "Expressive default parameters are not supported in most languages due to their instability and tendency to hide important logic execution from the caller.",
            suggestions: vec!["Create a seperated function for when this value is not provided".into()],
            default_level: Self::default_level(),
            span,
        }
    }

    fn default_level() -> LintLevel {
        LintLevel::Warn
    }

    fn tag() -> &'static str {
        "non_constant_default_parameter"
    }
}

impl EarlyExpressionPass for NonConstantDefaultParameter {
    fn visit_expression_early(
        _config: &crate::Config,
        expression: &Expression,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Expression::FunctionDeclaration(Function { parameters, .. }) = expression {
            for param in parameters {
                if let Some(expression) = &param.default_value {
                    if !matches!(
                        expression.expression(),
                        Expression::Identifier(_) | Expression::Literal(_),
                    ) {
                        reports.push(Self::generate_report(span))
                    }
                }
            }
        }
    }
}
