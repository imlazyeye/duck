use crate::{
    Config,
    lint::{EarlyExpressionPass, Lint, LintLevel, LintReport},
    parse::{Evaluation, EvaluationOperator, Expression, Token},
    parse::Span,
};

#[derive(Debug, PartialEq)]
pub struct ModPreference;
impl Lint for ModPreference {
    fn explanation() -> &'static str {
        "GML supports both `mod` and `%` to perform modulo division. Consistent use of one over the other yields cleaner code."
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "mod_preference"
    }
}
impl EarlyExpressionPass for ModPreference {
    fn visit_expression_early(config: &Config, expression: &Expression, span: Span, reports: &mut Vec<LintReport>) {
        if let Expression::Evaluation(Evaluation {
            operator: EvaluationOperator::Modulo(token),
            ..
        }) = expression
        {
            if config.prefer_mod_keyword() && token != &Token::Mod {
                Self::report("Use of `%`", ["Use `mod` instead of `%`".into()], span, reports);
            } else if token == &Token::Mod {
                Self::report("Use of `mod`", ["Use `%` instead of `mod`".into()], span, reports);
            }
        }
    }
}
