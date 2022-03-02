use crate::{
    lint::{EarlyExpressionPass, Lint, LintLevel, LintReport},
    parsing::{Expression, Function},
    utils::Span,
    Config,
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
            default_level: Self::default_level(),
            span,
        }
    }

    fn default_level() -> LintLevel {
        LintLevel::Warn
    }

    fn tag() -> &'static str {
        "too_many_arguments"
    }
}

impl EarlyExpressionPass for TooManyArguments {
    fn visit_expression_early(config: &Config, expression: &Expression, span: Span, reports: &mut Vec<LintReport>) {
        if let Expression::FunctionDeclaration(Function { parameters, .. }) = expression {
            if parameters.len() > config.max_arguments {
                reports.push(Self::generate_report(span))
            }
        }
    }
}
