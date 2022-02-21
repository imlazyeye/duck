use crate::{
    parsing::expression::{AccessScope, Expression},
    Duck, Lint, LintCategory, LintReport, Span,
};

#[derive(Debug, PartialEq)]
pub struct Global;
impl Lint for Global {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
			display_name: "Use of `global`",
			tag: "global",
			explanation: "While useful at times, global variables reduce saftey since they can be accessed or mutated anywhere.",
			suggestions: vec!["Scope this variable to an individual object"],
			category: LintCategory::Pedantic,
			span,
		}
    }

    fn visit_expression(
        _duck: &Duck,
        expression: &Expression,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Expression::Access(_, AccessScope::Global) = expression {
            reports.push(Self::generate_report(span))
        }
    }
}
