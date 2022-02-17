use std::collections::HashMap;

use colored::Colorize;
use duck::{lints::*, DuckConfig};
use duck::{Duck, Lint, LintLevel, ParseError, Position};
use enum_map::{enum_map, EnumMap};
use yy_boss::{Resource, YyResource, YypBoss};

#[macro_use]
extern crate log;

fn main() {
    let mut lint_levels = HashMap::new();
    lint_levels.insert("and_keyword".to_string(), LintLevel::Allow);
    lint_levels.insert("or_keyword".to_string(), LintLevel::Allow);
    lint_levels.insert("with_keyword".to_string(), LintLevel::Allow);
    lint_levels.insert("no_space_begining_comment".to_string(), LintLevel::Allow);
    let mut duck = Duck::new_with_config(DuckConfig { lint_levels });
    parse_all_gml(&mut duck);

    let mut lint_counts: EnumMap<LintLevel, usize> = enum_map! {
        LintLevel::Allow => 0,
        LintLevel::Warn => 0,
        LintLevel::Deny => 0,
    };

    run_lint(AndKeyword, &duck, &mut lint_counts);
    run_lint(OrKeyword, &duck, &mut lint_counts);
    run_lint(Exit, &duck, &mut lint_counts);
    run_lint(Global, &duck, &mut lint_counts);
    run_lint(Globalvar, &duck, &mut lint_counts);
    run_lint(ModKeyword, &duck, &mut lint_counts);
    run_lint(TryCatch, &duck, &mut lint_counts);
    run_lint(WithLoop, &duck, &mut lint_counts);
    run_lint(AnonymousConstructor, &duck, &mut lint_counts);
    run_lint(ConstructorWithoutNew, &duck, &mut lint_counts);
    run_lint(MissingCaseMember, &duck, &mut lint_counts);
    run_lint(MissingDefaultCase, &duck, &mut lint_counts);
    run_lint(NoSpaceBeginingComment, &duck, &mut lint_counts);
    run_lint(NonPascalCase, &duck, &mut lint_counts);
    run_lint(NonScreamCase, &duck, &mut lint_counts);

    // Print the results
    let output = format!(
        "🦆 <( {} {} {}, {} {}, {} {} {}! )",
        "Finished lint with".bold(),
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

fn run_lint<L: Lint>(lint: L, duck: &Duck, lint_counts: &mut EnumMap<LintLevel, usize>) {
    L::run(duck)
        .into_iter()
        .for_each(|r| duck.report_lint(&lint, r, lint_counts));
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
        if let Err(error) = duck.parse_gml(&gml_file, &path) {
            match error {
                ParseError::UnexpectedToken(cursor, token) => {
                    let target = Position::new(&gml_file, path.to_str().unwrap(), cursor);
                    error!(target: &target.file_string, "Unexpected token: {:?}", token)
                }
                ParseError::ExpectedToken(token) => {
                    error!("Expected token: {:?}", token)
                }
                ParseError::UnexpectedEnd => {
                    error!(target: path.to_str().unwrap(), "Unexpected end.")
                }
                ParseError::InvalidLintLevel(cursor, level) => {
                    let target = Position::new(&gml_file, path.to_str().unwrap(), cursor);
                    error!(target: &target.file_string, "Invalid lint level: {:?}", level)
                }
            }
        }
    }
}
