use std::path::PathBuf;

use colored::Colorize;
use duck::parsing::expression::Expression;
use duck::parsing::statement::Statement;
use duck::{DuckConfig, LintReport, Position};
use duck::{Duck, Lint, LintLevel};
use enum_map::{enum_map, EnumMap};
use yy_boss::{Resource, YyResource, YypBoss};

#[macro_use]
extern crate log;

fn main() {
    pretty_env_logger::formatted_builder()
        .format_module_path(true)
        .init();
    color_eyre::install().unwrap();

    let timer = std::time::Instant::now();
    let _current_directory = std::env::current_dir().expect("Cannot access the current directory!");

    // TEMPORARY
    let current_directory = PathBuf::from("../SwordAndField");

    let mut duck = if let Ok(text) = std::fs::read_to_string(current_directory.join(".duck.toml")) {
        let config: DuckConfig = toml::from_str(&text).unwrap_or_else(|e| {
            error!("Failed to parse `.duck.toml`: {e}");
            info!("Falling back to default settings...");
            DuckConfig::default()
        });
        Duck::new_with_config(config)
    } else {
        Duck::new()
    };
    parse_all_gml(&mut duck);

    let mut lint_counts: EnumMap<LintLevel, usize> = enum_map! {
        LintLevel::Allow => 0,
        LintLevel::Warn => 0,
        LintLevel::Deny => 0,
    };

    // Print the results
    let output = format!(
        "🦆 <( {} {} {}, {} {}, {} {} {}! )",
        format!(
            "Finished lint in {:.2}s with",
            std::time::Instant::now()
                .duration_since(timer)
                .as_secs_f32()
        )
        .bold(),
        lint_counts[LintLevel::Deny].to_string().bright_red(),
        "errors".bold(),
        lint_counts[LintLevel::Warn].to_string().yellow(),
        "warnings".bold(),
        "and".bold(),
        lint_counts[LintLevel::Allow].to_string().bright_black(),
        "ignored lints".bold()
    );
    println!(
        "{}",
        String::from_utf8(vec![b'-'; output.len() / 2]).unwrap()
    );
    println!("\n{output}\n");
}

fn run_lint_for_expression<L: Lint>(
    lint: L,
    duck: &Duck,
    expression: &Expression,
    position: &Position,
    reports: &mut Vec<LintReport>,
    lint_counts: &mut EnumMap<LintLevel, usize>,
) {
    L::visit_expression(duck, expression, position, reports)
}

fn run_lint_for_statement<L: Lint>(
    lint: L,
    duck: &Duck,
    statement: &Statement,
    position: &Position,
    reports: &mut Vec<LintReport>,
    lint_counts: &mut EnumMap<LintLevel, usize>,
) {
    L::visit_statement(duck, statement, position, reports)
}

fn parse_all_gml(duck: &mut Duck) {
    let boss = YypBoss::with_startup_injest(
        "../SwordAndField/FieldsOfMistria.yyp",
        &[Resource::Script, Resource::Object],
    )
    .unwrap();

    // Parse it all
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

    for (gml_file, path) in gml {
        match duck.parse_gml(&gml_file, &path) {
            Ok(ast) => ast.into_iter().for_each(|statement| {
                // run_lint_for_statement(AndKeyword, &duck, &mut lint_counts);
                // run_lint_for_statement(OrKeyword, &duck, &mut lint_counts);
                // run_lint_for_statement(Exit, &duck, &mut lint_counts);
                // run_lint_for_statement(Global, &duck, &mut lint_counts);
                // run_lint_for_statement(Globalvar, &duck, &mut lint_counts);
                // run_lint_for_statement(ModKeyword, &duck, &mut lint_counts);
                // run_lint_for_statement(TryCatch, &duck, &mut lint_counts);
                // run_lint_for_statement(WithLoop, &duck, &mut lint_counts);
                // run_lint_for_statement(AnonymousConstructor, &duck, &mut lint_counts);
                // run_lint_for_statement(ConstructorWithoutNew, &duck, &mut lint_counts);
                // run_lint_for_statement(MissingCaseMember, &duck, &mut lint_counts);
                // run_lint_for_statement(MissingDefaultCase, &duck, &mut lint_counts);
                // run_lint_for_statement(NoSpaceBeginingComment, &duck, &mut lint_counts);
                // run_lint_for_statement(NonPascalCase, &duck, &mut lint_counts);
                // run_lint_for_statement(NonScreamCase, &duck, &mut lint_counts);
            }),
            Err(error) => error!(target: &error.position().file_string, "{}", error),
        }
    }
}
