use crate::{Duck, Position};

mod room_goto;
pub use room_goto::RoomGoto;
mod no_space_begining_comment;
pub use no_space_begining_comment::NoSpaceBeginingComment;
mod anonymous_constructor;
pub use anonymous_constructor::AnonymousConstructor;
mod and_keyword;
pub use and_keyword::AndKeyword;
mod constructor_without_new;
pub use constructor_without_new::ConstructorWithoutNew;
mod draw_sprite;
pub use draw_sprite::DrawSprite;
mod draw_text;
pub use draw_text::DrawText;
mod exit;
pub use exit::Exit;
mod global;
pub use global::Global;
mod globalvar;
pub use globalvar::Globalvar;
mod missing_case_members;
pub use missing_case_members::MissingCaseMember;
mod missing_default_case;
pub use missing_default_case::MissingDefaultCase;
mod mod_keyword;
pub use mod_keyword::ModKeyword;
mod non_pascal_case;
pub use non_pascal_case::NonPascalCase;
mod non_scream_case;
pub use non_scream_case::NonScreamCase;
mod or_keyword;
pub use or_keyword::OrKeyword;
mod show_debug_message;
pub use show_debug_message::ShowDebugMessage;
mod single_switch_case;
pub use single_switch_case::SingleSwitchCase;
mod todo;
pub use todo::Todo;
mod too_many_arguments;
pub use too_many_arguments::TooManyArguments;
mod too_many_lines;
pub use too_many_lines::TooManyLines;
mod try_catch;
pub use try_catch::TryCatch;
mod with_loop;
pub use with_loop::WithLoop;

/// An individual lint in duck.
///
/// Lints should be named after the *bad* action, not the good one. For example,
/// a lint that prevents switch statements from having no default case should be
/// called `MissingDefaultCase`, not, say, `DefaultCaseInSwitch`. This makes tagging
/// read more clearly (ie: `#[allow(missing_default_case)])`).
pub trait Lint {
    /// The string representation of this lint used for referencing it in code.
    /// For example, the lint `"MissingDefaultCase"` should return a string like
    /// `"missing_default_case"`.
    fn tag() -> &'static str;

    /// The title of the lint as displayed when it fires into the output.
    fn display_name() -> &'static str;

    /// A justification for this lint, expressing why it may be desirable to enable.
    fn explanation() -> &'static str;

    /// A collection of suggestions on how to avoid this lint that will be displayed to the user
    /// when this lint fires.
    fn suggestions() -> Vec<&'static str>;

    /// The [LintCategory] this lint belongs to.
    fn category() -> LintCategory;

    /// The execution of this lint, returning any discoveries through LintReports.
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    fn run(duck: &Duck) -> Vec<LintReport> {
        let mut reports = vec![];
        reports
    }
}

/// The three different levels a lint can be set to, changing how it will be treated.
#[derive(Debug, Copy, Clone, enum_map::Enum, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LintLevel {
    /// These lints will be ran, but their results will not affect the outcome of duck.
    Allow,
    /// These lints will be reported to the user, but will not fail the run by default.
    Warn,
    /// These lints will be reported to the user and will fail the run.
    Deny,
}
impl LintLevel {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "allow" => Some(Self::Allow),
            "warn" => Some(Self::Warn),
            "deny" => Some(Self::Deny),
            _ => None,
        }
    }
    pub fn to_str(&self) -> &str {
        match self {
            LintLevel::Allow => "allow",
            LintLevel::Warn => "warn",
            LintLevel::Deny => "deny",
        }
    }
}

/// The data from a user-written tag (ie: #[allow(draw_text)])
#[derive(Debug)]
pub struct LintTag(pub String, pub LintLevel);

/// The category a lint falls into. This effects duck's default permission level for all lints.
#[derive(Debug, Copy, Clone, enum_map::Enum)]
pub enum LintCategory {
    /// Code that is outright wrong or useless
    Correctness,
    /// Code that is most likely wrong or useless
    Suspicious,
    /// Code that could be written in a more idomatic way
    Style,
    /// Lints that express strict opinions over GML, or may have false positives
    Pedantic,
}

/// A report returned by a lint if it fails.
pub struct LintReport {
    pub position: Position,
}
