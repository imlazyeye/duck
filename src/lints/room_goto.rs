use crate::{parsing::expression::Expression, Duck, Lint, LintCategory, LintReport, Position};

#[derive(Debug, PartialEq)]
pub struct RoomGoto;
impl Lint for RoomGoto {
    fn generate_report(position: Position) -> LintReport {
        LintReport {
			display_name: "Use of `room_goto_*`",
			tag: "room_goto",
			explanation: "Projects that implement their own frameworks for room management may wish to be restrictive around when and where the `room_goto` functions are called.",
			suggestions: vec!["Replace this call with your API's ideal function"],
			category: LintCategory::Pedantic,
			position,
		}
    }

    fn visit_expression(
        _duck: &Duck,
        expression: &Expression,
        position: &Position,
        reports: &mut Vec<LintReport>,
    ) {
        if let Expression::Call(caller, _, _) = expression {
            if let Expression::Identifier(name) = caller.expression() {
                if gm_room_goto_functions().contains(&name.as_str()) {
                    reports.push(Self::generate_report(position.clone()))
                }
            }
        }
    }
}

fn gm_room_goto_functions() -> &'static [&'static str] {
    &["room_goto", "room_goto_next", "room_goto_previous"]
}
