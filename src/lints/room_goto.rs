use crate::{Lint, LintCategory};

pub struct RoomGoto;
impl Lint for RoomGoto {
    fn tag() -> &'static str {
        "room_goto"
    }

    fn display_name() -> &'static str {
        "Use of `room_goto_*`"
    }

    fn explanation() -> &'static str {
        "Projects that implement their own frameworks for room management may wish to be restrictive around when and where the `room_goto` functions are called."
    }

    fn suggestions() -> Vec<&'static str> {
        vec!["Replace this call with your API's ideal function"]
    }

    fn category() -> LintCategory {
        LintCategory::Pedantic
    }
}
