use clap::Parser;
use colored::Colorize;
use duck::Config;
use duck::{utils::FilePreviewUtil, Duck};
use std::path::PathBuf;

mod input;
pub use input::*;

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    color_eyre::install().unwrap();
    let input = Cli::parse();
    match input.command {
        Commands::Lint { path } => run_lint(path).await,
        Commands::NewConfig => new_config(),
    }
}

async fn run_lint(path: Option<PathBuf>) {
    // Run duck
    let timer = std::time::Instant::now();
    let current_directory = path
        .unwrap_or_else(|| std::env::current_dir().expect("Cannot access the current directory!"));
    let (duck, config_usage) =
        if let Ok(text) = std::fs::read_to_string(current_directory.join(".duck.toml")) {
            match toml::from_str::<Config>(&text) {
                Ok(config) => (Duck::new(config), ConfigUsage::Some),
                Err(e) => (Duck::default(), ConfigUsage::Failed(e)),
            }
        } else {
            (Duck::default(), ConfigUsage::None)
        };
    let run_result = duck.run(&current_directory).await;
    let total_duration = timer.elapsed();

    // Output the results
    println!(
        "{}",
        run_result
            .iter_lint_reports()
            .map(|(path, gml, report)| {
                report.generate_string(
                    duck.config(),
                    &FilePreviewUtil::new(gml, path.to_str().unwrap(), report.span.0),
                )
            })
            .collect::<String>()
    );

    let seperation_string = String::from_utf8(vec![b'-'; 75]).unwrap().bold();
    println!("{seperation_string}");
    println!(
        "\n{}\n",
        format!(
            "🦆 <( Finished lint in {:.2}s with {} errors and {} warnings! )",
            total_duration.as_secs_f32(),
            run_result.denial_count().to_string().bright_red(),
            run_result.warning_count().to_string().yellow(),
        )
        .bold()
    );
    println!("{seperation_string}\n");
    match config_usage {
        ConfigUsage::None => println!("{}", "note: You are not using a configuration file, which is highly recommended! Use `duck new-config` to generate one.\n".bright_black().bold()),
        ConfigUsage::Failed(error) => warn!("Your config was not used in this run, as duck encountered the following error while being parsed: {}\n", error),
        ConfigUsage::Some => {}
    }
    if !run_result.io_errors().is_empty() {
        warn!("The following errors occured while trying to read your project's files...\n");
        run_result.io_errors().iter().for_each(|error| {
            println!("{error}");
        })
    }
    if !run_result.parse_errors().is_empty() {
        warn!("The following errors occured while trying to parse the project...\n");
        run_result
            .parse_errors()
            .iter()
            .for_each(|(path, file, error)| {
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

enum ConfigUsage {
    None,
    Some,
    Failed(toml::de::Error),
}
