use clap::{Parser, Subcommand};
use colored::Colorize;
use duck::{utils::FilePreviewUtil, Duck, LintLevel};
use duck::{Config, DuckTask};
use std::path::PathBuf;
use std::sync::Arc;

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    pretty_env_logger::formatted_builder()
        .format_module_path(true)
        .init();
    color_eyre::install().unwrap();

    let input = Cli::parse();

    match input.command {
        Commands::Lint { path } => run_lint(path).await,
        Commands::NewConfig => new_config(),
    }
}

async fn run_lint(path: Option<PathBuf>) {
    let timer = std::time::Instant::now();
    let current_directory = path
        .unwrap_or_else(|| std::env::current_dir().expect("Cannot access the current directory!"));
    let mut config_usage = ConfigUsage::None;
    let duck = if let Ok(text) = std::fs::read_to_string(current_directory.join(".duck.toml")) {
        match toml::from_str::<Config>(&text) {
            Ok(config) => {
                config_usage = ConfigUsage::Some;
                Duck::new(config)
            }
            Err(e) => {
                config_usage = ConfigUsage::Failed(e);
                Duck::default()
            }
        }
    } else {
        Duck::default()
    };

    let run = duck.run(&current_directory).await;
    let deny_count = run.denial_count();
    let warn_count = run.warning_count();
    let mut report_strings: Vec<String> = vec![];
    for (path, gml, report) in run.iter_lint_reports() {
        let cursor = report.span.0;
        report_strings.push(report.generate_string(
            duck.config(),
            &FilePreviewUtil::new(gml, path.to_str().unwrap(), cursor),
        ));
    }

    // Doing things this way let's us disclude the time it takes to print from our report...
    let total_duration = std::time::Instant::now()
        .duration_since(timer)
        .as_secs_f32();
    println!("{}", report_strings.into_iter().collect::<String>());

    let output = format!(
        "🦆 <( Finished lint in {:.2}s with {} errors and {} warnings! )",
        total_duration,
        deny_count.to_string().bright_red(),
        warn_count.to_string().yellow(),
    )
    .bold();
    let seperation_string = String::from_utf8(vec![b'-'; 75]).unwrap().bold();
    println!("{seperation_string}");
    println!("\n{output}\n");
    println!("{seperation_string}\n");
    match config_usage {
        ConfigUsage::None => println!("{}", "note: You are not using a configuration file, which is highly recommended! Use `duck new-config` to generate one.\n".bright_black().bold()),
        ConfigUsage::Failed(error) => warn!("Your config was not used in this run, as duck encountered the following error while being parsed: {}\n", error),
        ConfigUsage::Some => {}
    }
    if !run.io_errors().is_empty() {
        warn!("The following errors occured while trying to read your project's files...\n");
        run.io_errors().iter().for_each(|error| {
            println!("{error}");
        })
    }
    if !run.parse_errors().is_empty() {
        warn!("The following errors occured while trying to parse the project...\n");
        warn!("In the future, we will actually give you file information here...");
        run.parse_errors().iter().for_each(|(path, file, error)| {
            println!(
                "{}",
                error.generate_report(&FilePreviewUtil::new(
                    file,
                    path.to_str().unwrap(),
                    error.span().0
                ))
            )
        })
    }
}

fn new_config() {
    todo!("lol sorry make it yourself");
}

#[derive(Parser, Debug)]
#[clap(name = "duck")]
#[clap(bin_name = "duck")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Runs the primary linting process.
    Lint {
        /// The path to the project directory to lint. Uses the current directory if not provided.
        #[clap(long, parse(from_os_str))]
        path: Option<PathBuf>,
    },
    /// Creates a new configuration file in the current directory.
    NewConfig,
}

enum ConfigUsage {
    None,
    Some,
    Failed(toml::de::Error),
}
