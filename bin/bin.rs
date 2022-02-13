use clippie::{Clippie, ClippieIssue, ClippieLevel, GmlSwitchStatementDefault};
use colored::Colorize;
use enum_map::{enum_map, EnumMap};
extern crate log;

fn main() {
    pretty_env_logger::formatted_builder()
        .format_module_path(true)
        .filter(None, log::LevelFilter::Trace)
        .init();

    color_eyre::install().unwrap();

    let clippie = Clippie::new("../SwordAndField/FieldsOfMistria.yyp");
    let mut lint_counts: EnumMap<ClippieLevel, usize> = enum_map! {
        ClippieLevel::Allow => 0,
        ClippieLevel::Warn => 0,
        ClippieLevel::Deny => 0,
    };

    // Validate every switch statement
    for switch in clippie.switches() {
        match switch.default_case() {
            GmlSwitchStatementDefault::TypeAssert(type_name) => {
                if let Some(gml_enum) = clippie.find_enum_by_name(type_name) {
                    let mut missing_members = vec![];
                    for member in gml_enum.iter_constructed_names() {
                        if member.contains(".Len") {
                            // A special case...
                            continue;
                        }
                        if !switch.cases().contains(&member) {
                            missing_members
                                .push(member[member.find('.').unwrap() + 1..].to_string());
                        }
                    }
                    if !missing_members.is_empty() {
                        clippie.raise_issue(
                            ClippieIssue::MissingCaseMembers,
                            switch.resource_path(),
                            missing_members.join(", "),
                            &mut lint_counts,
                        );
                    }
                } else {
                    clippie.raise_issue(
                        ClippieIssue::UnrecognizedEnum,
                        switch.resource_path(),
                        type_name.clone(),
                        &mut lint_counts,
                    );
                }
            }
            GmlSwitchStatementDefault::None => {
                clippie.raise_issue(
                    ClippieIssue::MissingDefaultCase,
                    switch.resource_path(),
                    "".into(),
                    &mut lint_counts,
                );
            }
            GmlSwitchStatementDefault::Some => {}
        }
    }

    // Print the results
    let output = format!(
        "{} {} {}, {} {}, {} {} {}.",
        "Finished lint with".bold(),
        lint_counts[ClippieLevel::Deny].to_string().bright_red(),
        "errors".bold(),
        lint_counts[ClippieLevel::Warn].to_string().yellow(),
        "warnings".bold(),
        "and".bold(),
        lint_counts[ClippieLevel::Allow].to_string().bright_black(),
        "ignored lints".bold()
    );
    println!(
        "{}",
        String::from_utf8(vec![b'-'; output.len() / 2]).unwrap()
    );
    println!("\n{output}\n");
}
