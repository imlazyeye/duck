use crate::{
    lint::{EarlyExpressionPass, Lint, LintLevel, LintReport},
    parsing::{Expression, Function},
    utils::Span,
    Config,
};

#[derive(Debug, PartialEq)]
pub struct TooManyArguments;
impl Lint for TooManyArguments {
    fn explanation() -> &'static str {
        "Functions with lots of parameters quickly become confusing and indicate a need for structural change."
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
                Self::report(
                    "Too many arguments",
                    [
                        "Split this into multiple functions".into(),
                        "Create a struct that holds the fields required by this function".into(),
                    ],
                    span,
                    reports,
                )
            }
        }
    }
}
