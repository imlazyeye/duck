use crate::{
    lint::{EarlyExpressionPass, Lint, LintLevel, LintReport},
    parsing::{Access, Expression},
    utils::Span,
};

#[derive(Debug, PartialEq)]
pub struct Global;
impl Lint for Global {
    fn explanation() -> &'static str {
        "While useful at times, global variables reduce saftey since they can be accessed or mutated anywhere, and provide no guarentee that they've already been initiailized."
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "global"
    }
}

impl EarlyExpressionPass for Global {
    fn visit_expression_early(
        _config: &crate::Config,
        expression: &Expression,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Expression::Access(Access::Global { .. }) = expression {
            Self::report(
                "Use of global variables",
                ["Scope this variable to an individual object".into()],
                span,
                reports,
            )
        }
    }
}
