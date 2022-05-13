use clap::{ArgEnum, Parser, Subcommand};
use duck::Config;
use std::path::PathBuf;

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
        /// The path to the project directory to run on. Uses the current directory if not provided.
        #[clap(long, short, parse(from_os_str))]
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
    /// Prints the provided lint's explanation for what it does and why it may be useful.
    Explain { lint_name: String },
    /// Serializes Asts into a file.
    ///
    /// Files that contain errors will not be included in the output.
    Emit {
        /// The name of the file to output to.
        #[clap(parse(from_os_str))]
        output_path: PathBuf,

        /// The path to the project directory to serialize. Uses the current directory if not
        /// provided. Can alternatively pass the path to a singular gml file.
        #[clap(long, short, parse(from_os_str))]
        path: Option<PathBuf>,

        /// The format to serialize the Asts as. Defaults to JSON
        #[clap(short, long, arg_enum)]
        format: Option<EmitFormat>,
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
            ConfigTemplate::Full => Config::full(),
        }
    }
}

#[derive(Parser, Debug, Copy, Clone, ArgEnum)]
pub enum EmitFormat {
    Json,
    Yaml,
}
