use crate::{
    lint::{EarlyExpressionPass, EarlyStatementPass, Lint, LintLevel, LintReport},
    parse::{Expression, Function, Span, Statement},
};
use heck::ToUpperCamelCase;

#[derive(Debug, PartialEq)]
pub struct NonPascalCase;
impl Lint for NonPascalCase {
    fn explanation() -> &'static str {
        " to distinguish them from other values."
    }

    fn default_level() -> LintLevel {
        LintLevel::Warn
    }

    fn tag() -> &'static str {
        "non_pascal_case"
    }
}

impl EarlyStatementPass for NonPascalCase {
    fn visit_statement_early(
        _config: &crate::Config,
        statement: &Statement,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Statement::EnumDeclaration(gml_enum) = statement {
            let name = &gml_enum.name;
            let ideal = pascal_case(name);
            if name != &ideal {
                Self::report(
                    format!("Enum should be PascalCase: {name}"),
                    [format!("Change this to `{}`", ideal)],
                    span,
                    reports,
                );
            }
            gml_enum.members.iter().map(|member| member.name()).for_each(|name| {
                let ideal = pascal_case(name);
                if name != ideal {
                    Self::report(
                        format!("Enum member should be PascalCase: {name}"),
                        [format!("Change this to `{}`", ideal)],
                        span,
                        reports,
                    );
                }
            });
        }
    }
}

impl EarlyExpressionPass for NonPascalCase {
    fn visit_expression_early(
        _config: &crate::Config,
        expression: &Expression,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Expression::FunctionDeclaration(Function {
            name: Some(name),
            constructor: Some(_),
            ..
        }) = expression
        {
            let ideal = pascal_case(name);
            if name != &ideal {
                Self::report(
                    "Constructor should be PascalCase: {name}",
                    [format!("Change this to `{}`", ideal)],
                    span,
                    reports,
                );
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
