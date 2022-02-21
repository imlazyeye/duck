use crate::{
    parsing::{
        expression::{AccessScope, Expression},
        statement::Statement,
    },
    Duck, Lint, LintCategory, LintReport, Span,
};

#[derive(Debug, PartialEq)]
pub struct Deprecated;
impl Lint for Deprecated {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
            tag: "deprecated",
            display_name: "Use of deprecated feature",
            explanation:
                "Deprecated features are liable to be removed at any time and should be avoided.",
            suggestions: vec![],
            category: LintCategory::Correctness,
            span,
        }
    }

    fn visit_statement(
        _duck: &Duck,
        statement: &Statement,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Statement::GlobalvarDeclaration(..) = statement {
            reports.push(Self::generate_report(span))
        }
    }

    fn visit_expression(
        _duck: &Duck,
        expression: &crate::parsing::expression::Expression,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Expression::Call(caller, _, _) = expression {
            if let Expression::Identifier(name) = caller.expression() {
                if gm_deprecated_functions().contains(&name.as_str()) {
                    reports.push(Self::generate_report(span))
                }
            }
        } else if let Expression::Access(_, AccessScope::Array(_, Some(_), ..)) = expression {
            reports.push(Self::generate_report(span))
        }
    }
}

fn gm_deprecated_functions() -> &'static [&'static str] {
    &[
        "array_height_2d",
        "array_length_2d",
        "array_length_2d",
        "buffer_surface_copy",
    ]
}
