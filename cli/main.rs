use clap::Parser;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use colored::Colorize;
use duck::{
    lint::{collection::*, Lint, LintLevelSetting},
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
    let (duck, config_usage) = create_duck(&current_directory);
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
    let current_directory = std::env::current_dir().expect("Cannot access the current directory!");
    let (duck, _) = create_duck(&current_directory);
    let (message, default_level) = match name.as_str() {
        // @explain. Do not remove!
        "accessor_alternative" => (
            AccessorAlternative::explanation().to_string(),
            AccessorAlternative::default_level(),
        ),
        "and_preference" => (AndPreference::explanation().to_string(), AndPreference::default_level()),
        "anonymous_constructor" => (
            AnonymousConstructor::explanation().to_string(),
            AnonymousConstructor::default_level(),
        ),
        "bool_equality" => (BoolEquality::explanation().to_string(), BoolEquality::default_level()),
        "casing_rules" => (CasingRules::explanation().to_string(), CasingRules::default_level()),
        "collapsable_if" => (CollapsableIf::explanation().to_string(), CollapsableIf::default_level()),
        "deprecated" => (Deprecated::explanation().to_string(), Deprecated::default_level()),
        "draw_sprite" => (DrawSprite::explanation().to_string(), DrawSprite::default_level()),
        "draw_text" => (DrawText::explanation().to_string(), DrawText::default_level()),
        "english_flavor_violation" => (
            EnglishFlavorViolation::explanation().to_string(),
            EnglishFlavorViolation::default_level(),
        ),
        "exit" => (Exit::explanation().to_string(), Exit::default_level()),
        "global" => (Global::explanation().to_string(), Global::default_level()),
        "invalid_assignment" => (
            InvalidAssignment::explanation().to_string(),
            InvalidAssignment::default_level(),
        ),
        "invalid_comparison" => (
            InvalidComparison::explanation().to_string(),
            InvalidComparison::default_level(),
        ),
        "invalid_equality" => (
            InvalidEquality::explanation().to_string(),
            InvalidEquality::default_level(),
        ),
        "missing_case_member" => (
            MissingCaseMember::explanation().to_string(),
            MissingCaseMember::default_level(),
        ),
        "missing_default_case" => (
            MissingDefaultCase::explanation().to_string(),
            MissingDefaultCase::default_level(),
        ),
        "mod_preference" => (ModPreference::explanation().to_string(), ModPreference::default_level()),
        "multi_var_declaration" => (
            MultiVarDeclaration::explanation().to_string(),
            MultiVarDeclaration::default_level(),
        ),
        "non_constant_default_parameter" => (
            NonConstantDefaultParameter::explanation().to_string(),
            NonConstantDefaultParameter::default_level(),
        ),
        "not_preference" => (NotPreference::explanation().to_string(), NotPreference::default_level()),
        "or_preference" => (OrPreference::explanation().to_string(), OrPreference::default_level()),
        "room_goto" => (RoomGoto::explanation().to_string(), RoomGoto::default_level()),
        "show_debug_message" => (
            ShowDebugMessage::explanation().to_string(),
            ShowDebugMessage::default_level(),
        ),
        "single_equals_comparison" => (
            SingleEqualsComparison::explanation().to_string(),
            SingleEqualsComparison::default_level(),
        ),
        "single_switch_case" => (
            SingleSwitchCase::explanation().to_string(),
            SingleSwitchCase::default_level(),
        ),
        "condition_wrapper" => (
            ConditionWrapper::explanation().to_string(),
            ConditionWrapper::default_level(),
        ),
        "suspicious_constant_usage" => (
            SuspicousConstantUsage::explanation().to_string(),
            SuspicousConstantUsage::default_level(),
        ),
        "todo" => (Todo::explanation().to_string(), Todo::default_level()),
        "too_many_arguments" => (
            TooManyArguments::explanation().to_string(),
            TooManyArguments::default_level(),
        ),
        "try_catch" => (TryCatch::explanation().to_string(), TryCatch::default_level()),
        "unassigned_constructor" => (
            UnassignedConstructor::explanation().to_string(),
            UnassignedConstructor::default_level(),
        ),
        "unnecessary_grouping" => (
            UnnecessaryGrouping::explanation().to_string(),
            UnnecessaryGrouping::default_level(),
        ),
        "useless_function" => (
            UselessFunction::explanation().to_string(),
            UselessFunction::default_level(),
        ),
        "var_prefix_violation" => (
            VarPrefixViolation::explanation().to_string(),
            VarPrefixViolation::default_level(),
        ),
        "with_loop" => (WithLoop::explanation().to_string(), WithLoop::default_level()),
        // @end explain. Do not remove!
        _ => {
            println!(
                "{}: Failed to find a lint with the name '{}'!",
                "error".bold().bright_red(),
                name
            );
            return -1;
        }
    };
    println!("{} {}", "Summary for".bright_white().bold(), name.bold().bright_green());
    println!();
    println!("{}: {message}", "Explanation".bold());
    println!("{}: {}", "Default Level".bold(), default_level.to_str());
    println!();
    println!(
        "{}",
        match duck.config().get_lint_level_setting(&name, default_level) {
            LintLevelSetting::Default(_) => "The current directory is using the default level for this lint.".into(),
            LintLevelSetting::ConfigSpecified(level) => format!(
                "This lint is set to `{}` due to your configuration file.",
                level.to_str()
            ),
        }
    );
    0
}

fn create_duck(current_directory: &Path) -> (Duck, ConfigUsage) {
    if let Ok(text) = std::fs::read_to_string(current_directory.join(".duck.toml")) {
        match toml::from_str::<Config>(&text) {
            Ok(config) => (Duck::new(config), ConfigUsage::Some),
            Err(e) => (Duck::default(), ConfigUsage::Failed(e)),
        }
    } else {
        (Duck::default(), ConfigUsage::None)
    }
}

#[derive(Debug)]
enum ConfigUsage {
    None,
    Some,
    Failed(toml::de::Error),
}
