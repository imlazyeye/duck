use colored::*;
use enum_map::{enum_map, EnumMap};
use heck::{ToShoutySnakeCase, ToUpperCamelCase};
use std::{collections::HashMap, path::Path};

use crate::{
    gml::{GmlEnum, GmlSwitchStatement},
    issues::{ClippieIssue, ClippieLevel},
    parsing::{ClippieParseError, Parser},
    ClippieIssueTag, GmlComment, GmlConstructor, GmlKeywords, GmlMacro,
};

pub struct Clippie {
    issue_tags: HashMap<(String, usize), ClippieIssueTag>,
    enums: Vec<GmlEnum>,
    macros: Vec<GmlMacro>,
    constructors: Vec<GmlConstructor>,
    switches: Vec<GmlSwitchStatement>,
    keywords: Vec<GmlKeywords>,
    comments: Vec<GmlComment>,
    levels: EnumMap<ClippieIssue, ClippieLevel>,
}

impl Clippie {
    #[allow(clippy::new_without_default)]
    /// Creates a new, blank Clippie. Use `Clippie::parse_gml` to start collecting data.
    pub fn new() -> Self {
        pretty_env_logger::formatted_builder()
            .format_module_path(true)
            .filter(None, log::LevelFilter::Trace)
            .init();

        color_eyre::install().unwrap();
        Self {
            issue_tags: HashMap::new(),
            enums: vec![],
            macros: vec![],
            constructors: vec![],
            switches: vec![],
            keywords: vec![],
            comments: vec![],
            levels: enum_map! {
                ClippieIssue::MissingCaseMembers => ClippieLevel::Deny,
                ClippieIssue::MissingDefaultCase => ClippieLevel::Warn,
                ClippieIssue::UnrecognizedEnum => ClippieLevel::Allow,
                ClippieIssue::AndKeyword => ClippieLevel::Allow,
                ClippieIssue::OrKeyword => ClippieLevel::Allow,
                ClippieIssue::NonScreamCase => ClippieLevel::Warn,
                ClippieIssue::NonPascalCase => ClippieLevel::Warn,
                ClippieIssue::AnonymousConstructor => ClippieLevel::Warn,
                ClippieIssue::NoSpaceAtStartOfComment => ClippieLevel::Allow,
            },
        }
    }

    /// Parses the given String of GML, collecting data for Clippie.
    pub fn parse_gml(&mut self, source_code: &str, path: &Path) -> Result<(), ClippieParseError> {
        let mut parser = Parser::new(source_code.to_string(), path.to_path_buf());
        self.issue_tags
            .extend(&mut parser.collect_issue_tags()?.into_iter());
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

    pub fn raise_issue(
        &self,
        issue: ClippieIssue,
        position: &ClippiePosition,
        additional_information: String,
        lint_counts: &mut EnumMap<ClippieLevel, usize>,
    ) {
        let user_provided_level = self.get_user_provided_level(issue, position);
        let actual_level = user_provided_level.unwrap_or(self.levels[issue]);
        lint_counts[actual_level] += 1;
        let issue_string = match actual_level {
            ClippieLevel::Allow => return, // allow this!
            ClippieLevel::Warn => "warning".yellow().bold(),
            ClippieLevel::Deny => "error".bright_red().bold(),
        };
        let path_message = format!("\n {} {}", "-->".bold().bright_blue(), position.file_string);
        let snippet_message = format!(
            "\n{}\n{}{}\n{}",
            " | ".bright_blue().bold(),
            " | ".bright_blue().bold(),
            position.snippet,
            " | ".bright_blue().bold()
        );
        let additional_message = match issue {
            ClippieIssue::MissingCaseMembers => {
                format!(
                    "\n\n Missing the following members: {}",
                    additional_information
                )
            }
            ClippieIssue::UnrecognizedEnum => {
                format!("\n\n Missing enum: {}", additional_information)
            }
            ClippieIssue::AndKeyword
            | ClippieIssue::OrKeyword
            | ClippieIssue::NonPascalCase
            | ClippieIssue::NonScreamCase => {
                format!("\n\n {additional_information}")
            }
            ClippieIssue::MissingDefaultCase
            | ClippieIssue::AnonymousConstructor
            | ClippieIssue::NoSpaceAtStartOfComment => "".into(),
        };
        let show_suggestions = true;
        let suggestion_message = if show_suggestions {
            format!("\n\n {}: {}", "suggestion".bold(), issue.hint_message())
        } else {
            "".into()
        };
        let note_message = format!(
            "\n\n {}: {}",
            "note".bold(),
            if user_provided_level.is_some() {
                "This lint was specifically requested by in line above this source code".into()
            } else {
                issue.explanation_message(actual_level)
            }
        )
        .bright_black();
        println!(
            "{}: {}{path_message}{snippet_message}{additional_message}{suggestion_message}{note_message}\n",
            issue_string,
            issue.error_message().bright_white(),
        );
    }

    pub fn find_enum_by_name(&self, name: &str) -> Option<&GmlEnum> {
        self.enums.iter().find(|v| v.name() == name)
    }

    /// Get an iterator to the clippie's switches.
    pub fn switches(&self) -> &[GmlSwitchStatement] {
        self.switches.as_ref()
    }

    /// Get a reference to the clippie's enums.
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
        issue: ClippieIssue,
        position: &ClippiePosition,
    ) -> Option<ClippieLevel> {
        // Check if the line above this position has a issue tag
        if let Some(issue_tag) = self
            .issue_tags
            // that clone there... look, we're all just doing our best here, okay?
            .get(&(position.file_name.clone(), position.line))
        {
            // Check if its the right one?
            if issue_tag.0 == issue {
                // Dabs -- you get this level
                Some(issue_tag.1)
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
impl Clippie {
    /// Returns the given string under Clippie's definition of PascalCase.
    pub fn pascal_case(string: &str) -> String {
        let output = string.to_upper_camel_case();
        let mut prefix = String::new();
        let mut chars = string.chars();
        while let Some('_') = chars.next() {
            prefix.push('_');
        }
        prefix + &output
    }

    /// Returns the given string under Clippie's definition of SCREAM_CASE.
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
    ) -> ClippiePosition {
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
        ClippiePosition {
            file_name: file_name.to_string(),
            line,
            column,
            file_string: format!("{}:{}:{}", file_name, line, column),
            snippet: snippet.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct ClippiePosition {
    pub file_name: String,
    pub line: usize,
    pub column: usize,
    pub file_string: String,
    pub snippet: String,
}
