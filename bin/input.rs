use clap::{ArgEnum, Parser, Subcommand};
use duck::{lint::LintLevel, Config};
use std::{collections::HashMap, path::PathBuf};

#[derive(Parser, Debug)]
#[clap(name = "duck")]
#[clap(bin_name = "duck")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Runs the primary linting process.
    Run {
        /// The path to the project directory to lint. Uses the current
        /// directory if not provided.
        #[clap(long, parse(from_os_str))]
        path: Option<PathBuf>,

        /// Prevents duck from returning a non-zero status due to lint warnings.
        #[clap(long)]
        allow_warnings: bool,

        /// Prevents duck from returning a non-zero status due to lint denials.
        #[clap(long)]
        allow_errors: bool,

        /// Prevents duck from returning a non-zero status due to gml parsing errors, or errors
        /// accessing the projects files.
        #[clap(long)]
        allow_duck_errors: bool,

        /// If provided, will force color output instead of deferring.
        #[clap(long)]
        color: bool,
    },
    /// Creates a new configuration file in the current directory.
    NewConfig {
        /// The template you'd like to use for this configuration. Defaults to "default".
        #[clap(arg_enum)]
        template: Option<ConfigTemplate>,
    },
}

#[derive(Parser, Debug, Copy, Clone, ArgEnum)]
pub enum ConfigTemplate {
    /// A standard config for duck, which includes the basics that every project should have.
    Default,
    /// A config that contains every possible option and lint.
    Full,
}
impl From<ConfigTemplate> for Config {
    fn from(template: ConfigTemplate) -> Self {
        match template {
            ConfigTemplate::Default => Config::default(),
            ConfigTemplate::Full => Config {
                lint_levels: HashMap::from([
                    // @tags
                    ("accessor_alternative".into(), LintLevel::Warn),
                    ("and_keyword".into(), LintLevel::Allow),
                    ("anonymous_constructor".into(), LintLevel::Allow),
                    ("assignment_to_call".into(), LintLevel::Deny),
                    ("bool_equality".into(), LintLevel::Allow),
                    ("constructor_without_new".into(), LintLevel::Deny),
                    ("deprecated".into(), LintLevel::Warn),
                    ("draw_sprite".into(), LintLevel::Allow),
                    ("draw_text".into(), LintLevel::Allow),
                    ("english_flavor_violation".into(), LintLevel::Allow),
                    ("exit".into(), LintLevel::Allow),
                    ("global".into(), LintLevel::Allow),
                    ("missing_case_member".into(), LintLevel::Warn),
                    ("missing_default_case".into(), LintLevel::Allow),
                    ("mod_keyword".into(), LintLevel::Allow),
                    ("multi_var_declaration".into(), LintLevel::Allow),
                    ("no_space_begining_comment".into(), LintLevel::Allow),
                    ("non_constant_default_parameter".into(), LintLevel::Warn),
                    ("non_pascal_case".into(), LintLevel::Warn),
                    ("non_scream_case".into(), LintLevel::Warn),
                    ("or_keyword".into(), LintLevel::Allow),
                    ("room_goto".into(), LintLevel::Allow),
                    ("show_debug_message".into(), LintLevel::Allow),
                    ("single_switch_case".into(), LintLevel::Warn),
                    ("statement_parenthetical_violation".into(), LintLevel::Allow),
                    ("suspicious_constant_usage".into(), LintLevel::Deny),
                    ("todo".into(), LintLevel::Allow),
                    ("too_many_arguments".into(), LintLevel::Warn),
                    ("too_many_lines".into(), LintLevel::Warn),
                    ("try_catch".into(), LintLevel::Allow),
                    ("var_prefix_violation".into(), LintLevel::Allow),
                    ("with_loop".into(), LintLevel::Allow),
                    // @end tags
                ]),
                ..Default::default()
            },
        }
    }
}
