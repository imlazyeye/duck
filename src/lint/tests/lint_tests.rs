use crate::{
    analyze::{GlobalScope, GlobalScopeBuilder},
    lint::{collection::*, Lint, LintLevel},
    parse::*,
    Config, DuckOperation, GmlLibrary,
};
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use pretty_assertions::assert_eq;
use unindent::Unindent;

fn config_for_lint<T: Lint>() -> Config {
    let mut config = Config::full();
    config.lint_levels.iter_mut().for_each(|(_, v)| *v = LintLevel::Allow);
    config.lint_levels.insert(T::tag().into(), LintLevel::Deny);
    config
}

fn harness_lint<T: Lint>(source: &str, expected_number: usize) {
    let source = Box::leak(Box::new(source.unindent()));
    let config = config_for_lint::<T>();
    let mut library = GmlLibrary::new();
    let file_id = library.add("test.gml".into(), source);
    let ast = Parser::new(source, file_id).into_ast().unwrap();
    let mut reports = vec![];
    let mut scope_builder = GlobalScopeBuilder::new();
    for statement in ast.statements() {
        DuckOperation::process_statement_early(&config, statement, &mut scope_builder, &mut reports);
    }
    let mut global_scope = GlobalScope::new();
    global_scope.drain(scope_builder);
    for statement in ast.statements() {
        DuckOperation::process_statement_late(&config, statement, &global_scope, &mut reports);
    }
    let writer = StandardStream::stdout(ColorChoice::Always);
    let config = codespan_reporting::term::Config::default();
    if reports.len() != expected_number {
        for report in reports.iter() {
            codespan_reporting::term::emit(&mut writer.lock(), &config, &library, report).unwrap();
        }
        assert_eq!(reports.len(), expected_number);
    }
    Box::leak(Box::new(library));
}

#[test]
fn accessor_alternative() {
    harness_lint::<AccessorAlternative>(
        "
            array_get(foo, index);
            ds_map_find_value(foo, key);
            ds_list_find_value(foo, index);
            ds_grid_get(foo, x, y);
            variable_struct_get(foo, bar);
        ",
        5,
    );
}

#[test]
fn and_preference() {
    harness_lint::<AndPreference>(
        "
            foo = 0 and 1;
            foo = 0 && 1;
        ",
        1,
    );
}

#[test]
fn anonymous_constructor() {
    harness_lint::<AnonymousConstructor>(
        "
            foo = function() constructor {};
        ",
        1,
    );
}

#[test]
fn assignment_to_call() {
    harness_lint::<AssignmentToCall>(
        "
            foo() = bar;
        ",
        1,
    );
}

#[test]
fn bool_equality() {
    harness_lint::<BoolEquality>(
        "
            foo = bar == true;
            foo = bar == false;
        ",
        2,
    );
}

#[test]
fn collapsable_if() {
    harness_lint::<CollapsableIf>(
        "
            if foo {
                if bar {}
            }
        ",
        1,
    );
    harness_lint::<CollapsableIf>(
        "
            if foo {
                if bar {}
                call();
            }
            if foo {
                if bar {} else {}
            }
            if foo { 
                if bar {}
            } else {
                call();
            }
        ",
        0,
    );
}

#[test]
fn deprecated() {
    harness_lint::<Deprecated>(
        "
            globalvar foo;
            foo = bar[0, 0];
            foo = array_height_2d(bar);
        ",
        3,
    );
}

#[test]
fn draw_sprite() {
    harness_lint::<DrawSprite>(
        "
            draw_sprite(foo, 0, x, y);
            draw_sprite_ext(foo, 0, x, y);
        ",
        2,
    );
}

#[test]
fn draw_text() {
    harness_lint::<DrawText>(
        "
            draw_text(foo, x, y);
            draw_text_ext(foo, x, y);
        ",
        2,
    );
}

#[test]
fn english_flavor_violation() {
    harness_lint::<EnglishFlavorViolation>(
        "
            draw_text_colour(foo, x, y, c_white);
            draw_text_color(foo, x, y, c_white);
        ",
        1,
    );
}

#[test]
fn exit() {
    harness_lint::<Exit>(
        "
            exit;
        ",
        1,
    );
}

#[test]
fn global() {
    harness_lint::<Global>(
        "
            global.foo = 0;
            globalvar bar;
        ",
        2,
    );
}

#[test]
fn missing_case_member() {
    harness_lint::<MissingCaseMember>(
        "
            enum Foo {
                Bar,
                Buzz
            }
            switch foo {
                case Foo.Bar: break;
            }
        ",
        1,
    );
    harness_lint::<MissingCaseMember>(
        "
            enum Foo {
                Bar,
                Buzz
            }
            // should not fire
            switch foo {
                case Foo.Bar: break;
                default: break;
            }
        ",
        0,
    );
}

#[test]
fn missing_default_case() {
    harness_lint::<MissingDefaultCase>(
        "
            switch foo {
                case Foo.Bar: break;
            }
        ",
        1,
    );
}

#[test]
fn mod_preference() {
    harness_lint::<ModPreference>(
        "
            foo = bar mod buzz;
            foo = bar % buzz;
        ",
        1,
    );
}

#[test]
fn multi_var_declaration() {
    harness_lint::<MultiVarDeclaration>(
        "
            var b, c;
            var d, e=1;
        ",
        2,
    );
}

#[test]
fn non_constant_default_parameter() {
    harness_lint::<NonConstantDefaultParameter>(
        "
            function(foo=Bar.Buzz) {}
        ",
        1,
    );
    harness_lint::<NonConstantDefaultParameter>(
        "
            function(foo=bar) {} // shouldn't fire
        ",
        0,
    );
}

#[test]
fn not_preference() {
    harness_lint::<NotPreference>(
        "
            foo = not buzz;
            foo = !buzz;
        ",
        1,
    );
}

#[test]
fn or_preference() {
    harness_lint::<OrPreference>(
        "
            foo = bar or buzz;
            foo = bar || buzz;
        ",
        1,
    );
}

#[test]
fn room_goto() {
    harness_lint::<RoomGoto>(
        "
            room_goto(foo);
            room_goto_next();
        ",
        2,
    );
}

#[test]
fn show_debug_message() {
    harness_lint::<ShowDebugMessage>(
        "
            show_debug_message(foo);
        ",
        1,
    );
}

#[test]
fn single_equals_comparison() {
    harness_lint::<SingleEqualsComparison>(
        "
            var a = b = c;
            var a = b + c = d + e;
        ",
        2,
    );
    harness_lint::<SingleEqualsComparison>(
        "
              var a = b == c; // shouldn't fire
        ",
        0,
    );
}

#[test]
fn statement_parenthetical_preference() {
    harness_lint::<StatementParentheticalPreference>(
        "
            if (foo) {}
            if foo {}
        ",
        1,
    );
}

#[test]
fn suspicious_constant_usage() {
    harness_lint::<SuspicousConstantUsage>(
        "
            foo += true; 
            foo = bar && undefined;
            foo += tile_index_mask;
            foo = bar > pointer_null;
            foo += [0, 1, 2];
            foo = [0] && {a: 0};
        ",
        6,
    );
    harness_lint::<SuspicousConstantUsage>(
        "
            foo = 2 - 1;
            foo = undefined;
            foo = bar == undefined;
            foo &= tile_index_mask;
            foo = 0 | tile_index_mask;
            foo = bar >= 0;
            foo = bar != undefined;
        ",
        0,
    );
}

#[test]
fn todo() {
    harness_lint::<Todo>(
        "
            todo()
        ",
        1,
    );
}

#[test]
fn too_many_arguments() {
    harness_lint::<TooManyArguments>(
        "
            function(a, b, c, d, e, f, g, h, i, j, j) {}
        ",
        1,
    );
    harness_lint::<TooManyArguments>(
        "
            function(a) {}
        ",
        0,
    );
}

#[test]
fn try_catch() {
    harness_lint::<crate::lint::collection::TryCatch>(
        "
            try {} catch (e) {}
        ",
        1,
    );
}

#[test]
fn var_prefix_violation() {
    harness_lint::<VarPrefixViolation>(
        "
            var _foo = 0;
            var foo = 0;
        ",
        1,
    );
}

#[test]
fn with_loop() {
    harness_lint::<crate::lint::collection::WithLoop>(
        "
            with foo {}
        ",
        1,
    );
}
