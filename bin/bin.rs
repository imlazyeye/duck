use colored::Colorize;
use duck::{Duck, ParseError, GmlSwitchStatementDefault, Lint, LintLevel};
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

    let mut duck = Duck::new();
    let mut lint_counts: EnumMap<LintLevel, usize> = enum_map! {
        LintLevel::Allow => 0,
        LintLevel::Warn => 0,
        LintLevel::Deny => 0,
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
        if let Err(error) = duck.parse_gml(&gml_file, &path) {
            match error {
                ParseError::UnexpectedToken(cursor, token) => {
                    let target = Duck::create_file_position_string(
                        &gml_file,
                        path.to_str().unwrap(),
                        cursor,
                    );
                    error!(target: &target.file_string, "Unexpected token: {:?}", token)
                }
                ParseError::ExpectedToken(token) => {
                    error!("Expected token: {:?}", token)
                }
                ParseError::UnexpectedEnd => {
                    error!(target: path.to_str().unwrap(), "Unexpected end.")
                }
                ParseError::InvalidLintLevel(cursor, level) => {
                    let target = Duck::create_file_position_string(
                        &gml_file,
                        path.to_str().unwrap(),
                        cursor,
                    );
                    error!(target: &target.file_string, "Invalid lint level: {:?}", level)
                }
                ParseError::InvalidLint(cursor, level) => {
                    let target = Duck::create_file_position_string(
                        &gml_file,
                        path.to_str().unwrap(),
                        cursor,
                    );
                    error!(target: &target.file_string, "Invalid lint: {:?}", level)
                }
            }
        }
    }

    // Validate every switch statement
    for switch in duck.switches() {
        match switch.default_case() {
            GmlSwitchStatementDefault::TypeAssert(type_name) => {
                if let Some(gml_enum) = duck.find_enum_by_name(type_name) {
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
                        duck.report_lint(
                            Lint::MissingCaseMembers,
                            switch.position(),
                            missing_members.join(", "),
                            &mut lint_counts,
                        );
                    }
                } else {
                    duck.report_lint(
                        Lint::UnrecognizedEnum,
                        switch.position(),
                        type_name.clone(),
                        &mut lint_counts,
                    );
                }
            }
            GmlSwitchStatementDefault::None => {
                duck.report_lint(
                    Lint::MissingDefaultCase,
                    switch.position(),
                    "".into(),
                    &mut lint_counts,
                );
            }
            GmlSwitchStatementDefault::Some => {}
        }
    }

    // Yell about illegal characters
    for illegal_char in duck.keywords() {
        match illegal_char {
            duck::GmlKeywords::And(position) => duck.report_lint(
                Lint::AndKeyword,
                position,
                "`and` should be `&&`".to_string(),
                &mut lint_counts,
            ),
            duck::GmlKeywords::Or(position) => duck.report_lint(
                Lint::OrKeyword,
                position,
                "`or` should be `||`".to_string(),
                &mut lint_counts,
            ),
        };
    }

    // Yell about improper macros
    for mac in duck.macros() {
        let name = mac.name();
        let ideal_name = Duck::scream_case(name);
        if name != ideal_name {
            duck.report_lint(
                Lint::NonScreamCase,
                mac.position(),
                format!("`{name}` should be `{ideal_name}`"),
                &mut lint_counts,
            );
        }
    }

    // Yell about improper enums
    for e in duck.enums() {
        let name = e.name();
        let ideal_name = Duck::pascal_case(name);
        if name != ideal_name {
            duck.report_lint(
                Lint::NonPascalCase,
                e.position(),
                format!("`{name}` should be `{ideal_name}`"),
                &mut lint_counts,
            );
        }
    }

    // Yell about improper constructors
    for constructor in duck.constructors() {
        if constructor.is_anonymous() {
            duck.report_lint(
                Lint::AnonymousConstructor,
                constructor.position(),
                "".into(),
                &mut lint_counts,
            );
        } else {
            let name = constructor.name().unwrap();
            let ideal_name = Duck::pascal_case(name);
            if name != &ideal_name {
                duck.report_lint(
                    Lint::NonPascalCase,
                    constructor.position(),
                    format!("`{name}` should be `{ideal_name}`"),
                    &mut lint_counts,
                );
            }
        }
    }

    // Yell about comments
    for comment in duck.comments() {
        // Seek out that space
        for c in comment.body().chars() {
            match c {
                '/' | '*' => {}
                ' ' => {
                    break;
                }
                _ => {
                    duck.report_lint(
                        Lint::NoSpaceAtStartOfComment,
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
