use crate::{analyze::*, new_array, new_function, new_struct, parse::*};
use colored::Colorize;
use pretty_assertions::assert_eq;
use Type::*;

pub(super) struct TestTypeWriter(Typewriter);
impl std::ops::Deref for TestTypeWriter {
    type Target = Typewriter;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Drop for TestTypeWriter {
    fn drop(&mut self) {
        Printer::flush()
    }
}

pub(super) fn get_type(source: &'static str) -> Type {
    let source = Box::leak(Box::new(format!("var a = {source}")));
    let (typewriter, scope) = harness_typewriter(source);
    scope.lookup_type(&Identifier::lazy("a"), &typewriter).unwrap()
}

pub(super) fn get_var_type(source: &'static str, name: &'static str) -> Type {
    let (typewriter, scope) = harness_typewriter(source);
    scope.lookup_type(&Identifier::lazy(name), &typewriter).unwrap()
}

pub(super) fn harness_typewriter(source: &str) -> (TestTypeWriter, Scope) {
    let source = Box::leak(Box::new(source.to_string()));
    let parser = Parser::new(source, 0);
    let mut typewriter = Typewriter::default();
    let mut ast = parser.into_ast().unwrap();
    typewriter.write(ast.stmts_mut());
    let scope = typewriter.scope.clone();
    println!("Result for: \n{source}");
    for name in scope.local_fields().iter() {
        let str = name.bright_black();
        let tpe = scope.lookup_type(&Identifier::lazy(name), &typewriter).unwrap();
        let whitespace = String::from_utf8(vec![b' '; 75 - str.len()]).unwrap();
        println!("{str}{whitespace}{}\n", Printer::tpe(&tpe).bright_cyan().bold());
    }
    (TestTypeWriter(typewriter), scope)
}

macro_rules! test_expr_type {
    ($name:ident, $src:expr => $should_be:expr) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            assert_eq!(get_type($src), $should_be);
        }
    };
    ($name:ident, $($src:expr => $should_be:expr), * $(,)?) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            $(assert_eq!(get_type($src), $should_be);)*
        }
    };
}

macro_rules! test_var_type {
    ($name:ident, $src:expr, $var:ident: $should_be:expr) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            assert_eq!(get_var_type($src, stringify!($var)), $should_be);
        }
    };
    ($name:ident, $src:expr, $($var:ident: $should_be:expr), * $(,)?) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            $(assert_eq!(get_var_type($src, stringify!($var)), $should_be);)*
        }
    };
}

// Basic expressions
test_expr_type!(undefined, "undefined" => Undefined);
test_expr_type!(bools, "true" => Bool, "false" => Bool);
test_expr_type!(real, "1" => Real, "0.1" => Real);
test_expr_type!(real_float, "0.1" => Real);
test_expr_type!(string, "\"foo\"" => Str);
test_expr_type!(grouping, "(0)" => Real);
test_expr_type!(postfix, "x++" => Real, "x--" => Real);
test_expr_type!(
    unary,
    "++x" => Real,
    "--x" => Real,
    "+x" => Real,
    "-x" => Real,
    "~x" => Real,
    "!x" => Bool,
);
test_expr_type!(ternary, "true ? 0 : 0" => Real);
// test_expr_type!(null_coalecence, "undefined ?? 0" => Real);
test_expr_type!(
    evaluation,
    "1 + 1" => Real,
    r#""foo" + "bar""# => Str,
    "1 * 1" => Real,
    "1 / 1" => Real,
    "1 % 1" => Real,
    "1 div 1" => Real,
);
test_expr_type!(logical, "true && false" => Bool);

// Arrays
test_expr_type!(empty_array, "[]" => new_array!(Any));
test_expr_type!(constant_array, "[0]" => new_array!(Real));
test_expr_type!(nested_array, "[[[0]]]" => new_array!(new_array!(new_array!(Real))));
test_var_type!(array_access, "var x = [0], y = x[0];", y: Real,);

// Structs
test_expr_type!(empty_struct, "{}" => new_struct!());
test_expr_type!(populated_struct, "{ x: 0 }" => new_struct!(x: Real));
test_var_type!(struct_access, "var foo = { x: 0 }, bar = foo.x;", bar: Real,);

// Functions
test_expr_type!(function, "function() {}" => new_function!(() => Undefined));
test_var_type!(
    default_return,
    "var foo = function() {};
    var bar = foo();",
    bar: Undefined,
);
test_var_type!(
    return_constant,
    "var foo = function() { return 0; };
    var bar = foo();",
    bar: Real,
);
test_var_type!(
    return_generic,
    "var foo = function(x) { return x; };
    var bar = foo(true);",
    bar: Bool,
);
test_var_type!(
    return_generic_array,
    "var foo = function(x) { return x[0]; };
    var bar = foo([0]);",
    bar: Real,
);
test_var_type!(
    return_generic_struct,
    "var foo = function(x) { return x.y; };
    var bar = foo({ y: 0 });",
    bar: Real,
);
test_var_type!(
    return_other_function_return,
    "var wrapper = function(lambda) {
        return lambda(0);
    }
    var inner = function(n) { return n; }
    var data = wrapper(inner);",
    data: Real,
);
test_var_type!(
    return_advanced_generic,
    r#"var foo = function(a, b) {
        return a[b];
    }
    var bar = function(x, y) { 
        return x + y * 2; 
    }
    var fizz = foo(["hello"], 0);
    var buzz = foo([ { a: true } ], bam(1, 2));"#,
    fizz: Str,
    buzz: new_struct!(a: Bool)
);
test_var_type!(
    multi_use_generic_function,
    "var foo = function(a) {
        return a;
    }
    var bar = foo(true);
    var fizz = foo(0);",
    bar: Bool,
    fizz: Real,
);
test_expr_type!(
    infer_function_in_parameters,
    "function(x) { return x() + 1; }" => new_function!(
        (new_function!(() => Real)) => Real
    )
);
test_expr_type!(
    infer_array_in_parameters,
    "function(x) { return x[0] + 1; }" => new_function!(
        (new_array!(Real)) => Real
    )
);
test_expr_type!(
    infer_struct_in_parameters,
    "function(x) { return x.y + 1; }" => new_function!(
        (new_struct!(y: Real)) => Real
    )
);
test_var_type!(
    mutate_struct_via_function,
    "var foo = function(a) {
        a.a = 0;
    }
    var bar = {};
    foo(bar);",
    bar: new_struct!(a: Real)
);
test_var_type!(
    retain_all_fields_in_generic_call,
    "var foo = function(a) {
        a.a = 0;
        return a;
    }
    var bar = { a: 0, b: 0 };
    foo(bar);",
    bar: new_struct!(a: Real, b: Real)
);
test_var_type!(
    retain_all_fields_in_generic_call_after_return,
    "var foo = function(a) {
        a.a = 0;
        return a;
    }
    var bar = { a: 0, b: 0 };
    bar = foo(bar);",
    bar: new_struct!(a: Real, b: Real)
);

// Self
test_var_type!(self_assignment_no_keyword, "foo = 0;", foo: Real);
test_var_type!(self_assignment_with_keyword, "self.foo = 0;", foo: Real);
test_expr_type!(
    function_write_constant_to_self,
    "function() { self.a = 0; }" => new_function!(
        (self: new_struct!(a: Real)) => Undefined
    ),
);
test_expr_type!(
    function_write_parameter_to_self,
    "function(x) { self.a = x + 1; }" => new_function!(
        (self: new_struct!(a: Real), Real) => Undefined
    ),
);
test_var_type!(
    mutate_self_via_function,
    "var foo = function() {
        self.a = 0;
    }
    foo();",
    a: Real,
);
test_var_type!(
    mutate_self_via_nested_function,
    "var foo = function() {
        self.a = 0;
    }
    var bar = function(foo) {
        foo();
    }
    bar(foo);",
    a: Real,
);

// Constructors
test_var_type!(
    constructor,
    "var foo = function() constructor {
        self.a = 0;
    }
    var bar = new foo();",
    bar: new_struct!(a: Real)
);
test_var_type!(
    manual_inheritance,
    "var foo = function() {
        self.a = 0;
    }
    var bar = function(foo) constructor {
        foo();
    }
    var fizz = new bar(foo);",
    fizz: new_struct!(a: Real)
);
test_var_type!(
    inheritance,
    "var foo = function() constructor {
        self.a = 0;
    }
    var bar = function() : foo() constructor {}
    var fizz = new bar(Foo);",
    fizz: new_struct!(a: Real)
);

// Misc
test_var_type!(
    needlessly_difficult,
    "var build_data = function(x, y, z) {
        return {
            x: x,
            y: y(0),
            z: z[0][0].a.b.c,
        };
    }
    var build_x = function(x) { return x; }
    var y_fn = function(n) { return n; }
    var z = [[{ a: { b: { c: 0 }}}]];
    var data = build_data(build_x(0), y_fn, z);
    var output = data.x + data.y + data.z;",
    z: new_array!(new_array!(new_struct!(a: new_struct!(b: new_struct!(c: Real))))),
    data: new_struct!(x: Real, y: Real, z: Real),
    output: Real
);
