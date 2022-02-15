use colored::*;
use enum_map::{enum_map, EnumMap};
use heck::{ToShoutySnakeCase, ToUpperCamelCase};
use std::path::PathBuf;

use crate::{
    gml::{GmlEnum, GmlSwitchStatement},
    issues::{ClippieIssue, ClippieLevel},
    parsing::{ClippieParseError, Parser},
    GmlConstructor, GmlKeywords, GmlMacro,
};

pub struct Clippie {
    enums: Vec<GmlEnum>,
    macros: Vec<GmlMacro>,
    constructors: Vec<GmlConstructor>,
    switches: Vec<GmlSwitchStatement>,
    keywords: Vec<GmlKeywords>,
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
            enums: vec![],
            macros: vec![],
            constructors: vec![],
            switches: vec![],
            keywords: vec![],
            levels: enum_map! {
                ClippieIssue::MissingCaseMembers => ClippieLevel::Allow,
                ClippieIssue::MissingDefaultCase => ClippieLevel::Allow,
                ClippieIssue::UnrecognizedEnum => ClippieLevel::Allow,
                ClippieIssue::AndKeyword => ClippieLevel::Allow,
                ClippieIssue::OrKeyword => ClippieLevel::Allow,
                ClippieIssue::NonScreamCase => ClippieLevel::Warn,
                ClippieIssue::NonPascalCase => ClippieLevel::Warn,
                ClippieIssue::AnonymousConstructor => ClippieLevel::Warn,
            },
        }
    }

    /// Parses the given String of GML, collecting data for Clippie.
    pub fn parse_gml(
        &mut self,
        source_code: String,
        path: PathBuf,
    ) -> Result<(), ClippieParseError> {
        let mut parser = Parser::new(source_code, path);
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
        position: &str,
        additional_information: String,
        lint_counts: &mut EnumMap<ClippieLevel, usize>,
    ) {
        let level = self.levels[issue];
        lint_counts[level] += 1;
        let issue_string = match level {
            ClippieLevel::Allow => return, // allow this!
            ClippieLevel::Warn => "warning".yellow().bold(),
            ClippieLevel::Deny => "error".bright_red().bold(),
        };
        let path_message = format!("\n {} {}", "-->".bold().bright_blue(), position);
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
            ClippieIssue::MissingDefaultCase | ClippieIssue::AnonymousConstructor => "".into(),
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
            issue.explanation_message(level)
        )
        .bright_black();
        println!(
            "{}: {}{path_message}{additional_message}{suggestion_message}{note_message}\n",
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

    /// Gets the global level for the given issue
    pub fn issue_level(&self, issue: ClippieIssue) -> &ClippieLevel {
        &self.levels[issue]
    }

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
}
