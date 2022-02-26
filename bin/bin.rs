use clap::{Parser, Subcommand};
use colored::Colorize;
use duck::config::Config;
use duck::fs::GmlWalker;
use duck::gml::GmlCollection;
use duck::parsing::parser::Ast;
use duck::parsing::ParseError;
use duck::LintReport;
use duck::{utils::FilePreviewUtil, Duck, LintLevel};
use futures::lock::Mutex;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tokio::sync::mpsc::channel;

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
    info!("Starting up duck...");
    let timer = std::time::Instant::now();
    let current_directory = path
        .unwrap_or_else(|| std::env::current_dir().expect("Cannot access the current directory!"));

    let mut config_usage = ConfigUsage::None;
    let mut duck = if let Ok(text) = std::fs::read_to_string(current_directory.join(".duck.toml")) {
        match toml::from_str::<Config>(&text) {
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

    // All the data
    let mut lint_reports: Vec<(&str, &Path, Vec<LintReport>)> = vec![];
    let mut parse_errors: Vec<(String, PathBuf, ParseError)> = vec![];
    let mut io_errors: Vec<std::io::Error> = vec![];
    let mut collection = GmlCollection::new();
    let mut asts: Vec<(String, PathBuf, Ast)> = vec![];
    let duck_arc = Arc::new(Mutex::new(duck));
    let lint_reports_arc = Arc::new(Mutex::new(lint_reports));
    let parse_errors_arc = Arc::new(Mutex::new(parse_errors));
    let mut gml_walker = GmlWalker::new(&current_directory);
    let (path_sender, mut path_reciever) = channel::<PathBuf>(1000);
    let (file_sender, mut file_reciever) = channel::<(PathBuf, String)>(1000);
    let (ast_sender, mut ast_receiever) = channel::<(Ast, PathBuf, String)>(1000);

    // Look for files
    let walker_handle = tokio::task::spawn(async move {
        while let Some(path) = gml_walker.next().await {
            path_sender.send(path).await.unwrap();
        }
    });

    // Read files
    let file_handle = tokio::task::spawn(async move {
        while let Some(path) = path_reciever.recv().await {
            match tokio::fs::read_to_string(&path).await {
                Ok(gml) => file_sender.send((path, gml)).await.unwrap(),
                Err(io_error) => todo!(),
            };
        }
    });

    // Parse files
    let duck = duck_arc.clone();
    let parse_handle = tokio::task::spawn(async move {
        while let Some((path, gml)) = file_reciever.recv().await {
            match duck.lock().await.parse_gml(&gml, &path) {
                Ok(ast) => ast_sender.send((ast, path, gml)).await.unwrap(),
                Err(parse_error) => parse_errors_arc.lock().await.push((gml, path, parse_error)),
            }
        }
    });

    let duck = duck_arc.clone();
    let lint_reports_arc = lint_reports_arc.clone();
    let lint_handle = tokio::task::spawn(async move {
        while let Some((ast, path, gml)) = ast_receiever.recv().await {
            for statement in ast {
                let mut duck = duck.lock().await;
                let mut reports = lint_reports_arc.lock().await;
                let mut gml_collection = GmlCollection::new(); // todo
                duck.process_statement_early(&statement, &mut gml_collection, &mut vec![]);
            }
        }
    });

    walker_handle.await.unwrap();
    file_handle.await.unwrap();
    parse_handle.await.unwrap();
    lint_handle.await.unwrap();

    // duck::fs::gml_walker(current_directory, &mut io_errors, |gml, path| {
    //     match duck.parse_gml(&gml, &path) {
    //         Ok(ast) => asts.push((gml, path, ast)),
    //         Err(e) => parse_error_registrar.push((gml, path, e)),
    //     }
    // });

    // // Early pass
    // for (gml, path, ast) in asts.iter() {
    //     let mut reports = vec![];
    //     for statement in ast.iter() {
    //         duck.process_statement_early(statement, &mut collection, &mut reports);
    //     }
    //     lint_reports.push((gml, path, reports));
    // }

    // // Late pass
    // for (gml, path, ast) in asts.iter() {
    //     let mut reports = vec![];
    //     for statement in ast.iter() {
    //         duck.process_statement_late(statement, &collection, &mut reports);
    //     }
    //     lint_reports.push((gml, path, reports));
    // }

    let mut deny_count = 0;
    let mut warn_count = 0;
    let mut report_strings: Vec<String> = vec![];
    // for (file, path, reports) in lint_reports {
    //     for report in reports {
    //         match *duck.get_level_for_lint(report.tag(), report.category()) {
    //             LintLevel::Allow => {}
    //             LintLevel::Warn => warn_count += 1,
    //             LintLevel::Deny => deny_count += 1,
    //         }
    //         let cursor = report.span.0;
    //         report_strings.push(report.generate_string(
    //             &duck,
    //             &FilePreviewUtil::new(&file, path.to_str().unwrap(), cursor),
    //         ));
    //     }
    // }

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
    // match config_usage {
    //     ConfigUsage::None => println!("{}", "note: You are not using a configuration file, which is highly recommended! Use `duck new-config` to generate one.\n".bright_black().bold()),
    //     ConfigUsage::Failed(error) => warn!("Your config was not used in this run, as duck encountered the following error while being parsed: {}\n", error),
    //     ConfigUsage::Some => {}
    // }
    // if !io_errors.is_empty() {
    //     warn!("The following errors occured while trying to read your project's files...\n");
    //     io_errors.iter().for_each(|error| {
    //         println!("{error}");
    //     })
    // }
    // if !parse_errors.is_empty() {
    //     warn!("The following errors occured while trying to parse the project...\n");
    //     warn!("In the future, we will actually give you file information here...");
    //     parse_errors.iter().for_each(|(file, path, error)| {
    //         println!(
    //             "{}",
    //             error.generate_report(&FilePreviewUtil::new(
    //                 file,
    //                 path.to_str().unwrap(),
    //                 error.span().0
    //             ))
    //         )
    //         // Todo: add file information here
    //     })
    // }
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
