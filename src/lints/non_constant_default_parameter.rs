use crate::{parsing::expression::Expression, Duck, Lint, LintCategory, LintReport, Span};

#[derive(Debug, PartialEq)]
pub struct NonConstantDefaultParameter;
impl Lint for NonConstantDefaultParameter {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
            tag: Self::tag(),
			display_name: "Non constant default parameter".into(),
			explanation: "Expressive default parameters are not supported in most languages due to their instability and tendency to hide important logic execution from the caller.",
			suggestions: vec![
                "Create a seperated function for when this value is not provided".into(),
            ],
			category: Self::category(),
			span,
		}
    }

    fn category() -> LintCategory {
        LintCategory::Pedantic
    }

    fn tag() -> &'static str {
        "non_constant_default_parameter"
    }

    fn visit_expression(
        _duck: &Duck,
        expression: &Expression,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Expression::FunctionDeclaration(_, params, ..) = expression {
            for param in params {
                if let Some(expression) = &param.1 {
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
