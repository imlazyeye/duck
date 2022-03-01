use crate::{gml::Access, lint::EarlyExpressionPass, parsing::Expression, utils::Span, Lint, LintLevel, LintReport};

#[derive(Debug, PartialEq)]
pub struct Global;
impl Lint for Global {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
            display_name: "Use of `global`".into(),
            tag: Self::tag(),
            explanation: "While useful at times, global variables reduce saftey since they can be accessed or mutated anywhere.",
            suggestions: vec!["Scope this variable to an individual object".into()],
            default_level: Self::default_level(),
            span,
        }
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
        if let Expression::Access(Access::Global { right }) = expression {
            reports.push(Self::generate_report(span))
        }
    }
}
