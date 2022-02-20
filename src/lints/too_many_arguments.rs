use crate::{parsing::expression::Expression, Duck, Lint, LintCategory, LintReport, Position};

#[derive(Debug, PartialEq)]
pub struct TooManyArguments;
impl Lint for TooManyArguments {
    fn generate_report(position: Position) -> LintReport {
        LintReport {
			tag: "too_many_arguments",
			display_name: "Too many arguments",
			explanation: "Functions with lots of parameters quickly become confusing and indicate a need for structural change.",
			suggestions: vec![
            "Split this into multiple functions",
            "Create a struct that holds the fields required by this function",
        ],
			category: LintCategory::Style,
			position,
		}
    }

    fn visit_expression(
        duck: &Duck,
        expression: &Expression,
        position: &Position,
        reports: &mut Vec<LintReport>,
    ) {
        if let Some(max) = duck.config().max_arguments() {
            if let Expression::FunctionDeclaration(_, params, ..) = expression {
                if params.len() > max {
                    reports.push(Self::generate_report(position.clone()))
                }
            }
        }
    }
}
