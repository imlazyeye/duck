use crate::{parsing::statement::Statement, Duck, Lint, LintCategory, LintReport, Span};

#[derive(Debug, PartialEq)]
pub struct VarPrefixes;
impl Lint for VarPrefixes {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
			display_name: "Var Prefixes".into(),
			tag: "var_prefixes",
			explanation: "It is common practice in GML to prefix local variables (longer than one charcter) with an underscore as it helps to visually distinguish them from instance (or global) variables. You can select either option via the config.",
			suggestions: vec![],
			category: LintCategory::Style,
			span,
		}
    }

    fn visit_statement(
        duck: &Duck,
        statement: &Statement,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Statement::LocalVariableSeries(vars) = statement {
            for (name, _) in vars.iter() {
                if duck.config().var_prefixes && name.len() > 1 && !name.starts_with('_') {
                    reports.push(Self::generate_report_with(
                        span,
                        "Local variable without underscore prefix",
                        [format!("Change `{name}` to `_{name}`")],
                    ));
                } else if !duck.config().var_prefixes && name.starts_with('_') {
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
