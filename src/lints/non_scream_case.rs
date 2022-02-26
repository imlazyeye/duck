use heck::ToShoutySnakeCase;

use crate::{
    lint::EarlyStatementPass, parsing::statement::Statement, utils::Span, Lint, LintCategory,
    LintReport,
};

#[derive(Debug, PartialEq)]
pub struct NonScreamCase;
impl Lint for NonScreamCase {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
			display_name: "Identifier should be SCREAM_CASE".into(),
            tag: Self::tag(),
			explanation: "Scream case is the ideal casing for constants to distingusih them from other values.",
			suggestions: vec!["Change your casing to SCREAM_CASE".into()],
			category: Self::category(),
			span,
		}
    }

    fn category() -> LintCategory {
        LintCategory::Style
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
        if let Statement::MacroDeclaration(name, ..) = statement {
            let ideal = scream_case(name);
            if name != &ideal {
                reports.push(Self::generate_report_with(
                    span,
                    "Macro should be PascalCase",
                    [format!("Change this to `{}`", ideal)],
                ));
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
