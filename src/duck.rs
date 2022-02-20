use colored::Colorize;
use enum_map::{enum_map, EnumMap};
use std::{
    collections::HashMap,
    path::{Path},
};

use crate::{
    lint::LintLevel,
    parsing::{parser::Ast, ParseError, Parser}, LintCategory, LintTag,
};

pub struct Duck {
    config: DuckConfig,
    lint_tags: HashMap<(String, usize), LintTag>,
    pub category_levels: EnumMap<LintCategory, LintLevel>,
}

impl Duck {
    #[allow(clippy::new_without_default)]
    /// Creates a new, blank Duck.
    pub fn new() -> Self {
        Self {
            config: Default::default(),
            lint_tags: HashMap::new(),
            category_levels: enum_map! {
                LintCategory::Correctness => LintLevel::Deny,
                LintCategory::Suspicious => LintLevel::Warn,
                LintCategory::Style => LintLevel::Warn,
                LintCategory::Pedantic => LintLevel::Allow,
            },
        }
    }

    /// Creates a new Duck based on a DuckConfig.
    pub fn new_with_config(config: DuckConfig) -> Self {
        let mut duck = Self::new();
        duck.config = config;
        duck
    }

    /// Parses the given String of GML, collecting data for Duck.
    pub fn parse_gml(&mut self, source_code: &str, path: &Path) -> Result<Ast, ParseError> {
        Parser::new(source_code, path.to_path_buf()).into_ast()
    }

    // /// Gets the user-specified level for the given position (if one exists)
    pub fn get_user_provided_level(
        &self,
        lint_tag: &str,
        position: &Position,
    ) -> Option<LintLevel> {
        // Check if the line above this position has a lint tag
        if let Some(tag) = self
            .lint_tags
            // that clone there... look, we're all just doing our best here, okay?
            .get(&(position.file_name.clone(), position.line))
        {
            // Check if its the right one?
            if tag.0 == lint_tag {
                return Some(tag.1);
            }
        }

        // Check if there is a config-based rule for this lint
        if let Some((_, level)) = self
            .config
            .lint_levels
            .iter()
            .find(|(key, _)| key == &lint_tag)
        {
            return Some(*level);
        }

        // User has specificed nada
        None
    }

    /// Get a reference to the duck's config.
    pub fn config(&self) -> &DuckConfig {
        &self.config
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DuckConfig {
    pub todo_keyword: Option<String>,
    pub max_arguments: Option<usize>,
    pub lint_levels: HashMap<String, LintLevel>,
}
impl Default for DuckConfig {
    fn default() -> Self {
        Self {
            todo_keyword: Default::default(),
            max_arguments: Some(7),
            lint_levels: Default::default(),
        }
    }
}

impl DuckConfig {
    /// Get a reference to the duck config's todo keyword.
    pub fn todo_keyword(&self) -> Option<&String> {
        self.todo_keyword.as_ref()
    }

    /// Get the duck config's max arguments.
    pub fn max_arguments(&self) -> Option<usize> {
        self.max_arguments
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Position {
    pub file_name: String,
    pub line: usize,
    pub column: usize,
    pub file_string: String,
    pub snippet: String,
}
impl Position {
    pub fn new(file_contents: &str, file_name: &str, cursor: usize) -> Self {
        let mut line = 1;
        let mut column = 0;
        file_contents[..cursor].chars().for_each(|c| {
            if c == '\n' {
                line += 1;
                column = 0;
            } else {
                column += 1;
            }
        });
        let line_and_after = &file_contents[cursor - column..];
        let last_index = line_and_after
            .match_indices('\n')
            .next()
            .map_or(line_and_after.len() - 1, |(i, _)| i - 1);
        let snippet = &line_and_after[..last_index];
        Self {
            file_name: file_name.to_string(),
            line,
            column,
            file_string: format!("{}:{}:{}", file_name, line, column),
            snippet: snippet.to_string(),
        }
    }

    pub fn snippet_message(&self) -> String {
        format!(
            "{}\n{}{}\n{}",
            " | ".bright_blue().bold(),
            " | ".bright_blue().bold(),
            self.snippet,
            " | ".bright_blue().bold()
        )
    }

    pub fn path_message(&self) -> String {
        format!(" {} {}", "-->".bold().bright_blue(), self.file_string)
    }
}
