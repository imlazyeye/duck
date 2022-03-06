use clap::Parser;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use colored::Colorize;
use duck::{
    lint::{collection::*, Lint},
    Config, Duck,
};
use num_format::{Locale, ToFormattedString};
use std::path::{Path, PathBuf};

mod input;
pub use input::*;

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
        Commands::Explain { lint_name } => explain_lint(lint_name),
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
    let writer = StandardStream::stderr(if color { ColorChoice::Always } else { ColorChoice::Auto });
    let config = codespan_reporting::term::Config::default();
    for report in run_summary.diagnostics() {
        codespan_reporting::term::emit(&mut writer.lock(), &config, run_summary.files(), report).unwrap();
    }

    let seperation_string = String::from_utf8(vec![b'-'; 50]).unwrap();
    println!("  {}", "duck complete!".italic().bold());
    println!("{seperation_string}");
    println!(
        "  {}",
        format!(
            "🦆 <( Found {} errors and {} warnings! )",
            (run_summary.denial_count()).to_string().bright_red().bold(),
            run_summary.warning_count().to_string().yellow().bold(),
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
    if !run_summary.io_errors().is_empty() {
        println!(
            "{}: The following errors occured while trying to read your project's files...\n",
            "error".bright_red().bold()
        );
        run_summary.io_errors().iter().for_each(|error| {
            println!("{error}");
        })
    }
    println!("{seperation_string}\n");
    match config_usage {
        ConfigUsage::None => println!("{}", "note: You are not using a configuration file, which is highly recommended! Use `duck new-config` to generate one.\n".bright_black().bold()),
        ConfigUsage::Failed(error) => println!("{}: Your config was not used in this run, as duck encountered the following error while being parsed: {:?}\n", "error".bright_red().bold(), error),
        ConfigUsage::Some => {}
    }

    // Return the status code
    if (!allow_warnings && run_summary.warning_count() != 0)
        || (!allow_denials && run_summary.denial_count() != 0)
        || (!allow_errors && (!run_summary.io_errors().is_empty()))
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

fn explain_lint(name: String) -> i32 {
    let message = match name.as_str() {
        // @explain. Do not remove!
        "accessor_alternative" => AccessorAlternative::explanation().into(),
        "and_preference" => AndPreference::explanation().into(),
        "anonymous_constructor" => AnonymousConstructor::explanation().into(),
        "assignment_to_call" => AssignmentToCall::explanation().into(),
        "bool_equality" => BoolEquality::explanation().into(),
        "collapsable_if" => CollapsableIf::explanation().into(),
        "deprecated" => Deprecated::explanation().into(),
        "draw_sprite" => DrawSprite::explanation().into(),
        "draw_text" => DrawText::explanation().into(),
        "english_flavor_violation" => EnglishFlavorViolation::explanation().into(),
        "exit" => Exit::explanation().into(),
        "global" => Global::explanation().into(),
        "missing_case_member" => MissingCaseMember::explanation().into(),
        "missing_default_case" => MissingDefaultCase::explanation().into(),
        "mod_preference" => ModPreference::explanation().into(),
        "multi_var_declaration" => MultiVarDeclaration::explanation().into(),
        "non_constant_default_parameter" => NonConstantDefaultParameter::explanation().into(),
        "casing_preference" => CasingPreference::explanation().into(),
        "not_preference" => NotPreference::explanation().into(),
        "or_preference" => OrPreference::explanation().into(),
        "room_goto" => RoomGoto::explanation().into(),
        "show_debug_message" => ShowDebugMessage::explanation().into(),
        "single_equals_comparison" => SingleEqualsComparison::explanation().into(),
        "single_switch_case" => SingleSwitchCase::explanation().into(),
        "statement_parenthetical_preference" => StatementParentheticalPreference::explanation().into(),
        "suspicious_constant_usage" => SuspicousConstantUsage::explanation().into(),
        "todo" => Todo::explanation().into(),
        "too_many_arguments" => TooManyArguments::explanation().into(),
        "try_catch" => TryCatch::explanation().into(),
        "var_prefix_violation" => VarPrefixViolation::explanation().into(),
        "with_loop" => WithLoop::explanation().into(),
        // @end explain. Do not remove!
        _ => format!(
            "{}: Failed to find a lint with the name '{}'!",
            "error".bold().bright_red(),
            name
        ),
    };
    println!(
        "{} {}: {message}",
        "Explanation for".bright_white().bold(),
        name.bold().bright_green()
    );
    0
}

#[derive(Debug)]
enum ConfigUsage {
    None,
    Some,
    Failed(toml::de::Error),
}
