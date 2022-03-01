use clap::{Parser, Subcommand};
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
    Lint {
        /// The path to the project directory to lint. Uses the current
        /// directory if not provided.
        #[clap(long, parse(from_os_str))]
        path: Option<PathBuf>,

        /// Prevents duck from returning a non-zero status due to lint warnings.
        #[clap(long)]
        allow_warnings: bool,

        /// Prevents duck from returning a non-zero status due to lint denials.
        #[clap(long)]
        allow_denials: bool,

        /// Prevents duck from returning a non-zero status due to gml parsing errors, or errors
        /// accessing the projects files.
        #[clap(long)]
        allow_errors: bool,
    },
    /// Creates a new configuration file in the current directory.
    NewConfig,
}
