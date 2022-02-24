use heck::ToUpperCamelCase;

use crate::{
    lint::{EarlyExpressionPass, EarlyStatementPass},
    parsing::{expression::Expression, statement::Statement},
    utils::Span,
    Duck, Lint, LintCategory, LintReport,
};

#[derive(Debug, PartialEq)]
pub struct NonPascalCase;
impl Lint for NonPascalCase {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
			display_name: "Identifier should be PascalCase".into(),
            tag: Self::tag(),
			explanation: "Pascal case is the ideal casing for \"types\" to distinguish them from other values.",
			suggestions: vec![],
			category: Self::category(),
			span,
		}
    }

    fn category() -> LintCategory {
        LintCategory::Style
    }

    fn tag() -> &'static str {
        "non_pascal_case"
    }
}

impl EarlyStatementPass for NonPascalCase {
    fn visit_statement_early(
        _duck: &Duck,
        statement: &Statement,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Statement::EnumDeclaration(gml_enum) = statement {
            let name = gml_enum.name();
            let ideal = pascal_case(name);
            if name != ideal {
                reports.push(Self::generate_report_with(
                    span,
                    "Enum should be PascalCase",
                    [format!("Change this to `{}`", ideal)],
                ));
            }
            gml_enum
                .members()
                .iter()
                .map(|member| member.name())
                .for_each(|name| {
                    if name != pascal_case(name) {
                        reports.push(Self::generate_report_with(
                            span,
                            "Enum member should be PascalCase",
                            [format!("Change this to `{}`", ideal)],
                        ));
                    }
                });
        }
    }
}

impl EarlyExpressionPass for NonPascalCase {
    fn visit_expression_early(
        _duck: &Duck,
        expression: &Expression,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Expression::FunctionDeclaration(Some(name), _, Some(_), ..) = expression {
            let ideal = pascal_case(name);
            if name != &ideal {
                reports.push(Self::generate_report_with(
                    span,
                    "Constructor should be PascalCase",
                    [format!("Change this to `{}`", ideal)],
                ));
            }
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
