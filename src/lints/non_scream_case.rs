use crate::{
    lint::{EarlyStatementPass, Lint, LintLevel, LintReport},
    parsing::{Macro, Statement},
    utils::Span,
};
use heck::ToShoutySnakeCase;

#[derive(Debug, PartialEq)]
pub struct NonScreamCase;
impl Lint for NonScreamCase {
    fn explanation() -> &'static str {
        "Scream case is the ideal casing for constants to distingusih them from other values."
    }

    fn default_level() -> LintLevel {
        LintLevel::Warn
    }

    fn tag() -> &'static str {
        "non_scream_case"
    }
}

impl EarlyStatementPass for NonScreamCase {
    fn visit_statement_early(
        _config: &crate::Config,
        statement: &Statement,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Statement::MacroDeclaration(Macro { name, .. }) = statement {
            let ideal = scream_case(name);
            if name != &ideal {
                Self::report(
                    format!("Macro should be SCREAM_CASE: {name}"),
                    [format!("Change this to `{}`", ideal)],
                    span,
                    reports,
                );
            }
        }
    }
}

/// Returns the given string under Duck's definition of SCREAM_CASE.
fn scream_case(string: &str) -> String {
    let output = string.to_shouty_snake_case();
    let mut prefix = String::new();
    let mut chars = string.chars();
    while let Some('_') = chars.next() {
        prefix.push('_');
    }
    prefix + &output
}
