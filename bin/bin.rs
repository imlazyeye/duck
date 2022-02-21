use colored::Colorize;
use duck::{Duck, LintLevel, Position};
use duck::{DuckConfig, LintReport};
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

    let registrar = parse_all_gml(&mut duck);
    let mut deny_count = 0;
    let mut warn_count = 0;
    let mut allow_count = 0;
    for (file, path, reports) in registrar {
        for report in reports {
            match report.get_true_level(&duck) {
                LintLevel::Allow => allow_count += 1,
                LintLevel::Warn => warn_count += 1,
                LintLevel::Deny => deny_count += 1,
            }
            let cursor = report.span.0;
            report.raise(&duck, &Position::new(&file, &path, cursor));
        }
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

fn parse_all_gml(duck: &mut Duck) -> Vec<(String, String, Vec<LintReport>)> {
    let boss = YypBoss::with_startup_injest(
        "../SwordAndField/FieldsOfMistria.yyp",
        &[Resource::Script, Resource::Object],
    )
    .unwrap();

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
        match duck.parse_gml(&gml, &path) {
            Ok(ast) => ast.into_iter().for_each(|statement| {
                let mut reports = vec![];
                duck.lint_statement(statement.statement(), statement.span(), &mut reports);
                registrar.push((gml.to_string(), path.to_str().unwrap().into(), reports));
            }),
            Err(error) => error!("{}", error),
        }
    }
    registrar
}
