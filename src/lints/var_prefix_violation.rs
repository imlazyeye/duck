use crate::{
    gml::LocalVariableSeries, lint::EarlyStatementPass, parsing::Statement, utils::Span, Config, Lint, LintLevel,
    LintReport,
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
            default_level: Self::default_level(),
            span,
        }
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "var_prefix_violation"
    }
}

impl EarlyStatementPass for VarPrefixViolation {
    fn visit_statement_early(config: &Config, statement: &Statement, span: Span, reports: &mut Vec<LintReport>) {
        if let Statement::LocalVariableSeries(LocalVariableSeries { declarations }) = statement {
            for local_variable in declarations.iter() {
                let name = local_variable.name();
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
