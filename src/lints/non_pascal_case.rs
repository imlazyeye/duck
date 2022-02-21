use heck::ToUpperCamelCase;

use crate::{
    parsing::{expression::Expression, statement::Statement},
    Duck, Lint, LintCategory, LintReport, Span,
};

#[derive(Debug, PartialEq)]
pub struct NonPascalCase;
impl Lint for NonPascalCase {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
			display_name: "Identifier should be PascalCase",
			tag: "non_pascal_case",
			explanation: "Pascal case is the ideal casing for \"types\" to distinguish them from other values.",
			suggestions: vec!["Change your casing to PascalCase"],
			category: LintCategory::Style,
			span,
		}
    }

    fn visit_expression(
        _duck: &Duck,
        expression: &Expression,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Expression::FunctionDeclaration(Some(name), _, Some(_), ..) = expression {
            if name != &pascal_case(name) {
                reports.push(Self::generate_report(span))
            }
        }
    }

    fn visit_statement(
        _duck: &Duck,
        statement: &Statement,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Statement::EnumDeclaration(name, members) = statement {
            if name != &pascal_case(name) {
                reports.push(Self::generate_report(span))
            }
            members.iter().map(|(n, _)| n).for_each(|name| {
                if name != &pascal_case(name) {
                    reports.push(Self::generate_report(span))
                }
            });
        }
    }
}

/// Returns the given string under Duck's definition of PascalCase.
fn pascal_case(string: &str) -> String {
    let output = string.to_upper_camel_case();
    let mut prefix = String::new();
    let mut chars = string.chars();
    while let Some('_') = chars.next() {
        prefix.push('_');
    }
    prefix + &output
}
