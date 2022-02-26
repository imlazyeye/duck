use std::collections::HashMap;

use crate::{lint::LintLevelSetting, LintCategory, LintLevel};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub todo_keyword: Option<String>,
    pub max_arguments: Option<usize>,
    pub statement_parentheticals: bool,
    pub var_prefixes: bool,
    pub english_flavor: Option<EnglishFlavor>,
    pub length_enum_member_name: Option<String>,
    pub lint_levels: HashMap<String, LintLevel>,
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
    /// Gets the user-desired level for the lint tag.
    pub fn get_level_for_lint(
        &self,
        lint_tag: &str,
        lint_category: LintCategory,
    ) -> LintLevelSetting {
        // Check if there is a config-based rule for this lint
        if let Some((_, level)) = self.lint_levels.iter().find(|(key, _)| key == &lint_tag) {
            return LintLevelSetting::ConfigSpecified(*level);
        }

        // User has specificed nada
        LintLevelSetting::Default(lint_category.default_level())
    }

    /// Get a reference to the duck config's todo keyword.
    pub fn todo_keyword(&self) -> Option<&String> {
        self.todo_keyword.as_ref()
    }

    /// Get the duck config's max arguments.
    pub fn max_arguments(&self) -> Option<usize> {
        self.max_arguments
    }

    /// Get the duck config's english flavor.
    pub fn english_flavor(&self) -> Option<EnglishFlavor> {
        self.english_flavor
    }

    /// Get a reference to the config's length enum member name.
    pub fn length_enum_member_name(&self) -> Option<&String> {
        self.length_enum_member_name.as_ref()
    }
}

#[derive(Debug, PartialEq, Copy, Clone, enum_map::Enum, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnglishFlavor {
    American,
    British,
}
