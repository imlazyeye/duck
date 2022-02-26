use crate::{
    lint::EarlyStatementPass, parsing::statement::Statement, utils::Span, Config, Lint,
    LintCategory, LintReport,
};

#[derive(Debug, PartialEq)]
pub struct VarPrefixViolation;
impl Lint for VarPrefixViolation {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
			display_name: "Var Prefix Violation".into(),
            tag: Self::tag(),
			explanation: "It is common practice in GML to prefix local variables (longer than one charcter) with an underscore as it helps to visually distinguish them from instance (or global) variables. You can select either option via the config.",
			suggestions: vec![],
			category: Self::category(),
			span,
		}
    }

    fn category() -> LintCategory {
        LintCategory::Style
    }

    fn tag() -> &'static str {
        "var_prefix_violation"
    }
}

impl EarlyStatementPass for VarPrefixViolation {
    fn visit_statement_early(
        config: &Config,
        statement: &Statement,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Statement::LocalVariableSeries(vars) = statement {
            for (name, _) in vars.iter() {
                if config.var_prefixes && name.len() > 1 && !name.starts_with('_') {
                    reports.push(Self::generate_report_with(
                        span,
                        "Local variable without underscore prefix",
                        [format!("Change `{name}` to `_{name}`")],
                    ));
                } else if !config.var_prefixes && name.starts_with('_') {
                    reports.push(Self::generate_report_with(
                        span,
                        "Local variable with underscore prefix",
                        [format!("Change `{name}` to `{}`", &name[1..])],
                    ));
                }
            }
        }
    }
}
