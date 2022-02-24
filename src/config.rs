use std::collections::HashMap;

use crate::LintLevel;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub todo_keyword: Option<String>,
    pub max_arguments: Option<usize>,
    pub statement_parentheticals: bool,
    pub var_prefixes: bool,
    pub english_flavor: Option<EnglishFlavor>,
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
            english_flavor: Some(EnglishFlavor::American),
        }
    }
}
impl Config {
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
}

#[derive(Debug, PartialEq, Copy, Clone, enum_map::Enum, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnglishFlavor {
    American,
    British,
}
