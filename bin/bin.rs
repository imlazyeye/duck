use colored::Colorize;
use duck::{Duck, LintLevel};
use duck::{DuckConfig, LintReport, Position};
use std::path::PathBuf;
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

    let reports = parse_all_gml(&mut duck);
    let deny_count = reports
        .iter()
        .filter(|v| v.get_true_level(&duck) == LintLevel::Deny)
        .count();
    let warn_count = reports
        .iter()
        .filter(|v| v.get_true_level(&duck) == LintLevel::Warn)
        .count();
    let allow_count = reports
        .iter()
        .filter(|v| v.get_true_level(&duck) == LintLevel::Allow)
        .count();
    for report in reports {
        report.raise(&duck);
    }

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
        deny_count.to_string().bright_red(),
        "errors".bold(),
        warn_count.to_string().yellow(),
        "warnings".bold(),
        "and".bold(),
        allow_count.to_string().bright_black(),
        "ignored lints".bold()
    );
    println!(
        "{}",
        String::from_utf8(vec![b'-'; output.len() / 2]).unwrap()
    );
    println!("\n{output}\n");
}

fn parse_all_gml(duck: &mut Duck) -> Vec<LintReport> {
    let boss = YypBoss::with_startup_injest(
        "../SwordAndField/FieldsOfMistria.yyp",
        &[Resource::Script, Resource::Object],
    )
    .unwrap();

    // Parse it all
    let mut reports = vec![];
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
                duck.lint_statement(&*statement, &Position::default(), &mut reports);
            }),
            Err(error) => error!(target: &error.position().file_string, "{}", error),
        }
    }
    reports
}
