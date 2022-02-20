use std::path::PathBuf;

use colored::Colorize;
use duck::lints::{
    AndKeyword, AnonymousConstructor, ConstructorWithoutNew, DrawSprite, DrawText, Exit, Global,
    Globalvar, MissingCaseMember, MissingDefaultCase, ModKeyword, NoSpaceBeginingComment,
    NonPascalCase, NonScreamCase, OrKeyword, RoomGoto, ShowDebugMessage, SingleSwitchCase, Todo,
    TooManyArguments, TooManyLines, TryCatch, WithLoop,
};
use duck::parsing::expression::{AccessScope, Expression};
use duck::parsing::statement::Statement;
use duck::{Duck, Lint, LintLevel};
use duck::{DuckConfig, LintReport, Position};
use enum_map::{enum_map, EnumMap};
use strum::IntoEnumIterator;
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

fn lint_statement(
    duck: &Duck,
    statement: &Statement,
    position: &Position,
    reports: &mut Vec<LintReport>,
) {
    // Run every lint...
    AndKeyword::visit_statement(duck, statement, position, reports);
    AnonymousConstructor::visit_statement(duck, statement, position, reports);
    ConstructorWithoutNew::visit_statement(duck, statement, position, reports);
    DrawSprite::visit_statement(duck, statement, position, reports);
    DrawText::visit_statement(duck, statement, position, reports);
    Exit::visit_statement(duck, statement, position, reports);
    Global::visit_statement(duck, statement, position, reports);
    Globalvar::visit_statement(duck, statement, position, reports);
    MissingCaseMember::visit_statement(duck, statement, position, reports);
    MissingDefaultCase::visit_statement(duck, statement, position, reports);
    ModKeyword::visit_statement(duck, statement, position, reports);
    NoSpaceBeginingComment::visit_statement(duck, statement, position, reports);
    NonPascalCase::visit_statement(duck, statement, position, reports);
    NonScreamCase::visit_statement(duck, statement, position, reports);
    OrKeyword::visit_statement(duck, statement, position, reports);
    RoomGoto::visit_statement(duck, statement, position, reports);
    ShowDebugMessage::visit_statement(duck, statement, position, reports);
    SingleSwitchCase::visit_statement(duck, statement, position, reports);
    Todo::visit_statement(duck, statement, position, reports);
    TooManyArguments::visit_statement(duck, statement, position, reports);
    TooManyLines::visit_statement(duck, statement, position, reports);
    TryCatch::visit_statement(duck, statement, position, reports);
    WithLoop::visit_statement(duck, statement, position, reports);

    // Recurse...
    match statement {
        Statement::MacroDeclaration(_, _, _) => {}
        Statement::EnumDeclaration(_, members) => {
            members.iter().flat_map(|(_, i)| i).for_each(|member| {
                lint_expression(duck, &*member, position, reports);
            });
        }
        Statement::GlobalvarDeclaration(_) => {}
        Statement::LocalVariableSeries(members) => {
            for member in members {
                lint_expression(duck, &*member, position, reports);
            }
        }
        Statement::TryCatch(try_stmt, condition, catch_stmt) => {
            lint_statement(duck, &*try_stmt, position, reports);
            lint_expression(duck, &*condition, position, reports);
            lint_statement(duck, &*catch_stmt, position, reports);
        }
        Statement::For(initializer, condition, tick, body) => {
            lint_statement(duck, &*initializer, position, reports);
            lint_expression(duck, &*condition, position, reports);
            lint_statement(duck, &*tick, position, reports);
            lint_statement(duck, &*body, position, reports);
        }
        Statement::With(expression, body) => {
            lint_expression(duck, &*expression, position, reports);
            lint_statement(duck, &*body, position, reports);
        }
        Statement::Repeat(expression, body) => {
            lint_expression(duck, &*expression, position, reports);
            lint_statement(duck, &*body, position, reports);
        }
        Statement::DoUntil(body, condition) => {
            lint_expression(duck, &*condition, position, reports);
            lint_statement(duck, &*body, position, reports);
        }
        Statement::While(condition, body) => {
            lint_expression(duck, &*condition, position, reports);
            lint_statement(duck, &*body, position, reports);
        }
        Statement::If(condition, body, else_branch) => {
            lint_expression(duck, &*condition, position, reports);
            lint_statement(duck, &*body, position, reports);
            if let Some(else_branch) = else_branch {
                lint_statement(duck, &*else_branch, position, reports);
            }
        }
        Statement::Switch(identity, cases, default) => {
            lint_expression(duck, &*identity, position, reports);
            for case in cases {
                lint_expression(duck, &*case.0, position, reports);
                for statement in case.1.iter() {
                    lint_statement(duck, &*statement, position, reports);
                }
            }
            if let Some(default) = default {
                for statement in default.iter() {
                    lint_statement(duck, &*statement, position, reports);
                }
            }
        }
        Statement::Block(statements) => {
            for statement in statements {
                lint_statement(duck, &*statement, position, reports);
            }
        }
        Statement::Return(value) => {
            if let Some(value) = value {
                lint_expression(duck, &*value, position, reports);
            }
        }
        Statement::Break => {}
        Statement::Continue => {}
        Statement::Exit => {}
        Statement::Expression(expression) => {
            lint_expression(duck, &*expression, position, reports);
        }
    }
}

fn lint_expression(
    duck: &Duck,
    expression: &Expression,
    position: &Position,
    reports: &mut Vec<LintReport>,
) {
    // Run every lint...
    AndKeyword::visit_expression(duck, expression, position, reports);
    AnonymousConstructor::visit_expression(duck, expression, position, reports);
    ConstructorWithoutNew::visit_expression(duck, expression, position, reports);
    DrawSprite::visit_expression(duck, expression, position, reports);
    DrawText::visit_expression(duck, expression, position, reports);
    Exit::visit_expression(duck, expression, position, reports);
    Global::visit_expression(duck, expression, position, reports);
    Globalvar::visit_expression(duck, expression, position, reports);
    MissingCaseMember::visit_expression(duck, expression, position, reports);
    MissingDefaultCase::visit_expression(duck, expression, position, reports);
    ModKeyword::visit_expression(duck, expression, position, reports);
    NoSpaceBeginingComment::visit_expression(duck, expression, position, reports);
    NonPascalCase::visit_expression(duck, expression, position, reports);
    NonScreamCase::visit_expression(duck, expression, position, reports);
    OrKeyword::visit_expression(duck, expression, position, reports);
    RoomGoto::visit_expression(duck, expression, position, reports);
    ShowDebugMessage::visit_expression(duck, expression, position, reports);
    SingleSwitchCase::visit_expression(duck, expression, position, reports);
    Todo::visit_expression(duck, expression, position, reports);
    TooManyArguments::visit_expression(duck, expression, position, reports);
    TooManyLines::visit_expression(duck, expression, position, reports);
    TryCatch::visit_expression(duck, expression, position, reports);
    WithLoop::visit_expression(duck, expression, position, reports);

    // Recurse...
    match expression {
        Expression::FunctionDeclaration(_, parameters, constructor, body, _) => {
            for parameter in parameters.iter() {
                if let Some(default_value) = &parameter.1 {
                    lint_expression(duck, &*default_value, position, reports);
                }
            }
            if let Some(Some(inheritance_call)) = constructor.as_ref().map(|c| &c.0) {
                lint_expression(duck, &*inheritance_call, position, reports);
            }
            lint_statement(duck, &*body, position, reports);
        }
        Expression::Logical(left, _, right)
        | Expression::Equality(left, _, right)
        | Expression::Evaluation(left, _, right)
        | Expression::Assignment(left, _, right)
        | Expression::NullCoalecence(left, right) => {
            lint_expression(duck, &*left, position, reports);
            lint_expression(duck, &*right, position, reports);
        }
        Expression::Ternary(condition, left, right) => {
            lint_expression(duck, &*condition, position, reports);
            lint_expression(duck, &*left, position, reports);
            lint_expression(duck, &*right, position, reports);
        }
        Expression::Unary(_, right) => {
            lint_expression(duck, &*right, position, reports);
        }
        Expression::Postfix(left, _) => {
            lint_expression(duck, &*left, position, reports);
        }
        Expression::Access(expression, access) => {
            lint_expression(duck, &*expression, position, reports);
            match access {
                AccessScope::Dot(other) => {
                    lint_expression(duck, &*other, position, reports);
                }
                AccessScope::Array(x, y, _) => {
                    lint_expression(duck, &*x, position, reports);
                    if let Some(y) = y {
                        lint_expression(duck, &*y, position, reports);
                    }
                }
                AccessScope::Map(key) => {
                    lint_expression(duck, &*key, position, reports);
                }
                AccessScope::Grid(x, y) => {
                    lint_expression(duck, &*x, position, reports);
                    lint_expression(duck, &*y, position, reports);
                }
                AccessScope::List(index) => {
                    lint_expression(duck, &*index, position, reports);
                }
                AccessScope::Struct(key) => {
                    lint_expression(duck, &*key, position, reports);
                }
                AccessScope::Global | AccessScope::Current => {}
            }
        }
        Expression::Call(left, arguments, _) => {
            lint_expression(duck, &*left, position, reports);
            for arg in arguments {
                lint_expression(duck, &*arg, position, reports);
            }
        }
        Expression::Grouping(expression) => {
            lint_expression(duck, &*expression, position, reports);
        }
        Expression::Literal(_) | Expression::Identifier(_) => {}
    }
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
                lint_statement(duck, &*statement, &Position::default(), &mut reports);
            }),
            Err(error) => error!(target: &error.position().file_string, "{}", error),
        }
    }
    reports
}
