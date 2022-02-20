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
}
