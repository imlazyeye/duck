use crate::{
    Config,
    lint::{EarlyStatementPass, Lint, LintLevel, LintReport},
    parse::{If, Statement},
    parse::Span,
};

#[derive(Debug, PartialEq)]
pub struct CollapsableIf;
impl Lint for CollapsableIf {
    fn explanation() -> &'static str {
        "If statements that contain nothing more than another if statement can be collapsed into a single statement."
    }

    fn default_level() -> LintLevel {
        LintLevel::Warn
    }

    fn tag() -> &'static str {
        "collapsable_if"
    }
}

impl EarlyStatementPass for CollapsableIf {
    fn visit_statement_early(_config: &Config, statement: &Statement, span: Span, reports: &mut Vec<LintReport>) {
        if let Statement::If(If { body, .. }) = statement {
            if let Some(block) = body.statement().as_block().filter(|block| block.body.len() == 1) {
                if let Statement::If(If {
                    else_statement: None, ..
                }) = block.body.first().unwrap().statement()
                {
                    Self::report(
                        "Collapsabe if statement",
                        ["Combine these if statements into one".into()],
                        span,
                        reports,
                    );
                }
            }
        }
    }
}
