use clap::{Parser, Subcommand};
use colored::Colorize;
use duck::parsing::ParseError;
use duck::{Duck, FilePreviewUtil, LintLevel};
use duck::{DuckConfig, LintReport};
use std::path::PathBuf;

#[macro_use]
extern crate log;

fn main() {
    pretty_env_logger::formatted_builder()
        .format_module_path(true)
        .init();
    color_eyre::install().unwrap();

    let input = Cli::parse();

    match input.command {
        Commands::Lint { path } => run_lint(path),
        Commands::NewConfig => new_config(),
    }
}

fn run_lint(path: Option<PathBuf>) {
    info!("Starting up duck...");
    let timer = std::time::Instant::now();
    let current_directory = path
        .unwrap_or_else(|| std::env::current_dir().expect("Cannot access the current directory!"));

    let mut config_usage = ConfigUsage::None;
    let mut duck = if let Ok(text) = std::fs::read_to_string(current_directory.join(".duck.toml")) {
        match toml::from_str::<DuckConfig>(&text) {
            Ok(config) => {
                config_usage = ConfigUsage::Some;
                Duck::new_with_config(config)
            }
            Err(e) => {
                config_usage = ConfigUsage::Failed(e);
                Duck::new()
            }
        }
    } else {
        Duck::new()
    };

    let mut lint_report_registrar: Vec<(String, PathBuf, Vec<LintReport>)> = vec![];
    let mut parse_error_registrar: Vec<(String, PathBuf, ParseError)> = vec![];
    let mut io_errors: Vec<std::io::Error> = vec![];
    duck::fs::visit_all_gml_files(current_directory, &mut io_errors, |gml, path| {
        let mut reports = vec![];
        match duck.lint_gml(gml.clone(), &path, &mut reports) {
            Ok(_) => lint_report_registrar.push((gml, path, reports)),
            Err(e) => parse_error_registrar.push((gml, path, e)),
        }
    });

    let mut deny_count = 0;
    let mut warn_count = 0;
    let mut report_strings = vec![];
    for (file, path, reports) in lint_report_registrar {
        for report in reports {
            match *duck.get_level_for_lint(report.tag(), report.category()) {
                LintLevel::Allow => {}
                LintLevel::Warn => warn_count += 1,
                LintLevel::Deny => deny_count += 1,
            }
            let cursor = report.span.0;
            report_strings.push(report.generate_string(
                &duck,
                &FilePreviewUtil::new(&file, path.to_str().unwrap(), cursor),
            ));
        }
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
    if !io_errors.is_empty() {
        warn!("The following errors occured while trying to read your project's files...\n");
        io_errors.iter().for_each(|error| {
            println!("{error}");
        })
    }
    if !parse_error_registrar.is_empty() {
        warn!("The following errors occured while trying to parse the project...\n");
        warn!("In the future, we will actually give you file information here...");
        parse_error_registrar
            .iter()
            .for_each(|(file, path, error)| {
                println!(
                    "{}",
                    error.generate_report(&FilePreviewUtil::new(
                        file,
                        path.to_str().unwrap(),
                        error.span().0
                    ))
                )
                // Todo: add file information here
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
