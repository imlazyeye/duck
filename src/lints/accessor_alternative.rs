use crate::{
    lint::EarlyExpressionPass,
    parsing::{Expression, Literal},
    utils::Span,
    Lint, LintLevel, LintReport,
};

#[derive(Debug, PartialEq)]
pub struct AccessorAlternative;
impl Lint for AccessorAlternative {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
            display_name: "Use of function with accessor alternative".into(),
            tag: Self::tag(),
            explanation: "GML offers accessors as an alternative to many common functions which are preferable for their readability and brevity.",
            suggestions: vec!["Scope this variable to an individual object".into()],
            default_level: Self::default_level(),
            span,
        }
    }

    fn default_level() -> LintLevel {
        LintLevel::Warn
    }

    fn tag() -> &'static str {
        "accessor_alternative"
    }
}

impl EarlyExpressionPass for AccessorAlternative {
    fn visit_expression_early(
        _config: &crate::Config,
        expression: &Expression,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Expression::Call(caller, args, _) = expression {
            if let Expression::Identifier(identifier) = caller.expression() {
                match identifier.name.as_ref() {
                    "ds_list_find_value" => reports.push(Self::generate_report_with(
                        span,
                        "Use of `ds_list_find_value`",
                        ["Use an accessor (`foo[| bar]`) instead".into()],
                    )),
                    "ds_grid_get" => reports.push(Self::generate_report_with(
                        span,
                        "Use of `ds_grid_get`",
                        ["Use an accessor (`foo[# bar, buzz]`) instead".into()],
                    )),
                    "ds_map_find_value" => reports.push(Self::generate_report_with(
                        span,
                        "Use of `ds_map_find_value`",
                        ["Use an accessor (`foo[? bar]`) instead".into()],
                    )),
                    "array_get" => reports.push(Self::generate_report_with(
                        span,
                        "Use of `array_get`",
                        ["Use an accessor (`foo[bar]`) instead".into()],
                    )),
                    "variable_struct_get"
                        if args
                            .get(1)
                            .map(|v| matches!(v.expression(), &Expression::Literal(Literal::String(_))))
                            .is_some() =>
                    {
                        reports.push(Self::generate_report_with(
                            span,
                            "Use of `variable_struct_get` using string literal",
                            ["Use an dot notation (`foo.bar`) instead".into()],
                        ))
                    }
                    _ => {}
                }
            }
        }
    }
}
