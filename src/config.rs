use std::collections::HashMap;

use crate::{lint::LintLevelSetting, LintLevel};

/// A series of various settings shared by the lints to customize their
/// behavior.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    /// The name of a function in the the user's codebase used to denote
    /// temporary code. Used by [todo].
    todo_keyword: Option<String>,
    /// The maximum number of arguments the [too_many_arguments] lint will allow
    /// function declarations to have.
    max_arguments: Option<usize>,
    /// Whether or not [statement_parenthetical_violation] lint should be
    /// asserting that statements do or do not contain surrounding
    /// parenthesis.
    statement_parentheticals: bool,
    /// Whether or not the [var_prefix_violation] lint should be asserting that
    /// local variables do or do not start with an underscore.
    var_prefixes: bool,
    /// The user's selected [EnglishFlavor].
    english_flavor: Option<EnglishFlavor>,
    /// The name of an enum member that the [missing_case_member] should ignore,
    /// such as "Len" or "Count".
    length_enum_member_name: Option<String>,
    /// Manual definitions for any lint's lint level. The key is the lint's tag.
    ///
    /// FIXME: We do not currently validate that all entries are valid lint
    /// tags!
    lint_levels: HashMap<String, LintLevel>,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            todo_keyword: Default::default(),
            max_arguments: Some(7),
            statement_parentheticals: true,
            var_prefixes: true,
            lint_levels: Default::default(),
            length_enum_member_name: None,
            english_flavor: Some(EnglishFlavor::American),
        }
    }
}
impl Config {
    /// Gets the level needed for a lint based on its tag, taking a default
    /// if the user hasn't specifide anything.
    pub fn get_lint_level_setting(&self, tag: &str, default: LintLevel) -> LintLevelSetting {
        // Check if there is a config-based rule for this lint
        if let Some((_, level)) = self.lint_levels.iter().find(|(key, _)| key == &tag) {
            LintLevelSetting::ConfigSpecified(*level)
        } else {
            LintLevelSetting::Default(default)
        }
    }

    /// Get a reference to the config's todo keyword.
    pub fn todo_keyword(&self) -> Option<&String> {
        self.todo_keyword.as_ref()
    }

    /// Get the config's max arguments.
    pub fn max_arguments(&self) -> Option<usize> {
        self.max_arguments
    }

    /// Get the config's english flavor.
    pub fn english_flavor(&self) -> Option<EnglishFlavor> {
        self.english_flavor
    }

    /// Get a reference to the config's length enum member name.
    pub fn length_enum_member_name(&self) -> Option<&String> {
        self.length_enum_member_name.as_ref()
    }

    /// Get the config's statement parentheticals.
    pub fn statement_parentheticals(&self) -> bool {
        self.statement_parentheticals
    }

    /// Get the config's var prefixes.
    pub fn var_prefixes(&self) -> bool {
        self.var_prefixes
    }
}

/// The spelling preference in the user's codebase for the built-in GameMaker
/// functions (such as `draw_color` vs `draw_colour`).
#[derive(Debug, PartialEq, Copy, Clone, enum_map::Enum, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnglishFlavor {
    /// American spelling preference (ie: `draw_color`).
    American,
    /// British spelling preference (ie: `draw_colour`).
    British,
}
