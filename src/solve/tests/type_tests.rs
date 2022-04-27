use super::*;
use crate::{adt, array, function, solve::*, test_expr_type, test_failure, test_success, test_var_type};
use Ty::*;

// Basic expressions
test_expr_type!(undefined, "undefined" => Undefined);
test_expr_type!(noone, "noone" => Noone);
test_expr_type!(bools, "true" => Bool, "false" => Bool);
test_expr_type!(real, "1" => Real);
test_expr_type!(real_float, "0.1" => Real);
test_expr_type!(hex, "$ffffff" => Real);
test_expr_type!(string, r#""foo""# => Str);
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
// test_expr_type!(
//     null_coalecence,
//     "function(x) {
//         return x ?? 0;
//     }" => function!((Real) => Real)
// );
test_expr_type!(
    evaluation,
    "1 + 1" => Real,
    "1 * 1" => Real,
    "1 / 1" => Real,
    "1 % 1" => Real,
    "1 div 1" => Real,
);
// this will require unions
// test_expr_type!(add_strings, "\"foo\" + \"foo\"" => Str);
test_failure!(subtract_strings, "var a = \"foo\" - \"foo\"");
test_expr_type!(logical, "true && false" => Bool);
test_failure!(invalid_equality, "var a = 0 == true;");

// Basic statements
test_success!(repeat_loop, "repeat 1 {}");
test_success!(for_loop, "for (var i = 0; i < 0; i++) {}");
test_success!(do_until_loop, "do {} until true;");
test_success!(while_loop, "while true {}");
test_success!(if_stmt, "if true {}");
test_success!(switch, "switch true { case true: break; case false: break; }");

// Local variable
test_var_type!(local_var, "var a = 0", a: Real);
test_var_type!(null_local_var, "var a;", a: Uninitialized);
test_var_type!(assign_to_null_var, "var a; a = 0;", a: Real);
test_failure!(undefined_variable, "var a = b;");
test_failure!(read_variable_before_declaration, "var a = b, b = 0;");

// Globals
test_var_type!(globalvar, "globalvar foo;", foo: Uninitialized);
test_var_type!(globalvar_assign, "globalvar foo; foo = 0", foo: Real);
test_var_type!(global, "global.foo = 0;", foo: Real);
test_failure!(duplicate_global_function, "function foo() {} function foo() {}");

// Enums
test_var_type!(enum_declaration, "enum foo { bar }", foo: adt!(bar: Real));
test_var_type!(
    access_enum,
    "enum foo { bar }; 
    var bar = foo.bar;",
    bar: Real,
);
test_var_type!(
    members_as_real,
    "enum foo { bar, buzz }
    var a = foo.bar + foo.buzz;
    var b = foo.bar + 1;",
    a: Real,
    b: Real,
);
test_var_type!(
    enum_promise,
    "self.foo = Fizz.Buzz;
    enum Fizz { Buzz }",
    foo: Real,
);
test_failure!(non_real_enum_member_value, "enum foo { bar = true };");
test_failure!(non_constant_enum_member, "var fizz = 0; enum foo { bar = fizz };");
// The following require a more sophisticated understanding of how the name of its enum is itself a
// type, not a real value.
// test_failure!(reference_enum_type, "enum foo {}; bar = foo;");
// test_failure!(double_enum_declaration, "enum foo {}; enum foo {};");

// Macros
test_var_type!(macro_reference, "#macro foo 0\nvar bar = foo;", bar: Any);
test_var_type!(
    macro_promise,
    "self.foo = Fizz;
    #macro Fizz 0",
    foo: Any,
);

// Arrays
test_expr_type!(empty_array, "[]" => array!(Any));
test_expr_type!(constant_array, "[0]" => array!(Real));
test_expr_type!(nested_array, "[[[0]]]" => array!(array!(array!(Real))));
test_var_type!(array_access, "var x = [0], y = x[0];", y: Real);
test_failure!(invalid_array_access, "var a = 0, b = a[0];");

// Structs
test_expr_type!(empty_struct, "{}" => adt!());
test_expr_type!(populated_struct, "{ x: 0 }" => adt!(x: Real));
test_var_type!(struct_access, "var foo = { x: 0 }, bar = foo.x;", bar: Real,);
test_var_type!(
    struct_extention,
    "var foo = { x: 0 };
    foo.y = 0;",
    foo: adt!(x: Real, y: Real),
);
test_var_type!(
    nested_structs,
    "var foo = { x: { y: { z: 0 } } };
    var bar = foo.x.y.z;",
    bar: Real,
);
test_var_type!(
    struct_field_transfer,
    "var foo = { x: 0 };
    var bar = { y: 0 };
    foo.x = bar.y;",
    foo: adt!(x: Real),
);
test_var_type!(
    function_on_struct,
    "var foo = {
        bar: function() { return 0; },
    };
    var fizz = foo.bar();",
    fizz: Real
);
test_var_type!(
    infinite_cycle,
    "var foo = { a: 0 };
    foo.bar = foo;
    var fizz = foo.bar.a",
    fizz: Real
);
test_failure!(undefined_field, "var a = {}, b = a.x;");
test_failure!(invalid_dot_access, "var a = 0, b = a.x;");

// Functions
test_expr_type!(function, "function() {}" => function!(() => Undefined));
test_var_type!(named_function, "function foo() {};", foo: function!(() => Undefined));
test_var_type!(
    default_argument,
    "function foo(x=0) { return x; }
    var a = foo();",
    a: Real
);
test_failure!(invalid_default_argument, "function foo(x=0, y) {}");
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
    identity_function,
    "var foo = function(x) { return x; };
    var bar = foo(true);",
    bar: Bool,
);
test_var_type!(
    return_generic_array_access,
    "var foo = function(x) { return x[0]; };
    var bar = foo([0]);",
    bar: Real,
);
test_var_type!(
    return_generic_struct_access,
    "var foo = function(x) { return x.y; };
    var bar = foo({ y: 0 });",
    bar: Real,
);
test_var_type!(
    return_other_function_return,
    "function wrapper(lambda) {
        return lambda(0);
    }
    function inner(n) { return n; }
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
    buzz: adt!(a: Bool)
);
test_var_type!(
    return_array_with_arg,
    "function foo(x) { return [x]; }
    var bar = foo(0);",
    bar: array!(Real)
);
test_var_type!(
    return_struct_with_arg,
    "function foo(x) { return { x: x }; }
    var bar = foo(0);",
    bar: adt!(x: Real)
);
test_var_type!(
    multi_use_identity,
    "function foo(a) {
        return a;
    }
    var bar = foo(true);
    var fizz = foo(0);",
    bar: Bool,
    fizz: Real,
);
test_var_type!(
    return_onto_known_type,
    "var foo = function() {
        return 0;
    }
    var bar = 0;
    bar = foo();",
    bar: Real
);
test_var_type!(
    return_nested_return,
    "function foo() {
        return function bar() {
            return 0;
        }
    }",
    foo: function!(() => function!(() => Real))
);
test_var_type!(
    return_self,
    "function foo() constructor {
        function bar() { 
            return self;
        }
    }
    var fizz = (new foo()).bar();",
    fizz: adt!(foo: function!(() => Identity), bar: function!(() => Identity),)
);
test_var_type!(
    self_as_argument,
    "var identity = function(x) { return x; }
    var foo = identity(self);",
    foo: adt!(),
);
test_var_type!(
    self_in_call_pattern,
    "function foo(a) {
        return a.x;
    }
    self.x = 0;
    self.y = self.foo(self);",
    y: Real,
);

test_expr_type!(
    infer_function,
    "function(x) { return x() + 1; }" => function!(
        (function!(() => Real)) => Real
    )
);
test_expr_type!(
    infer_array,
    "function(x) { return x[0] + 1; }" => function!(
        (array!(Real)) => Real
    )
);
test_expr_type!(
    infer_struct,
    "function(x) { return x.y + 1; }" => function!(
        (adt!(y: Real)) => Real
    )
);
test_success!(infer_multi_field_struct, "function foo(o) { return o.x + o.y; }");
test_var_type!(
    mutate_struct_via_function,
    "var foo = function(a) {
        a.a = 0;
    }
    var bar = {};
    foo(bar);",
    bar: adt!(a: Real)
);
test_var_type!(
    retain_all_fields_in_generic_call,
    "var foo = function(a) {
        a.a = 0;
        return a;
    }
    var bar = { a: 0, b: 0 };
    foo(bar);",
    bar: adt!(a: Real, b: Real)
);
test_var_type!(
    retain_all_fields_in_generic_call_after_return,
    "var foo = function(a) {
        a.a = 0;
        return a;
    }
    var bar = { a: 0, b: 0 };
    bar = foo(bar);",
    bar: adt!(a: Real, b: Real)
);
test_failure!(invalid_call_target, "var a = 0, b = a();");
test_failure!(invalid_argument, "var a = function(x) { return x + 1; }, b = a(true);");
test_failure!(missing_argument, "var a = function(x) {}, b = a();");
test_success!(missing_default_argument, "var a = function(x=0) {}, b = a();");
test_failure!(extra_argument, "var a = function() {}, b = a(0);");
test_failure!(contrasting_returns, "function() { return 0; return true; }");

// Self
test_var_type!(self_assignment_no_keyword, "foo = 0;", foo: Real);
test_var_type!(self_assignment_with_keyword, "self.foo = 0;", foo: Real);
test_var_type!(
    function_write_constant_to_self,
    "self.a = 0;
    function bar() { self.a = 0; }",
    bar: function!(() => Undefined),
);
test_var_type!(
    function_write_parameter_to_self,
    "self.a = 0;
    function bar(x) { self.a = x + 1; }",
    bar: function!((Real) => Undefined),
);
test_var_type!(function_self_extention, "function foo() { self.a = 0; }", a: Real,);
test_var_type!(
    function_self_extention_nested,
    "function foo() { 
        function bar() { self.a = 0; }
    }",
    a: Real,
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
test_var_type!(
    obj_setter,
    "self.x = 0;
    self.y = 0;
    function set(obj) {
        self.x = obj.x;
        self.y = obj.y;
    }",
    set: function!((adt!(x: Real, y: Real)) => Undefined),
);

// Constructors
test_var_type!(
    constructor,
    "var foo = function() constructor {
        self.a = 0;
    }
    var bar = new foo();",
    bar: adt!(a: Real)
);
test_var_type!(
    constructor_with_parameter,
    "function foo(y) constructor {
        self.x = y;
    }
    var bar = foo(0);",
    bar: adt!(foo: function!((Real) => Identity), x: Real,)
);
test_var_type!(
    constructor_getter,
    "var foo = function() constructor {
        self.a = 0;
        function get_a() {
            return self.a;
        }
    }
    var bar = new foo()
    var fizz = bar.get_a();",
    fizz: Real,
);
test_var_type!(
    inheritance,
    "var foo = function() constructor {
        self.a = 0;
    }
    var bar = function() : foo() constructor {}
    var fizz = new bar();",
    fizz: adt!(a: Real)
);
test_var_type!(
    inheritance_passing_arguments,
    "var foo = function(x) constructor {
        self.a = x;
    }
    var bar = function(x) : foo(x) constructor {}
    var fizz = new bar(0);",
    fizz: adt!(a: Real)
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
    buzz: adt!(a: Real, b: Real)
);
test_var_type!(
    alias_function,
    "function foo() constructor {
        self.x = 0;
    }
    var bar = function() {
        var new_struct = new foo();
        return new_struct;
    }
    var fizz = bar();",
    fizz: adt!(foo: function!(() => Identity), x: Real,)
);
test_var_type!(
    clone,
    "function foo() constructor {
        function clone() { return new foo(); }
    }
    var bar = (new foo()).clone();",
    bar: adt!(foo: function!(() => Identity), clone: function!(() => Identity),)
);
test_var_type!(
    identity_sanitization,
    "function alias() {
        return new con();
    }

    function con() constructor {
        function clone() {
            return alias();
        }
    }

    var result = alias();",
    result: adt!(con: function!(() => Identity), clone: function!(() => Identity),)
);
test_failure!(
    constructor_extention,
    "function foo() constructor {}
    var bar = new foo();
    bar.a = 0;"
);

// Out of order
test_var_type!(
    function_read_self_out_of_order,
    "function bar() { return self.a; }
    self.a = 0;",
    bar: function!(() => Real),
);
test_var_type!(
    function_write_self_out_of_order,
    "function bar(x) { self.a = x; }
    self.a = 0;",
    bar: function!((Real) => Undefined),
);
test_var_type!(
    function_calls_out_of_order,
    "function foo() { self.bar();}
    function bar() {}",
    bar: function!(() => Undefined),
);
test_var_type!(
    identity_out_of_order,
    "function wrapper() {
        return identity(0);
    }
    function identity(x) {
        return x;
    }
    var bar = wrapper();",
    bar: Real,
);

// Stress tests
test_var_type!(
    complicted_data_construction,
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
    z: array!(array!(adt!(a: adt!(b: adt!(c: Real))))),
    data: adt!(x: Real, y: Real, z: Real),
    output: Real
);
test_var_type!(
    vec_2,
    r#"
    // placeholder until the std is in here
    function sqrt(num) {
        return num + 1;
    }

    function Vec2(_x, _y) {
        return new __Vec2(_x, _y);
    }

    function __Vec2(_x, _y) constructor {
        self.x = _x;
        self.y = _y;

        static set = function(o) {
            self.x = o.x;
            self.y = o.y;
        }

        static clone = function() {
            return Vec2(self.x, self.y);
        }

        static eq = function(o) {
            return o.x == self.x && o.y == self.y;
        }

        static scale = function(scalar) {
            return Vec2(self.x * scalar, self.y * scalar);
        }

        static set_scale = function(scalar) {
            self.x *= scalar;
            self.y *= scalar;
        }

        static sqrd_magnitude = function() {
            return self.dot(self);
        }

        static normalize = function() {
            var sqrd_magnitude_rcp = 1.0 / sqrt(self.sqrd_magnitude());
            return self.scale(sqrd_magnitude_rcp);
        }

        static dot = function(o) {
            return self.x * o.x + self.y * o.y;
        }
    }

    var a = Vec2(0, 0);
    "#,
    a: adt!(
        __Vec2: function!((Real, Real) => Identity),
        x: Real,
        y: Real,
        set: function!((adt!(x: Real, y: Real)) => Undefined),
        clone: function!(() => Identity),
        eq: function!((adt!(x: Real, y: Real)) => Bool),
        scale: function!((Real) => Identity),
        set_scale: function!((Real) => Undefined),
        sqrd_magnitude: function!(() => Real),
        normalize: function!(() => Identity),
        dot: function!((adt!(x: Real, y: Real)) => Real),
    )
);