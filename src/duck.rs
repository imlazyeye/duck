use colored::*;
use enum_map::{enum_map, EnumMap};
use heck::{ToShoutySnakeCase, ToUpperCamelCase};
use std::{collections::HashMap, path::Path};

use crate::{
    gml::{GmlEnum, GmlSwitchStatement},
    lint::{Lint, LintLevel},
    parsing::{ParseError, Parser},
    GmlComment, GmlConstructor, GmlKeywords, GmlMacro, LintTag,
};

pub struct Duck {
    lint_tags: HashMap<(String, usize), LintTag>,
    enums: Vec<GmlEnum>,
    macros: Vec<GmlMacro>,
    constructors: Vec<GmlConstructor>,
    switches: Vec<GmlSwitchStatement>,
    keywords: Vec<GmlKeywords>,
    comments: Vec<GmlComment>,
    levels: EnumMap<Lint, LintLevel>,
}

impl Duck {
    #[allow(clippy::new_without_default)]
    /// Creates a new, blank Duck. Use `Duck::parse_gml` to start collecting data.
    pub fn new() -> Self {
        pretty_env_logger::formatted_builder()
            .format_module_path(true)
            .filter(None, log::LevelFilter::Trace)
            .init();

        color_eyre::install().unwrap();
        Self {
            lint_tags: HashMap::new(),
            enums: vec![],
            macros: vec![],
            constructors: vec![],
            switches: vec![],
            keywords: vec![],
            comments: vec![],
            levels: enum_map! {
                Lint::MissingCaseMembers => LintLevel::Deny,
                Lint::MissingDefaultCase => LintLevel::Warn,
                Lint::UnrecognizedEnum => LintLevel::Allow,
                Lint::AndKeyword => LintLevel::Allow,
                Lint::OrKeyword => LintLevel::Allow,
                Lint::NonScreamCase => LintLevel::Warn,
                Lint::NonPascalCase => LintLevel::Warn,
                Lint::AnonymousConstructor => LintLevel::Warn,
                Lint::NoSpaceAtStartOfComment => LintLevel::Allow,
            },
        }
    }

    /// Parses the given String of GML, collecting data for Duck.
    pub fn parse_gml(&mut self, source_code: &str, path: &Path) -> Result<(), ParseError> {
        let mut parser = Parser::new(source_code.to_string(), path.to_path_buf());
        self.lint_tags
            .extend(&mut parser.collect_lint_tags()?.into_iter());
        self.comments.append(&mut parser.collect_gml_comments()?);
        self.enums.append(&mut parser.collect_gml_enums()?);
        self.macros.append(&mut parser.collect_gml_macros()?);
        self.constructors
            .append(&mut parser.collect_gml_constructors()?);
        self.keywords.append(&mut parser.collect_gml_keywords()?);
        self.switches
            .append(&mut parser.collect_gml_switch_statements()?);
        Ok(())
    }

    pub fn report_lint(
        &self,
        lint: Lint,
        position: &Position,
        additional_information: String,
        lint_counts: &mut EnumMap<LintLevel, usize>,
    ) {
        let user_provided_level = self.get_user_provided_level(lint, position);
        let actual_level = user_provided_level.unwrap_or(self.levels[lint]);
        lint_counts[actual_level] += 1;
        let lint_string = match actual_level {
            LintLevel::Allow => return, // allow this!
            LintLevel::Warn => "warning".yellow().bold(),
            LintLevel::Deny => "error".bright_red().bold(),
        };
        let path_message = format!("\n {} {}", "-->".bold().bright_blue(), position.file_string);
        let snippet_message = format!(
            "\n{}\n{}{}\n{}",
            " | ".bright_blue().bold(),
            " | ".bright_blue().bold(),
            position.snippet,
            " | ".bright_blue().bold()
        );
        let additional_message = match lint {
            Lint::MissingCaseMembers => {
                format!(
                    "\n\n Missing the following members: {}",
                    additional_information
                )
            }
            Lint::UnrecognizedEnum => {
                format!("\n\n Missing enum: {}", additional_information)
            }
            Lint::AndKeyword | Lint::OrKeyword | Lint::NonPascalCase | Lint::NonScreamCase => {
                format!("\n\n {additional_information}")
            }
            Lint::MissingDefaultCase
            | Lint::AnonymousConstructor
            | Lint::NoSpaceAtStartOfComment => "".into(),
        };
        let show_suggestions = true;
        let suggestion_message = if show_suggestions {
            format!("\n\n {}: {}", "suggestion".bold(), lint.hint_message())
        } else {
            "".into()
        };
        let note_message = format!(
            "\n\n {}: {}",
            "note".bold(),
            if user_provided_level.is_some() {
                "This lint was specifically requested by in line above this source code".into()
            } else {
                lint.explanation_message(actual_level)
            }
        )
        .bright_black();
        println!(
            "{}: {}{path_message}{snippet_message}{additional_message}{suggestion_message}{note_message}\n",
            lint_string,
            lint.error_message().bright_white(),
        );
    }

    pub fn find_enum_by_name(&self, name: &str) -> Option<&GmlEnum> {
        self.enums.iter().find(|v| v.name() == name)
    }

    /// Get an iterator to the duck's switches.
    pub fn switches(&self) -> &[GmlSwitchStatement] {
        self.switches.as_ref()
    }

    /// Get a reference to the duck's enums.
    pub fn enums(&self) -> &[GmlEnum] {
        self.enums.as_ref()
    }

    /// Get a reference to the collected keywords.
    pub fn keywords(&self) -> &[GmlKeywords] {
        self.keywords.as_ref()
    }

    /// Get a reference to the collected macros.
    pub fn macros(&self) -> &[GmlMacro] {
        self.macros.as_ref()
    }

    /// Get a reference to the the collected constructors.
    pub fn constructors(&self) -> &[GmlConstructor] {
        self.constructors.as_ref()
    }

    /// Get a reference to the colllected comments.
    pub fn comments(&self) -> &[GmlComment] {
        self.comments.as_ref()
    }

    /// Gets the user-specified level for the given position (if one exists)
    pub fn get_user_provided_level(
        &self,
        lint: Lint,
        position: &Position,
    ) -> Option<LintLevel> {
        // Check if the line above this position has a lint tag
        if let Some(lint_tag) = self
            .lint_tags
            // that clone there... look, we're all just doing our best here, okay?
            .get(&(position.file_name.clone(), position.line))
        {
            // Check if its the right one?
            if lint_tag.0 == lint {
                // Dabs -- you get this level
                Some(lint_tag.1)
            } else {
                // W-what are you doing here? You get the global...
                None
            }
        } else {
            None
        }
    }
}

// Utils
impl Duck {
    /// Returns the given string under Duck's definition of PascalCase.
    pub fn pascal_case(string: &str) -> String {
        let output = string.to_upper_camel_case();
        let mut prefix = String::new();
        let mut chars = string.chars();
        while let Some('_') = chars.next() {
            prefix.push('_');
        }
        prefix + &output
    }

    /// Returns the given string under Duck's definition of SCREAM_CASE.
    pub fn scream_case(string: &str) -> String {
        let output = string.to_shouty_snake_case();
        let mut prefix = String::new();
        let mut chars = string.chars();
        while let Some('_') = chars.next() {
            prefix.push('_');
        }
        prefix + &output
    }

    pub fn create_file_position_string(
        file_contents: &str,
        file_name: &str,
        cursor: usize,
    ) -> Position {
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
        let line_and_after = &file_contents[cursor - column + 1..];
        let last_index = line_and_after
            .match_indices('\n')
            .next()
            .map_or(line_and_after.len() - 1, |(i, _)| i);
        let snippet = &line_and_after[..last_index];
        Position {
            file_name: file_name.to_string(),
            line,
            column,
            file_string: format!("{}:{}:{}", file_name, line, column),
            snippet: snippet.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Position {
    pub file_name: String,
    pub line: usize,
    pub column: usize,
    pub file_string: String,
    pub snippet: String,
}
