use clap::{Parser, Subcommand};
use colored::Colorize;
use duck::{Duck, FilePreviewUtil, LintLevel};
use duck::{DuckConfig, LintReport};
use std::ffi::OsStr;
use std::path::PathBuf;
use yy_boss::{Resource, YyResource, YypBoss};

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
    let yyp_path = current_directory
        .read_dir()
        .unwrap()
        .flatten()
        .find(|file| file.path().extension() == Some(OsStr::new("yyp")))
        .expect("No yyp found in active directory!")
        .path();

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

    let registrar = parse_all_gml(&mut duck, yyp_path);
    let mut deny_count = 0;
    let mut warn_count = 0;
    for (file, path, reports) in registrar {
        for report in reports {
            match report.get_true_level(&duck) {
                LintLevel::Allow => {}
                LintLevel::Warn => warn_count += 1,
                LintLevel::Deny => deny_count += 1,
            }
            let cursor = report.span.0;
            report.raise(&duck, &FilePreviewUtil::new(&file, &path, cursor));
        }
    }

    let output = format!(
        "🦆 <( Finished lint in {:.2}s with {} errors and {} warnings! )",
        std::time::Instant::now()
            .duration_since(timer)
            .as_secs_f32(),
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
    if !duck.parsing_errors().is_empty() {
        println!("The following errors occured while trying to parse the project...\n");
        warn!("In the future, we will actually give you file information here...");
        duck.parsing_errors().iter().for_each(|error| {
            // Todo: add file information here
            println!("{error}");
        })
    }
}

fn new_config() {
    todo!("lol sorry make it yourself");
}

fn parse_all_gml(duck: &mut Duck, yyp_path: PathBuf) -> Vec<(String, String, Vec<LintReport>)> {
    let boss =
        YypBoss::with_startup_injest(yyp_path, &[Resource::Script, Resource::Object]).unwrap();

    // Parse it all
    let mut registrar = vec![];
    let gml = boss
        .scripts
        .into_iter()
        .map(|script| {
            (
                script.associated_data.clone().unwrap(),
                script
                    .yy_resource
                    .relative_yy_directory()
                    .join(format!("{}.gml", &script.yy_resource.resource_data.name)),
            )
        })
        .chain(boss.objects.into_iter().flat_map(|object| {
            object
                .associated_data
                .as_ref()
                .unwrap()
                .iter()
                .map(|(event_type, gml_content)| {
                    (
                        gml_content.to_string(),
                        object
                            .yy_resource
                            .relative_yy_directory()
                            .join(format!("{}.gml", event_type.filename_simple())),
                    )
                })
        }));

    for (gml, path) in gml {
        let mut reports = vec![];
        match duck.lint_gml(gml.clone(), &path, &mut reports) {
            Ok(_) => registrar.push((gml, path.to_str().unwrap().into(), reports)),
            Err(error) => error!("{}", error),
        }
    }
    registrar
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
