use crate::{
    lint::{EarlyExpressionPass, Lint, LintLevel, LintReport},
    parse::{Call, Expression, Span},
};

#[derive(Debug, PartialEq)]
pub struct RoomGoto;
impl Lint for RoomGoto {
    fn explanation() -> &'static str {
        "Projects that implement their own frameworks for room management may wish to be restrictive around when and where the `room_goto` functions are called."
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
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
        if let Expression::Call(Call { left, .. }) = expression {
            if let Expression::Identifier(identifier) = left.expression() {
                if gm_room_goto_functions().contains(&identifier.name.as_str()) {
                    Self::report(
                        format!("Use of `{}`", identifier.name),
                        ["Replace this call with your API's ideal function".into()],
                        span,
                        reports,
                    )
                }
            }
        }
    }
}

fn gm_room_goto_functions() -> &'static [&'static str] {
    &["room_goto", "room_goto_next", "room_goto_previous"]
}
