use crate::{
    parsing::expression::{Expression, Literal},
    Duck, Lint, LintCategory, LintReport, Span,
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
			category: Self::category(),
			span,
		}
    }

    fn category() -> LintCategory {
        LintCategory::Style
    }

    fn tag() -> &'static str {
        "accessor_alternative"
    }

    fn visit_expression(
        _duck: &Duck,
        expression: &Expression,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Expression::Call(caller, args, _) = expression {
            if let Expression::Identifier(name) = caller.expression() {
                match name.as_ref() {
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
                            .map(|v| {
                                matches!(v.expression(), &Expression::Literal(Literal::String(_)))
                            })
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
