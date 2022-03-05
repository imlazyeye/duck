use crate::{
    lint::{EarlyExpressionPass, EarlyStatementPass, Lint, LintLevel, LintReport},
    parse::{Access, Call, Expression, Globalvar, Span, Statement},
};

#[derive(Debug, PartialEq)]
pub struct Deprecated;
impl Lint for Deprecated {
    fn explanation() -> &'static str {
        "Deprecated features are liable to be removed at any time and should be avoided."
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
        if let Statement::GlobalvarDeclaration(Globalvar { name }) = statement {
            Self::report(
                "Use of globalvar",
                [
                    format!("Change this to the newer `global.{}` syntax.", name),
                    "Scope this variable to a singular object".into(),
                ],
                span,
                reports,
            );
        }
    }
}

impl EarlyExpressionPass for Deprecated {
    fn visit_expression_early(
        _config: &crate::Config,
        expression: &crate::parse::Expression,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Expression::Call(Call { left, .. }) = expression {
            if let Expression::Identifier(identifier) = left.expression() {
                if gm_deprecated_functions().contains(&identifier.name.as_str()) {
                    Self::report(
                        format!("Use of deprecated function: {}", identifier.name),
                        [],
                        span,
                        reports,
                    );
                }
            }
        } else if let Expression::Access(Access::Array { index_two: Some(_), .. }) = expression {
            Self::report(
                "Use of 2d array",
                ["Use chained arrays instead (`foo[0][0]).".into()],
                span,
                reports,
            );
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
