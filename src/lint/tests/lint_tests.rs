use crate::{
    Config, GmlLibrary, driver,
    lint::{Lint, LintLevel, collection::*},
    parse::*,
};
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use pretty_assertions::assert_eq;

fn config_for_lint<T: Lint>() -> Config {
    let mut config = Config::full();
    config.lint_levels.iter_mut().for_each(|(_, v)| *v = LintLevel::Allow);
    config.lint_levels.insert(T::tag().into(), LintLevel::Deny);
    config
}

pub(super) fn harness_lint<T: Lint>(source: &'static str, expected_number: usize) {
    let config = config_for_lint::<T>();
    let mut library = GmlLibrary::new();
    let file_id = library.add("test.gml".into(), source);
    let mut ast = Parser::new_with_default_ids(source, file_id).into_ast().unwrap();
    let mut reports = vec![];
    for stmt in ast.stmts_mut() {
        driver::process_stmt_early(stmt, &mut reports, &config);
    }
    for stmt in ast.stmts() {
        driver::process_stmt_late(stmt, &mut reports, &config);
    }
    let writer = StandardStream::stdout(ColorChoice::Always);
    let config = codespan_reporting::term::Config::default();
    if reports.len() != expected_number {
        for report in reports.iter() {
            codespan_reporting::term::emit(&mut writer.lock(), &config, &library, report).unwrap();
        }
        assert_eq!(
            reports.len(),
            expected_number,
            "{} got the wrong number of lints on `{}`!",
            T::tag(),
            source
        );
    }
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
fn bool_equality() {
    harness_lint::<BoolEquality>(
        "
            foo = bar == true;
            foo = bar == false;
        ",
        2,
    );
}

/// Relying on the definitions in [CasingRules::default()].
/// TODO: restore!
#[test]
fn casing_rules() {
    harness_lint::<CasingRules>(
        "
            var fooBar = 0;
            #macro fooBar 0
            enum fooBar { 
                fooBar
            }
            globalvar fooBar;
            global.fooBar = 0;
            function fooBar() {}
            function fooBar() constructor {}
        ",
        8,
    );
    harness_lint::<CasingRules>(
        "
            var foo_bar = 0;
            #macro FOO_BAR 0
            enum FooBar { 
                FooBar
            }
            globalvar foo_bar;
            global.foo_bar = 0;
            function foo_bar() {}
            function FooBar() constructor {}
        ",
        0,
    );
}

#[test]
fn collapsible_if() {
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
    harness_lint::<CollapsableIf>(
        "
            if foo {
                while bar() {}
            }
        ",
        0,
    );
}

#[test]
fn condition_wrapper() {
    harness_lint::<ConditionWrapper>(
        "
            with foo {}
            repeat foo {}
            while foo {}
            do {} until foo;
            while foo {}
            if foo {}
            switch foo {}
        ",
        7,
    );
    harness_lint::<ConditionWrapper>(
        "
            with (foo) {}
            repeat (foo) {}
            while (foo) {}
            do {} until (foo);
            while (foo) {}
            if (foo) {}
            switch (foo) {}
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
        0,
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
        0,
    );
    harness_lint::<NonConstantDefaultParameter>(
        "
            function(foo=bar) {} // shouldn't fire
        ",
        0,
    );
}

#[test]
fn non_simplified_expression() {
    harness_lint::<NonSimplifiedExpression>(
        r#"
            // Based on default lint preferences
            var a = 1 + 1;
            var b = 1 - 1;
            var c = "a" + "b";
            var d = 1 * 1;
            var e = 1 / 1;
            var f = 1 mod 1;
            var g = 1 div 1;
            var h = 1 | 1;
            var i = 1 & 1;
            var j = 1 << 1;
            var k = 1 >> 1;

            // Now test for positives when groups are involved
            var a = (1) + 1;
            var b = (1) + (1);
            var c = ((1)) + (((1)));
        "#,
        6,
    )
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
fn unassigned_constructor() {
    harness_lint::<UnassignedConstructor>(
        "
            new Foo();
        ",
        1,
    );
    harness_lint::<UnassignedConstructor>(
        "
            Foo();
        ",
        0,
    );
}

#[test]
fn unnecessary_grouping() {
    harness_lint::<UnnecessaryGrouping>(
        "
            foo = (1 + 1);
            delete (1 + 1);
            return (1 + 1);
            throw (1 + 1);
            foo[(1 + 1)]();
            foo((1 + 1))
        ",
        6,
    );
    harness_lint::<UnnecessaryGrouping>(
        "
            // Stylistic, thereof not a part of this lint (see `condition_wrapper`)
            do {} until (foo);
            if (foo) {}
            repeat (foo) {}
            switch (foo) {}
            try {} catch(foo) {}
            while (foo) {}
            with (foo) {}

            // Fine in general
            foo = bar && (buzz || pazz) ? 1 : 0;
            foo = -(1 + 1);
            foo = 1 && !(bar);
            foo = 1 && (2 || 3);
            foo = 1 + (bar);
            foo = 1 == (true);
            foo = (1 + 1) / 2;
            foo = ((1 + 1) / 2) - 1;
        ",
        0,
    );
}

#[test]
fn useless_function() {
    harness_lint::<UselessFunction>(
        "
            function() {}
        ",
        1,
    );
    harness_lint::<UselessFunction>(
        "
            function foo() {}
            var foo = function() {}
        ",
        0,
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
