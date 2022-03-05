use crate::{
    lint::{EarlyStatementPass, Lint, LintLevel, LintReport},
    parse::{LocalVariableSeries, Statement},
    parse::Span,
};

#[derive(Debug, PartialEq)]
pub struct MultiVarDeclaration;
impl Lint for MultiVarDeclaration {
    fn explanation() -> &'static str {
        "While GML allows you to create multiple local variables at once, it can often lead to confusing syntax that would read better with each variable seperated."
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
                Self::report(
                    "Multiple local variables declared at once",
                    ["Break these down into seperate declarations".into()],
                    span,
                    reports,
                );
            }
        }
    }
}
