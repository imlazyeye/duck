use crate::lint::{LintLevel, LintLevelSetting};
use std::collections::HashMap;

/// A series of various settings shared by the lints to customize their
/// behavior.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    /// The name of a function in the the user's codebase used to denote
    /// temporary code. Used by [todo].
    #[serde(default = "default_todo_keyword")]
    pub todo_keyword: String,
    /// The maximum number of arguments the [too_many_arguments] lint will allow
    /// function declarations to have.
    #[serde(default = "default_max_arguments")]
    pub max_arguments: usize,
    /// Whether or not [statement_parenthetical_violation] lint should be
    /// asserting that statements do or do not contain surrounding
    /// parenthesis.
    #[serde(default = "default_statement_parentheticals")]
    pub statement_parentheticals: bool,
    /// Whether or not the [var_prefix_violation] lint should be asserting that
    /// local variables do or do not start with an underscore.
    #[serde(default = "default_var_prefixes")]
    pub var_prefixes: bool,
    /// The user's selected [EnglishFlavor].
    #[serde(default = "default_english_flavor")]
    pub english_flavor: EnglishFlavor,
    /// The name of an enum member that the [missing_case_member] should ignore,
    /// such as "Len" or "Count".
    #[serde(default = "default_length_enum_member_name")]
    pub length_enum_member_name: String,
    /// Whether or not to prefer `and` instead of `&&` for [and_preference].
    #[serde(default)]
    pub prefer_and_keyword: bool,
    /// Whether or not to prefer `or` instead of `||` for [or_preference].
    #[serde(default)]
    pub prefer_or_keyword: bool,
    /// Whether or not to prefer `mod` instead of `%` for [mod_preference].
    #[serde(default)]
    pub prefer_mod_keyword: bool,
    /// Whether or not to prefer `not` instead of `!` for [not_preference].
    #[serde(default)]
    pub prefer_not_keyword: bool,
    /// Manual definitions for any lint's lint level. The key is the lint's tag.
    ///
    /// FIXME: We do not currently validate that all entries are valid lint
    /// tags!
    ///
    /// Additionally, in the future, we *could* default this field to the actual default
    /// levels of the lint, and then we'd never have to fall back to calling
    /// Lint::default_level()...
    #[serde(default)]
    pub lint_levels: HashMap<String, LintLevel>,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            todo_keyword: default_todo_keyword(),
            max_arguments: default_max_arguments(),
            statement_parentheticals: default_statement_parentheticals(),
            var_prefixes: default_var_prefixes(),
            length_enum_member_name: default_length_enum_member_name(),
            english_flavor: default_english_flavor(),
            prefer_and_keyword: false,
            prefer_or_keyword: false,
            prefer_mod_keyword: false,
            prefer_not_keyword: false,
            lint_levels: Default::default(),
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

    /// Get the config's prefer and keyword.
    pub fn prefer_and_keyword(&self) -> bool {
        self.prefer_and_keyword
    }

    /// Get the config's prefer or keyword.
    pub fn prefer_or_keyword(&self) -> bool {
        self.prefer_or_keyword
    }

    /// Get the config's prefer mod keyword.
    pub fn prefer_mod_keyword(&self) -> bool {
        self.prefer_mod_keyword
    }

    /// Get the config's prefer not keyword.
    pub fn prefer_not_keyword(&self) -> bool {
        self.prefer_not_keyword
    }
}

// Default values used by serde. No, I don't love this, I just don't think there's a better
// way for me to allow all of the config values to be optional to the user, but still always have
// default values for the lints...
fn default_todo_keyword() -> String {
    "todo".into()
}
fn default_max_arguments() -> usize {
    7
}
fn default_statement_parentheticals() -> bool {
    true
}
fn default_var_prefixes() -> bool {
    true
}
fn default_english_flavor() -> EnglishFlavor {
    EnglishFlavor::American
}
fn default_length_enum_member_name() -> String {
    "Len".into()
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
