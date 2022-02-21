use crate::{parsing::expression::Expression, Duck, Lint, LintCategory, LintReport, Span};

#[derive(Debug, PartialEq)]
pub struct TooManyArguments;
impl Lint for TooManyArguments {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
			tag: "too_many_arguments",
			display_name: "Too many arguments".into(),
			explanation: "Functions with lots of parameters quickly become confusing and indicate a need for structural change.",
			suggestions: vec![
            "Split this into multiple functions".into(),
            "Create a struct that holds the fields required by this function".into(),
        ],
			category: LintCategory::Style,
			span,
		}
    }

    fn visit_expression(
        duck: &Duck,
        expression: &Expression,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Some(max) = duck.config().max_arguments() {
            if let Expression::FunctionDeclaration(_, params, ..) = expression {
                if params.len() > max {
                    reports.push(Self::generate_report(span))
                }
            }
        }
    }
}
