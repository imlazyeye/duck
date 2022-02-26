use crate::{
    lint::EarlyExpressionPass, parsing::expression::Expression, utils::Span, Lint, LintCategory,
    LintReport,
};

#[derive(Debug, PartialEq)]
pub struct RoomGoto;
impl Lint for RoomGoto {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
			display_name: "Use of `room_goto_*`".into(),
            tag: Self::tag(),
			explanation: "Projects that implement their own frameworks for room management may wish to be restrictive around when and where the `room_goto` functions are called.",
			suggestions: vec!["Replace this call with your API's ideal function".into()],
			category: Self::category(),
			span,
		}
    }

    fn category() -> LintCategory {
        LintCategory::Strict
    }

    fn tag() -> &'static str {
        "room_goto"
    }
}

impl EarlyExpressionPass for RoomGoto {
    fn visit_expression_early(
        _config: &crate::Config,
        expression: &Expression,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Expression::Call(caller, _, _) = expression {
            if let Expression::Identifier(name) = caller.expression() {
                if gm_room_goto_functions().contains(&name.as_str()) {
                    reports.push(Self::generate_report_with(
                        span,
                        format!("Use of `{}`", name),
                        [],
                    ))
                }
            }
        }
    }
}

fn gm_room_goto_functions() -> &'static [&'static str] {
    &["room_goto", "room_goto_next", "room_goto_previous"]
}
