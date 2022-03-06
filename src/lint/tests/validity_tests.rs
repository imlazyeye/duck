use crate::{
    lint::{
        collection::{InvalidAssignment, InvalidComparison, InvalidEquality},
        tests::lint_tests::harness_lint,
        Lint,
    },
    parse::Parser,
};

fn harness_valid(source: &'static str) {
    let parser = Parser::new(source, 0).into_ast();
    assert!(
        parser.map_or(false, |v| v.statements().len() == 1),
        "`{}` was invalid!",
        source
    )
}

fn harness_valid_but_linted<T: Lint>(source: &'static str) {
    let parser = Parser::new(source, 0).into_ast();
    assert!(
        parser.map_or(false, |v| v.statements().len() == 1),
        "`{}` was invalid!",
        source
    );
    harness_lint::<T>(source, 1);
}

fn harness_invalid(source: &'static str) {
    let parser = Parser::new(source, 0).into_ast();
    assert!(
        parser.map_or(true, |v| v.statements().len() != 1),
        "`{}` was valid!",
        source
    )
}

/// The following are tests assert the validity of normal assignments. They do *not* assert
/// non-valid assignments, because those are all technically legal in gml due to how their parser
/// recurses through the AST (and subsequently, ours as well). The lint `invalid_assignment_target`
/// will take care of those!
#[test]
fn assignment_targets() {
    harness_valid("a.b = 1;");
    harness_valid("a[0] = 1;");
    harness_valid("a = 1;");
    harness_valid("++a = 1;"); // technically ++(a = 1)
    harness_valid_but_linted::<InvalidAssignment>("a() = 1;");
    harness_valid_but_linted::<InvalidAssignment>("function() {} = 1;");
    harness_valid_but_linted::<InvalidAssignment>("(a) = 1;");
    harness_valid_but_linted::<InvalidAssignment>("true = 1;");
    harness_valid_but_linted::<InvalidAssignment>("a++ = 1;");

    // Invalid in both us and gml, because we both output two different statements for this:
    // ```gml
    // a;
    // +b = 1;
    // ```
    harness_invalid("a + b = 1;");
}

#[test]
fn access_targets() {
    harness_valid("a = b.c.d;");
    harness_valid("a = b().c;");
    harness_valid("a = b.c;");
    harness_valid("a = ++b.c;");
    harness_invalid("a = (b).c;");
    harness_invalid("a = function(){}.b;");
    harness_invalid("a = true.b;");
    harness_invalid("a = b++.c;");
}

#[test]
fn call_targets() {
    harness_valid("a.b();");
    harness_valid("a()();");
    harness_valid("a();");
    harness_valid("++a();");
    harness_invalid("(a)();");
    harness_invalid("function(){}();");
    harness_invalid("true();");
    harness_invalid("a = b++();");
}

#[test]
fn equality_targets() {
    harness_valid("a = b.c == c;");
    harness_valid("a = b() == c;");
    harness_valid("a = b == c");
    harness_valid("a = ++b == c;");
    harness_valid("a = (b) == c;");
    harness_valid("a = true == b;");
    harness_valid("a = b++ == b;");
    harness_valid_but_linted::<InvalidEquality>("a = function(){} == b;");
}

#[test]
fn evaluation_targets() {
    harness_valid("a = b.c + c;");
    harness_valid("a = b() + c;");
    harness_valid("a = b + c");
    harness_valid("a = ++b + c;");
    harness_valid("a = (b) + c;");
    harness_valid("a = true + b;");
    harness_valid("a = b++ + b;");
    // harness_valid_but_linted::<InvalidEvaluation>("a = function(){} + b;");
}

#[test]
fn logical_targets() {
    harness_valid("a = b.c && c;");
    harness_valid("a = b() && c;");
    harness_valid("a = b && c");
    harness_valid("a = ++b && c;");
    harness_valid("a = (b) && c;");
    harness_valid("a = true && b;");
    harness_valid("a = b++ && b;");
    harness_valid_but_linted::<InvalidComparison>("a = function(){} && b;");
}
