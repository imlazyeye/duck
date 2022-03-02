use crate::{
    lint::{EarlyStatementPass, Lint, LintLevel, LintReport},
    parsing::{LocalVariableSeries, Statement},
    utils::Span,
};

#[derive(Debug, PartialEq)]
pub struct MultiVarDeclaration;
impl Lint for MultiVarDeclaration {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
            display_name: "Multiple local variables declared at once".into(),
            tag: Self::tag(),
            explanation: "While GML allows you to create multiple local variables at once, it can often lead to confusing syntax that would read better with each variable seperated.",
            suggestions: vec!["Break these down into seperate declarations".into()],
            default_level: Self::default_level(),
            span,
        }
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "multi_var_declaration"
    }
}

impl EarlyStatementPass for MultiVarDeclaration {
    fn visit_statement_early(
        _config: &crate::Config,
        statement: &Statement,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Statement::LocalVariableSeries(LocalVariableSeries { declarations }) = statement {
            if declarations.len() > 1 {
                reports.push(Self::generate_report(span));
            }
        }
    }
}
