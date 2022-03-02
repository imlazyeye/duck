use crate::{
    lint::{EarlyExpressionPass, Lint, LintLevel, LintReport},
    parsing::{Call, Expression, Literal},
    utils::Span,
    Config,
};

#[derive(Debug, PartialEq)]
pub struct AccessorAlternative;
impl Lint for AccessorAlternative {
    fn explanation() -> &'static str {
        "GML offers accessors as an alternative to many common functions which are preferable for their readability and brevity."
    }

    fn default_level() -> LintLevel {
        LintLevel::Warn
    }

    fn tag() -> &'static str {
        "accessor_alternative"
    }
}

impl EarlyExpressionPass for AccessorAlternative {
    fn visit_expression_early(_config: &Config, expression: &Expression, span: Span, reports: &mut Vec<LintReport>) {
        if let Expression::Call(Call { left, arguments, .. }) = expression {
            if let Expression::Identifier(identifier) = left.expression() {
                match identifier.name.as_ref() {
                    "ds_list_find_value" => Self::report(
                        "Use of `ds_list_find_value`",
                        ["Use an accessor (`foo[| bar]`) instead".into()],
                        span,
                        reports,
                    ),
                    "ds_grid_get" => Self::report(
                        "Use of `ds_grid_get`",
                        ["Use an accessor (`foo[# bar, buzz]`) instead".into()],
                        span,
                        reports,
                    ),
                    "ds_map_find_value" => Self::report(
                        "Use of `ds_map_find_value`",
                        ["Use an accessor (`foo[? bar]`) instead".into()],
                        span,
                        reports,
                    ),
                    "array_get" => Self::report(
                        "Use of `array_get`",
                        ["Use an accessor (`foo[bar]`) instead".into()],
                        span,
                        reports,
                    ),
                    "variable_struct_get"
                        if arguments
                            .get(1)
                            .map(|v| matches!(v.expression(), &Expression::Literal(Literal::String(_))))
                            == Some(true) =>
                    {
                        Self::report(
                            "Use of `variable_struct_get` using string literal",
                            ["Use an dot notation (`foo.bar`) instead".into()],
                            span,
                            reports,
                        )
                    }
                    _ => {}
                }
            }
        }
    }
}
