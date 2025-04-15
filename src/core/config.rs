use crate::lint::{LintLevel, LintLevelSetting};
use hashbrown::HashMap;
use heck::{ToLowerCamelCase, ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use itertools::Itertools;

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
    /// The casing preferences for all symbols in gml, used by `casing_rules`.
    /// Instance/self-bound variables are noteably missing from this. These cannot be
    /// reliably added until static analysis is added to `duck`, which will come in
    /// a future update.
    #[serde(default)]
    pub casing_rules: CasingRules,
    /// The preferences for the `non_simplified_expression` lint to define whether
    /// or not different evaluation expressions are allowed to contain non-simplified
    /// math.
    #[serde(default)]
    pub simplification_rules: SimplificationRules,
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
    /// File paths that duck should ignore.
    #[serde(default)]
    pub ignored_file_paths: Vec<String>,
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
            casing_rules: Default::default(),
            lint_levels: Default::default(),
            simplification_rules: Default::default(),
            ignored_file_paths: Default::default(),
        }
    }
}
impl Config {
    /// Creates a config with every possible field present at its default value.
    pub fn full() -> Self {
        Self {
            lint_levels: HashMap::from([
                // @tags
                ("accessor_alternative".into(), LintLevel::Warn),
                ("and_preference".into(), LintLevel::Allow),
                ("anonymous_constructor".into(), LintLevel::Allow),
                ("bool_equality".into(), LintLevel::Allow),
                ("casing_rules".into(), LintLevel::Allow),
                ("collapsable_if".into(), LintLevel::Warn),
                ("condition_wrapper".into(), LintLevel::Allow),
                ("deprecated".into(), LintLevel::Warn),
                ("draw_sprite".into(), LintLevel::Allow),
                ("draw_text".into(), LintLevel::Allow),
                ("english_flavor_violation".into(), LintLevel::Allow),
                ("exit".into(), LintLevel::Allow),
                ("global".into(), LintLevel::Allow),
                ("invalid_assignment".into(), LintLevel::Deny),
                ("invalid_comparison".into(), LintLevel::Deny),
                ("invalid_equality".into(), LintLevel::Deny),
                ("missing_case_member".into(), LintLevel::Warn),
                ("missing_default_case".into(), LintLevel::Allow),
                ("mod_preference".into(), LintLevel::Allow),
                ("multi_var_declaration".into(), LintLevel::Allow),
                ("non_constant_default_parameter".into(), LintLevel::Warn),
                ("non_simplified_expression".into(), LintLevel::Warn),
                ("not_preference".into(), LintLevel::Allow),
                ("or_preference".into(), LintLevel::Allow),
                ("room_goto".into(), LintLevel::Allow),
                ("show_debug_message".into(), LintLevel::Allow),
                ("single_equals_comparison".into(), LintLevel::Warn),
                ("single_switch_case".into(), LintLevel::Warn),
                ("suspicious_constant_usage".into(), LintLevel::Deny),
                ("switch_without_case".into(), LintLevel::Warn),
                ("todo".into(), LintLevel::Allow),
                ("too_many_arguments".into(), LintLevel::Warn),
                ("try_catch".into(), LintLevel::Allow),
                ("unassigned_constructor".into(), LintLevel::Warn),
                ("unnecessary_grouping".into(), LintLevel::Warn),
                ("unused_parameter".into(), LintLevel::Warn),
                ("useless_function".into(), LintLevel::Deny),
                ("var_prefix_violation".into(), LintLevel::Allow),
                ("with_loop".into(), LintLevel::Allow),
                // @end tags
            ]),
            ..Default::default()
        }
    }
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

/// A container of casing rules used by `casing_rules`.
#[derive(Debug, PartialEq, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct CasingRules {
    /// Casing preference for constructors.
    pub constructor_rule: Casing,
    /// Casing preference for functions.
    pub function_rule: Casing,
    /// Casing preference for enums.
    pub enum_rule: Casing,
    /// Casing preference for enum members.
    pub enum_member_rule: Casing,
    /// Casing preference for macros.
    pub macro_rule: Casing,
    /// Casing preference for globals.
    pub global_rule: Casing,
    /// Casing preference for local variables.
    pub local_var_rule: Casing,
    /// Casing preference for struct fields.
    pub struct_field: Casing,
}
impl Default for CasingRules {
    fn default() -> Self {
        Self {
            constructor_rule: Casing::Pascal,
            function_rule: Casing::Snake,
            enum_rule: Casing::Pascal,
            enum_member_rule: Casing::Pascal,
            macro_rule: Casing::Scream,
            global_rule: Casing::Snake,
            local_var_rule: Casing::Snake,
            struct_field: Casing::Snake,
        }
    }
}

/// Contains preferences for the `non_simplified_expression` lint.
#[derive(Debug, PartialEq, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct SimplificationRules {
    /// Whether ot not to permit non-simplified addition (1 + 1)
    pub check_addition: bool,
    /// Whether ot not to permit non-simplified subtraction (1 - 1)
    pub check_subtraction: bool,
    /// Whether ot not to permit non-simplified multiplication (1 * 1)
    pub check_multiplication: bool,
    /// Whether ot not to permit non-simplified division (1 / 1)
    pub check_division: bool,
    /// Whether ot not to permit non-simplified bitwise (1 | 1)
    pub check_bitwise: bool,
}
impl Default for SimplificationRules {
    fn default() -> Self {
        Self {
            check_addition: true,
            check_subtraction: true,
            check_multiplication: false,
            check_division: false,
            check_bitwise: false,
        }
    }
}

/// The various casing options supported by duck for the `casing_rules` lint.
#[derive(Debug, PartialEq, Copy, Clone, enum_map::Enum, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Casing {
    /// Allows any type of casing.
    Any,
    /// snake_case
    Snake,
    /// camelCase
    Camel,
    /// PascalCase
    Pascal,
    /// SCREAM_CASE
    Scream,
}
impl Casing {
    /// Returns none if the string already matches the casing, otherwise returns a string with the
    /// desired appearance.
    pub fn test(&self, lexeme: &str) -> Option<String> {
        let mut output = match self {
            Casing::Any => return None,
            Casing::Snake => lexeme.to_snake_case(),
            Casing::Camel => lexeme.to_lower_camel_case(),
            Casing::Pascal => lexeme.to_upper_camel_case(),
            Casing::Scream => lexeme.to_shouty_snake_case(),
        };
        // We allow underscore prefxies, which heck removes (ie: __FooBar is valid PascalCase to us)
        output.insert_str(0, &lexeme[0..self.prefix_len(lexeme)]);
        if output.as_str() != lexeme { Some(output) } else { None }
    }
    /// Returns the length of the prefix in this lexeme. Ie: `__foo` has a `__` prefix.
    fn prefix_len(&self, lexeme: &str) -> usize {
        lexeme.chars().find_position(|c| c != &'_').map_or(0, |(i, _)| i)
    }
}
impl std::fmt::Display for Casing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Casing::Any => f.pad("any"),
            Casing::Snake => f.pad("snake_case"),
            Casing::Camel => f.pad("camelCase"),
            Casing::Pascal => f.pad("PascalCase"),
            Casing::Scream => f.pad("SCREAM_CASE"),
        }
    }
}
