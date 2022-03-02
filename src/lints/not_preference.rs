use crate::{
    lint::{EarlyExpressionPass, Lint, LintLevel, LintReport},
    prelude::{Expression, Token, Unary, UnaryOperator},
    utils::Span,
    Config,
};

#[derive(Debug, PartialEq)]
pub struct NotPreference;
impl Lint for NotPreference {
    fn explanation() -> &'static str {
        "GML supports both `not` and `!` to refer to unary \"not\". Consistent use of one over the other yields cleaner code."
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "or_preference"
    }
}
impl EarlyExpressionPass for NotPreference {
    fn visit_expression_early(config: &Config, expression: &Expression, span: Span, reports: &mut Vec<LintReport>) {
        if let Expression::Unary(Unary {
            operator: UnaryOperator::Not(token),
            ..
        }) = expression
        {
            if config.prefer_not_keyword() && token != &Token::Not {
                Self::report("Use of `!`", ["Use `not` instead of `!`".into()], span, reports);
            } else if token == &Token::Not {
                Self::report("Use of `not`", ["Use `!` instead of `not`".into()], span, reports);
            }
        }
    }
}
