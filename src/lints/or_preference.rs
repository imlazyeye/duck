use crate::{
    lint::{EarlyExpressionPass, Lint, LintLevel, LintReport},
    prelude::{Expression, Logical, LogicalOperator, Token},
    utils::Span,
    Config,
};

#[derive(Debug, PartialEq)]
pub struct OrPreference;
impl Lint for OrPreference {
    fn explanation() -> &'static str {
        "GML supports both `or` and `||` to refer to logical \"or\" -- `||` is more consistent with other languages and is preferred."
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "or_preference"
    }
}
impl EarlyExpressionPass for OrPreference {
    fn visit_expression_early(config: &Config, expression: &Expression, span: Span, reports: &mut Vec<LintReport>) {
        if let Expression::Logical(Logical {
            operator: LogicalOperator::Or(token),
            ..
        }) = expression
        {
            if config.prefer_or_keyword() && token != &Token::Or {
                Self::report("Use of `||`", ["Use `or` instead of `||`".into()], span, reports);
            } else if token == &Token::Or {
                Self::report("Use of `or`", ["Use `||` instead of `or`".into()], span, reports);
            }
        }
    }
}
