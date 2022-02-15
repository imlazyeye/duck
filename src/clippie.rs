use std::path::{Path, PathBuf};

use enum_map::{enum_map, EnumMap};
use yy_boss::{Resource, YyResource, YypBoss};

use colored::*;

use crate::{
    gml::{GmlEnum, GmlSwitchStatement},
    issues::{ClippieIssue, ClippieLevel},
    parsing::{ClippieParseError, Parser},
    GmlMacro, IllegalGmlCharacter,
};

pub struct Clippie {
    boss: YypBoss,
    enums: Vec<GmlEnum>,
    macros: Vec<GmlMacro>,
    switches: Vec<GmlSwitchStatement>,
    illegal_characters: Vec<IllegalGmlCharacter>,
    levels: EnumMap<ClippieIssue, ClippieLevel>,
}

impl Clippie {
    pub fn new(yyp_path: &str) -> Self {
        let mut clippie = Self {
            boss: YypBoss::with_startup_injest(yyp_path, &[Resource::Script, Resource::Object])
                .unwrap(),
            enums: vec![],
            macros: vec![],
            switches: vec![],
            illegal_characters: vec![],
            levels: enum_map! {
                ClippieIssue::MissingCaseMembers => ClippieLevel::Allow,
                ClippieIssue::MissingDefaultCase => ClippieLevel::Allow,
                ClippieIssue::UnrecognizedEnum => ClippieLevel::Allow,
                ClippieIssue::AndKeyword => ClippieLevel::Allow,
                ClippieIssue::OrKeyword => ClippieLevel::Allow,
                ClippieIssue::NonScreamCase => ClippieLevel::Warn,
                ClippieIssue::NonPascalCase => ClippieLevel::Warn,
            },
        };

        // We must get all enums first
        Self::iterate_gml_files(&clippie.boss, |data| {
            let mut parser = Parser::new(data.gml_content, data.resource_path.to_path_buf());
            match parser.collect_gml_enums() {
                Ok(mut enums) => clippie.enums.append(&mut enums),
                Err(e) => match e {
                    ClippieParseError::UnexpectedToken(token, target) => {
                        error!(target: &target, "Unexpected token: {:?}", token)
                    }
                    e => error!(target: "Location Unknown", "{:?}", e),
                },
            }
        });

        // The rest can happen side by side
        Self::iterate_gml_files(&clippie.boss, |data| {
            let mut parser = Parser::new(data.gml_content, data.resource_path.to_path_buf());
            match parser.collect_gml_switch_statements() {
                Ok(mut switches) => clippie.switches.append(&mut switches),
                Err(e) => match e {
                    ClippieParseError::UnexpectedToken(token, target) => {
                        error!(target: &target, "Unexpected token: {:?}", token)
                    }
                    e => error!(target: "Location Unknown", "{:?}", e),
                },
            }

            let mut parser = Parser::new(data.gml_content, data.resource_path.to_path_buf());
            match parser.collect_illegal_keywords_from_gml() {
                Ok(mut illegal_characters) => {
                    clippie.illegal_characters.append(&mut illegal_characters)
                }
                Err(e) => match e {
                    ClippieParseError::UnexpectedToken(token, target) => {
                        error!(target: &target, "Unexpected token: {:?}", token)
                    }
                    e => error!(target: "Location Unknown", "{:?}", e),
                },
            }

            let mut parser = Parser::new(data.gml_content, data.resource_path.to_path_buf());
            match parser.collect_gml_macros() {
                Ok(mut macros) => clippie.macros.append(&mut macros),
                Err(e) => match e {
                    ClippieParseError::UnexpectedToken(token, target) => {
                        error!(target: &target, "Unexpected token: {:?}", token)
                    }
                    e => error!(target: "Location Unknown", "{:?}", e),
                },
            }
        });

        clippie
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
            ClippieIssue::MissingDefaultCase => "".into(),
            ClippieIssue::AndKeyword
            | ClippieIssue::OrKeyword
            | ClippieIssue::NonPascalCase
            | ClippieIssue::NonScreamCase => {
                format!("\n\n {additional_information}")
            }
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

    fn iterate_gml_files<P: FnMut(GmlFileIteration)>(boss: &YypBoss, predicate: P) {
        // Check all scripts...
        let scripts = boss.scripts.into_iter().map(|v| GmlFileIteration {
            resource_path: v
                .yy_resource
                .relative_yy_directory()
                .join(format!("{}.gml", &v.yy_resource.resource_data.name)),
            gml_content: v.associated_data.as_ref().unwrap(),
        });

        // Check all objects...
        let objects = boss
            .objects
            .into_iter()
            .flat_map(|object| {
                object
                    .associated_data
                    .as_ref()
                    .unwrap()
                    .values()
                    .map(|gml_content| {
                        (
                            object
                                .yy_resource
                                .relative_yy_directory()
                                .join(format!("{}.gml", &object.yy_resource.resource_data.name)),
                            gml_content,
                        )
                    })
            })
            .map(|(resource_path, gml_content)| GmlFileIteration {
                resource_path,
                gml_content,
            });

        // Run them
        scripts.chain(objects).for_each(predicate);
    }

    /// Get an iterator to the clippie's switches.
    pub fn switches(&self) -> &[GmlSwitchStatement] {
        self.switches.as_ref()
    }

    /// Get a reference to the clippie's enums.
    pub fn enums(&self) -> &[GmlEnum] {
        self.enums.as_ref()
    }

    /// Gets the global level for the given issue
    pub fn issue_level(&self, issue: ClippieIssue) -> &ClippieLevel {
        &self.levels[issue]
    }

    /// Get a reference to the clippie's illegal character.
    pub fn illegal_characters(&self) -> &[IllegalGmlCharacter] {
        self.illegal_characters.as_ref()
    }

    /// Get a reference to the clippie's macros.
    pub fn macros(&self) -> &[GmlMacro] {
        self.macros.as_ref()
    }
}

struct GmlFileIteration<'a> {
    resource_path: PathBuf,
    gml_content: &'a String,
}
