use crate::{
    lint::{EarlyExpressionPass, EarlyStatementPass},
    parsing::{Expression, Scope, Statement},
    utils::Span,
    Lint, LintLevel, LintReport,
};

#[derive(Debug, PartialEq)]
pub struct Deprecated;
impl Lint for Deprecated {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
            tag: Self::tag(),
            display_name: "Use of deprecated feature".into(),
            explanation: "Deprecated features are liable to be removed at any time and should be avoided.",
            suggestions: vec![],
            default_level: Self::default_level(),
            span,
        }
    }

    fn default_level() -> LintLevel {
        LintLevel::Warn
    }

    fn tag() -> &'static str {
        "deprecated"
    }
}

impl EarlyStatementPass for Deprecated {
    fn visit_statement_early(
        _config: &crate::Config,
        statement: &Statement,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Statement::GlobalvarDeclaration(name) = statement {
            reports.push(Self::generate_report_with(
                span,
                "Use of globalvar",
                [
                    format!("Change this to the newer `global.{}` syntax.", name),
                    "Scope this variable to a singular object".into(),
                ],
            ));
        }
    }
}

impl EarlyExpressionPass for Deprecated {
    fn visit_expression_early(
        _config: &crate::Config,
        expression: &crate::parsing::Expression,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Expression::Call(caller, _, _) = expression {
            if let Expression::Identifier(name) = caller.expression() {
                if gm_deprecated_functions().contains(&name.as_str()) {
                    reports.push(Self::generate_report_with(
                        span,
                        format!("Use of deprecated function: {}", name),
                        [],
                    ));
                }
            }
        } else if let Expression::Access(Scope::Array(_, Some(_), ..), _) = expression {
            reports.push(Self::generate_report_with(
                span,
                "Use of 2d array",
                ["Use chained arrays instead (`foo[0][0]).".into()],
            ));
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
