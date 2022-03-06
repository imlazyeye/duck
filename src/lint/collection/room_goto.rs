use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyExpressionPass, Lint, LintLevel},
    parse::{Call, Expression, ExpressionBox},
    FileId,
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
        expression_box: &ExpressionBox,
        config: &crate::Config,
        reports: &mut Vec<Diagnostic<FileId>>,
    ) {
        if let Expression::Call(Call { left, .. }) = expression_box.expression() {
            if let Expression::Identifier(identifier) = left.expression() {
                if gm_room_goto_functions().contains(&identifier.lexeme.as_str()) {
                    reports.push(
                        Self::diagnostic(config)
                            .with_message(format!("Use of `{}`", identifier.lexeme))
                            .with_labels(vec![
                                Label::primary(left.file_id(), left.span())
                                    .with_message("replace this call with your API's ideal function"),
                            ]),
                    );
                }
            }
        }
    }
}

fn gm_room_goto_functions() -> &'static [&'static str] {
    &["room_goto", "room_goto_next", "room_goto_previous"]
}
