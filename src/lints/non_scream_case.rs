use heck::ToShoutySnakeCase;

use crate::{parsing::statement::Statement, Duck, Lint, LintCategory, LintReport, Position};

#[derive(Debug, PartialEq)]
pub struct NonScreamCase;
impl Lint for NonScreamCase {
    fn generate_report(position: Position) -> LintReport {
        LintReport {
			display_name: "Identifier should be SCREAM_CASE",
			tag: "non_scream_case",
			explanation: "Scream case is the ideal casing for constants to distingusih them from other values.",
			suggestions: vec!["Change your casing to SCREAM_CASE"],
			category: LintCategory::Style,
			position,
		}
    }

    fn visit_statement(
        _duck: &Duck,
        statement: &Statement,
        position: &Position,
        reports: &mut Vec<LintReport>,
    ) {
        if let Statement::MacroDeclaration(name, ..) = statement {
            if name != &scream_case(name) {
                reports.push(Self::generate_report(position.clone()))
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
