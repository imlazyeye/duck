use clap::Parser;
use colored::Colorize;
use duck::{utils::FilePreviewUtil, Config, Duck};
use num_format::{Locale, ToFormattedString};
use std::path::{Path, PathBuf};

mod input;
pub use input::*;

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    let input = Cli::parse();
    let status_code = match input.command {
        Commands::Run {
            path,
            allow_warnings,
            allow_errors,
            allow_duck_errors,
            color,
        } => run_lint(path, allow_warnings, allow_errors, allow_duck_errors, color).await,
        Commands::NewConfig { template } => new_config(template.unwrap_or(ConfigTemplate::Default)),
    };
    std::process::exit(status_code);
}

async fn run_lint(
    path: Option<PathBuf>,
    allow_warnings: bool,
    allow_denials: bool,
    allow_errors: bool,
    color: bool,
) -> i32 {
    // Force colors?
    if color {
        std::env::set_var("CLICOLOR_FORCE", "1");
    }

    // Run duck
    let timer = std::time::Instant::now();
    let current_directory =
        path.unwrap_or_else(|| std::env::current_dir().expect("Cannot access the current directory!"));
    let (duck, config_usage) = if let Ok(text) = std::fs::read_to_string(current_directory.join(".duck.toml")) {
        match toml::from_str::<Config>(&text) {
            Ok(config) => (Duck::new(config), ConfigUsage::Some),
            Err(e) => (Duck::default(), ConfigUsage::Failed(e)),
        }
    } else {
        (Duck::default(), ConfigUsage::None)
    };
    let run_summary = duck.run(&current_directory).await.unwrap();
    let total_duration = timer.elapsed();

    // Output the results
    println!(
        "{}",
        run_summary
            .iter_lint_reports()
            .map(|(path, gml, report)| {
                report.generate_string(
                    duck.config(),
                    &FilePreviewUtil::new(gml, path.to_str().unwrap(), report.span().0),
                )
            })
            .collect::<String>()
    );

    let seperation_string = String::from_utf8(vec![b'-'; 50]).unwrap();
    println!("  {}", "duck complete!".italic().bold());
    println!("{seperation_string}");
    println!(
        "  {}",
        format!(
            "🦆 <( Found {} errors and {} warnings! )",
            run_summary.denial_count().to_string().bright_red(),
            run_summary.warning_count().to_string().yellow(),
        )
        .bold()
    );
    println!(
        "  {}",
        format!(
            "Ran on {} lines in {:.2}s.",
            run_summary.lines_parsed().to_formatted_string(&Locale::en),
            total_duration.as_secs_f32(),
        )
        .italic()
        .bright_black()
    );
    println!("{seperation_string}\n");
    match config_usage {
        ConfigUsage::None => println!("{}", "note: You are not using a configuration file, which is highly recommended! Use `duck new-config` to generate one.\n".bright_black().bold()),
        ConfigUsage::Failed(error) => println!("{}: Your config was not used in this run, as duck encountered the following error while being parsed: {:?}\n", "error".bright_red().bold(), error),
        ConfigUsage::Some => {}
    }
    if !run_summary.io_errors().is_empty() {
        warn!("The following errors occured while trying to read your project's files...\n");
        run_summary.io_errors().iter().for_each(|error| {
            println!("{error}");
        })
    }
    if !run_summary.parse_errors().is_empty() {
        warn!("The following errors occured while trying to parse the project...\n");
        run_summary.parse_errors().iter().for_each(|(path, file, error)| {
            println!(
                "{}",
                error.generate_report(&FilePreviewUtil::new(file, path.to_str().unwrap(), error.span().0))
            )
        })
    }

    // Return the status code
    if (!allow_warnings && run_summary.warning_count() != 0)
        || (!allow_denials && run_summary.denial_count() != 0)
        || (!allow_errors && (!run_summary.io_errors().is_empty() || !run_summary.parse_errors().is_empty()))
    {
        1
    } else {
        0
    }
}

fn new_config(template: ConfigTemplate) -> i32 {
    let config_path = std::env::current_dir()
        .expect("Cannot access the current directory!")
        .join(".duck.toml");
    let config: Config = template.into();
    if Path::exists(&config_path) {
        println!("You already have a config in this directory! Please remove it before creating a new one.");
    } else {
        std::fs::write(&config_path, toml::to_string(&config).unwrap()).unwrap();
        println!("Created a new configuration file at {:?}", config_path);
    }
    0
}

enum ConfigUsage {
    None,
    Some,
    Failed(toml::de::Error),
}
