use crate::{
    lint::{EarlyExpressionPass, Lint, LintLevel, LintReport},
    parse::{Expression, Function, Span},
    Config,
};

#[derive(Debug, PartialEq)]
pub struct AnonymousConstructor;
impl Lint for AnonymousConstructor {
    fn explanation() -> &'static str {
        "Constructors should be reserved for larger, higher scoped types."
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "anonymous_constructor"
    }
}

impl EarlyExpressionPass for AnonymousConstructor {
    fn visit_expression_early(_config: &Config, expression: &Expression, span: Span, reports: &mut Vec<LintReport>) {
        if let Expression::FunctionDeclaration(Function {
            name: None,
            constructor: Some(_),
            ..
        }) = expression
        {
            Self::report(
                "Use of an anonymous constructor",
                [
                    "Change this to a named function".into(),
                    "Change this to a function that returns a struct literal".into(),
                ],
                span,
                reports,
            )
        }
    }
}
