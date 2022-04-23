use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyExprPass, Lint, LintLevel},
    parse::{Call, Expr, ExprKind},
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

impl EarlyExprPass for RoomGoto {
    fn visit_expr_early(expr: &Expr, config: &crate::Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let ExprKind::Call(Call { left, .. }) = expr.kind() {
            if let ExprKind::Identifier(identifier) = left.kind() {
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
