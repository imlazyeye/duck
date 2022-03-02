use crate::{
    lint::{EarlyExpressionPass, Lint, LintLevel, LintReport},
    parsing::{Expression, Logical, LogicalOperator, Token},
    utils::Span,
    Config,
};

#[derive(Debug, PartialEq)]
pub struct AndPreference;
impl Lint for AndPreference {
    fn explanation() -> &'static str {
        "GML supports both `and` and `&&` to refer to logical \"and\". Consistent use of one over the other yields cleaner code."
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "and_preference"
    }
}
impl EarlyExpressionPass for AndPreference {
    fn visit_expression_early(config: &Config, expression: &Expression, span: Span, reports: &mut Vec<LintReport>) {
        if let Expression::Logical(Logical {
            operator: LogicalOperator::And(token),
            ..
        }) = expression
        {
            if config.prefer_and_keyword() && token != &Token::And {
                Self::report("Use of `&&`", ["Use `and` instead of `&&`".into()], span, reports);
            } else if token == &Token::And {
                Self::report("Use of `and`", ["Use `&&` instead of `and`".into()], span, reports);
            }
        }
    }
}
