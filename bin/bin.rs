use clippie::{Clippie, ClippieIssue, ClippieLevel, GmlSwitchStatementDefault};
use colored::Colorize;
use enum_map::{enum_map, EnumMap};
use yy_boss::{Resource, YyResource, YypBoss};

#[macro_use]
extern crate log;

fn main() {
    let boss = YypBoss::with_startup_injest(
        "../SwordAndField/FieldsOfMistria.yyp",
        &[Resource::Script, Resource::Object],
    )
    .unwrap();

    let mut clippie = Clippie::new();
    let mut lint_counts: EnumMap<ClippieLevel, usize> = enum_map! {
        ClippieLevel::Allow => 0,
        ClippieLevel::Warn => 0,
        ClippieLevel::Deny => 0,
    };

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
        if let Err(error) = clippie.parse_gml(&gml_file, &path) {
            match error {
                clippie::ClippieParseError::UnexpectedToken(cursor, token) => {
                    let target = Clippie::create_file_position_string(
                        &gml_file,
                        path.to_str().unwrap(),
                        cursor,
                    );
                    error!(target: &target.file_string, "Unexpected token: {:?}", token)
                }
                clippie::ClippieParseError::ExpectedToken(token) => {
                    error!("Expected token: {:?}", token)
                }
                clippie::ClippieParseError::UnexpectedEnd => {
                    error!(target: path.to_str().unwrap(), "Unexpected end.")
                }
                clippie::ClippieParseError::InvalidClippieLevel(cursor, level) => {
                    let target = Clippie::create_file_position_string(
                        &gml_file,
                        path.to_str().unwrap(),
                        cursor,
                    );
                    error!(target: &target.file_string, "Invalid Clippie level: {:?}", level)
                }
                clippie::ClippieParseError::InvalidClippieIssue(cursor, level) => {
                    let target = Clippie::create_file_position_string(
                        &gml_file,
                        path.to_str().unwrap(),
                        cursor,
                    );
                    error!(target: &target.file_string, "Invalid Clippie issue: {:?}", level)
                }
            }
        }
    }

    // Validate every switch statement
    for switch in clippie.switches() {
        match switch.default_case() {
            GmlSwitchStatementDefault::TypeAssert(type_name) => {
                if let Some(gml_enum) = clippie.find_enum_by_name(type_name) {
                    let mut missing_members = vec![];
                    for member in gml_enum.iter_constructed_names() {
                        if member.contains(".Len") || member.contains(".LEN") {
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
                            switch.position(),
                            missing_members.join(", "),
                            &mut lint_counts,
                        );
                    }
                } else {
                    clippie.raise_issue(
                        ClippieIssue::UnrecognizedEnum,
                        switch.position(),
                        type_name.clone(),
                        &mut lint_counts,
                    );
                }
            }
            GmlSwitchStatementDefault::None => {
                clippie.raise_issue(
                    ClippieIssue::MissingDefaultCase,
                    switch.position(),
                    "".into(),
                    &mut lint_counts,
                );
            }
            GmlSwitchStatementDefault::Some => {}
        }
    }

    // Yell about illegal characters
    for illegal_char in clippie.keywords() {
        match illegal_char {
            clippie::GmlKeywords::And(position) => clippie.raise_issue(
                ClippieIssue::AndKeyword,
                position,
                "`and` should be `&&`".to_string(),
                &mut lint_counts,
            ),
            clippie::GmlKeywords::Or(position) => clippie.raise_issue(
                ClippieIssue::OrKeyword,
                position,
                "`or` should be `||`".to_string(),
                &mut lint_counts,
            ),
        };
    }

    // Yell about improper macros
    for mac in clippie.macros() {
        let name = mac.name();
        let ideal_name = Clippie::scream_case(name);
        if name != ideal_name {
            clippie.raise_issue(
                ClippieIssue::NonScreamCase,
                mac.position(),
                format!("`{name}` should be `{ideal_name}`"),
                &mut lint_counts,
            );
        }
    }

    // Yell about improper enums
    for e in clippie.enums() {
        let name = e.name();
        let ideal_name = Clippie::pascal_case(name);
        if name != ideal_name {
            clippie.raise_issue(
                ClippieIssue::NonPascalCase,
                e.position(),
                format!("`{name}` should be `{ideal_name}`"),
                &mut lint_counts,
            );
        }
    }

    // Yell about improper constructors
    for constructor in clippie.constructors() {
        if constructor.is_anonymous() {
            clippie.raise_issue(
                ClippieIssue::AnonymousConstructor,
                constructor.position(),
                "".into(),
                &mut lint_counts,
            );
        } else {
            let name = constructor.name().unwrap();
            let ideal_name = Clippie::pascal_case(name);
            if name != &ideal_name {
                clippie.raise_issue(
                    ClippieIssue::NonPascalCase,
                    constructor.position(),
                    format!("`{name}` should be `{ideal_name}`"),
                    &mut lint_counts,
                );
            }
        }
    }

    // Yell about comments
    for comment in clippie.comments() {
        // Seek out that space
        for c in comment.body().chars() {
            match c {
                '/' | '*' => {}
                ' ' => {
                    break;
                }
                _ => {
                    clippie.raise_issue(
                        ClippieIssue::NoSpaceAtStartOfComment,
                        comment.position(),
                        "".into(),
                        &mut lint_counts,
                    );
                }
            }
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
