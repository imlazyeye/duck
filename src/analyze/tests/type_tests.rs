use super::*;
use crate::{analyze::*, new_array, new_function, new_struct, new_union, test_expr_type, test_var_type};
use pretty_assertions::assert_eq;
use Type::*;

pub struct TestTypeWriter(Typewriter);
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

// Basic expressions
test_expr_type!(undefined, "undefined" => Undefined);
test_expr_type!(bools, "true" => Bool, "false" => Bool);
test_expr_type!(real, "1" => Real, "0.1" => Real);
test_expr_type!(real_float, "0.1" => Real);
test_expr_type!(string, "\"foo\"" => Str);
test_expr_type!(grouping, "(0)" => Real);
test_var_type!(postfix, "var x = 0, y = x++, z = x--;", x: Real, y: Real, z: Real);
test_var_type!(
    unary,
    "var a = 0, b = ++a, c = --a, d = +a, e = -a, f = ~a, g = true, h = !g;",
    a: Real,
    b: Real,
    c: Real,
    d: Real,
    e: Real,
    f: Real,
    g: Bool,
    h: Bool,
);
test_expr_type!(ternary, "true ? 0 : 0" => Real);
test_expr_type!(
    null_coalecence,
    "function(x) {
        return x ?? 0;
    }" => new_function!((new_union!(Real, Undefined)) => Real)
);
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

// Local variable
test_var_type!(local_variable, "var a = 0", a: Real);

// Arrays
test_expr_type!(empty_array, "[]" => new_array!(Any));
test_expr_type!(constant_array, "[0]" => new_array!(Real));
test_expr_type!(nested_array, "[[[0]]]" => new_array!(new_array!(new_array!(Real))));
test_var_type!(array_access, "var x = [0], y = x[0];", y: Real);

// Structs
test_expr_type!(empty_struct, "{}" => new_struct!());
test_expr_type!(populated_struct, "{ x: 0 }" => new_struct!(x: Real));
test_var_type!(struct_access, "var foo = { x: 0 }, bar = foo.x;", bar: Real,);
test_var_type!(
    function_on_struct,
    "var foo = {
        bar: function() { return 0; },
    };
    var fizz = foo.bar();",
    fizz: Real
);

// Functions
test_expr_type!(function, "function() {}" => new_function!(() => Undefined));
test_var_type!(
    named_function,
    "function foo() {};",
    foo: new_function!(() => Undefined)
);
test_var_type!(
    return_nothing,
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
    return_inferred_constant,
    "var foo = function(x) { return x + 1; };
    var bar = foo(1);",
    bar: Real,
);
test_var_type!(
    return_generic_value,
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
    var buzz = foo([ { a: true } ], bar(1, 2));"#,
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
    "function foo() {
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
test_var_type!(
    mutate_self_via_multi_nested_function,
    "var foo = function() {
        self.a = 0;
    }
    var bar = function() {
        self.b = 0;
    }
    var fizz = function(foo, bar) {
        foo();
        bar();
    }
    fizz(foo, bar);",
    a: Real,
    b: Real,
);
test_var_type!(
    alias_function,
    "var bar = function() {
        var new_struct = new foo();
        return new_struct;
    }
    function foo() constructor {
        self.x = 0;
    }
    var fizz = bar();",
    fizz: new_struct!(x: Real)
);
test_var_type!(
    bound_scope_in_struct,
    "var foo = {
        bar: 0,
        fizz: function() {
            return self.bar;
        }
    };
    var buzz = foo.fizz();",
    buzz: Real,
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
    multi_manual_inheritance,
    "var foo = function() {
        self.a = 0;
    }
    var bar = function() {
        self.b = 0;
    }
    var fizz = function(foo, bar) constructor {
        foo();
        bar();
    }
    var buzz = new fizz(foo, bar);",
    buzz: new_struct!(a: Real, b: Real)
);
test_var_type!(
    inheritance,
    "var foo = function() constructor {
        self.a = 0;
    }
    var bar = function() : foo() constructor {}
    var fizz = new bar();",
    fizz: new_struct!(a: Real)
);
test_var_type!(
    inheritance_passing_arguments,
    "var foo = function(x) constructor {
        self.a = x;
    }
    var bar = function(x) : foo(x) constructor {}
    var fizz = new bar(0);",
    fizz: new_struct!(a: Real)
);
test_var_type!(
    multi_inheritance,
    "var foo = function() constructor {
        self.a = 0;
    }
    var bar = function() : foo() constructor {
        self.b = 0;
    }
    var fizz = function() : bar() constructor {}
    var buzz = new fizz();",
    buzz: new_struct!(a: Real, b: Real)
);

// Stress tests
test_var_type!(
    stressful_data,
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
test_var_type!(
    vec_2,
    r#"
    function Vec2(_x, _y) {
        return new __Vec2(_x, _y);
    }

    function __Vec2(_x, _y) constructor {
        self.x = _x;
        self.y = _y;
    }

    var a = Vec2(0, 0);
    "#,
    a: new_struct!(x: Real, y: Real)
);
