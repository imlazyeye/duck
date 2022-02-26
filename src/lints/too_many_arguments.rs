use crate::{
    lint::EarlyExpressionPass, parsing::expression::Expression, utils::Span, Config, Duck, Lint,
    LintCategory, LintReport,
};

#[derive(Debug, PartialEq)]
pub struct TooManyArguments;
impl Lint for TooManyArguments {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
            tag: Self::tag(),
			display_name: "Too many arguments".into(),
			explanation: "Functions with lots of parameters quickly become confusing and indicate a need for structural change.",
			suggestions: vec![
            "Split this into multiple functions".into(),
            "Create a struct that holds the fields required by this function".into(),
        ],
			category: Self::category(),
			span,
		}
    }

    fn category() -> LintCategory {
        LintCategory::Style
    }

    fn tag() -> &'static str {
        "too_many_arguments"
    }
}

impl EarlyExpressionPass for TooManyArguments {
    fn visit_expression_early(
        config: &Config,
        expression: &Expression,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Some(max) = config.max_arguments() {
            if let Expression::FunctionDeclaration(_, params, ..) = expression {
                if params.len() > max {
                    reports.push(Self::generate_report(span))
                }
            }
        }
    }
}
