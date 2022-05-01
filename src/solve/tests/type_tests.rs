use super::*;
use crate::{adt, array, function, option, solve::*, test_failure, test_success, test_type};
use Ty::*;

// Basic expressions
test_type!(undefined, "undefined" => Undefined);
test_type!(noone, "noone" => Noone);
test_type!(bools, "true" => Bool, "false" => Bool);
test_type!(real, "1" => Real);
test_type!(real_float, "0.1" => Real);
test_type!(hex, "$ffffff" => Real);
test_type!(string, r#""foo""# => Str);
test_type!(grouping, "(0)" => Real);
test_type!(
    postfix,
    "var x = 0",
    "x++" => Real,
    "x--" => Real,
);
test_type!(
    unary,
    "var a = 0, b = false;",
    "++a" => Real,
    "--a" => Real,
    "+a" => Real,
    "-a" => Real,
    "~a" => Real,
    "!b" => Bool,
);
test_type!(ternary, "true ? 0 : 0" => Real);
test_type!(
    null_coalecence,
    "function(x) {
        return x ?? 0;
    }" => function!((option!(Real)) => Real)
);
test_type!(
    evaluation,
    "1 + 1" => Real,
    "1 * 1" => Real,
    "1 / 1" => Real,
    "1 % 1" => Real,
    "1 div 1" => Real,
);
test_type!(logical, "true && false" => Bool);
// this will require unions
// test_type!(add_strings, "\"foo\" + \"foo\"" => Str);
test_failure!(subtract_strings, "var a = \"foo\" - \"foo\"");
test_failure!(invalid_equality, "var a = 0 == true;");

// Basic statements
test_success!(repeat_loop, "repeat 1 {}");
test_success!(for_loop, "for (var i = 0; i < 0; i++) {}");
test_success!(do_until_loop, "do {} until true;");
test_success!(while_loop, "while true {}");
test_success!(if_stmt, "if true {}");
test_success!(switch, "switch true { case true: break; case false: break; }");
test_success!(
    numerical_assignment_ops,
    "var a = 0;
    a += 1;
    a -= 1;
    a /= 1;
    a *= 1;
    a %= 1;
    a &= 1;
    a |= 1;"
);

// Local variable
test_type!(local_var, "var a = 0", "a" => Real);
test_type!(assign_to_null_var, "var a; a = 0;", "a" => Real);
test_type!(shadowing, "var a = 0; var a = true;", "a" => Bool);
test_failure!(undefined_variable, "var a = b;");
test_failure!(read_variable_before_declaration, "var a = b, b = 0;");

// Globals
test_type!(globalvar, "globalvar foo; foo = 0", "foo" => Real);
test_type!(global, "global.foo = 0;", "foo" => Real);
test_failure!(duplicate_global_function, "function foo() {} function foo() {}");

// Enums
test_type!(enum_declaration, "enum foo { bar }", "foo" => adt!(bar: Real));
test_type!(access_enum, "enum foo { bar };", "foo.bar" => Real);
test_type!(
    members_as_real,
    "enum foo { bar, buzz }",
    "foo.bar + foo.buzz" => Real,
    "foo.bar + 1" => Real,
);
test_type!(
    enum_promise,
    "self.foo = Fizz.Buzz;
    enum Fizz { Buzz }",
    "foo" => Real,
);
test_failure!(non_real_enum_member_value, "enum foo { bar = true };");
test_failure!(non_constant_enum_member, "var fizz = 0; enum foo { bar = fizz };");
// The following require a more sophisticated understanding of how the name of its enum is itself a
// type, not a real value.
// test_failure!(reference_enum_type, "enum foo {}; bar = foo;");
// test_failure!(double_enum_declaration, "enum foo {}; enum foo {};");

// Macros
test_type!(macro_reference, "#macro foo 0\nvar bar = foo;", "bar" => Any);
test_type!(
    macro_promise,
    "self.foo = Fizz;
    #macro Fizz 0",
    "foo" => Any,
);

// Arrays
test_type!(constant_array, "[0]" => array!(Real));
test_type!(nested_array, "[[[0]]]" => array!(array!(array!(Real))));
test_type!(array_access, "var x = [0];", "x[0]" => Real);
test_type!(
    infer_array_type_from_copy,
    "var arr = [];
    var con = [0];
    array_copy(con, 0, arr, 0, 0);",
    "arr" => array!(Real)
);
test_failure!(invalid_array_access, "var a = 0, b = a[0];");
test_failure!(mixed_array, "var a = [0, true];");

// Structs
test_type!(empty_struct, "{}" => adt!());
test_type!(populated_struct, "{ x: 0 }" => adt!(x: Real));
test_type!(struct_access, "var foo = { x: 0 }", "foo.x" => Real);
test_type!(
    struct_extention,
    "var foo = { x: 0 };
    foo.y = 0;",
    "foo" => adt!(x: Real, y: Real),
);
test_type!(
    nested_structs,
    "var foo = { x: { y: { z: 0 } } };",
    "foo.x.y.z;" => Real,
);
test_type!(
    struct_field_transfer,
    "var foo = { x: 0 };
    var bar = { y: 0 };
    foo.x = bar.y;",
    "foo" => adt!(x: Real),
);
test_type!(
    function_on_struct,
    "var foo = {
        bar: function() { return 0; },
    };",
    "foo.bar()" => Real,
);
test_type!(
    infinite_cycle,
    "var foo = { a: 0 };
    foo.bar = foo;",
    "foo.bar.a" => Real
);
test_failure!(undefined_field, "var a = {}, b = a.x;");
test_failure!(invalid_dot_access, "var a = 0, b = a.x;");

// Functions
test_type!(function, "function() {}" => function!(() => Undefined));
test_type!(named_function, "function foo() {};", "foo" => function!(() => Undefined));
test_type!(
    default_argument,
    "function foo(x=0) { return x; }",
    "foo()" => Real
);
test_failure!(invalid_default_argument, "function foo(x=0, y) {}");
test_type!(return_nothing, "var foo = function() {};", "foo()" => Undefined,);
test_type!(return_constant, "var foo = function() { return 0; };", "foo()" => Real);
test_type!(
    return_inferred_constant,
    "var foo = function(x) { return x + 1; };",
    "foo(1);" => Real,
);
test_type!(
    echo,
    "var echo = function(x) { return x; };",
    "echo(true);" => Bool,
);
test_type!(
    return_generic_array_access,
    "var foo = function(x) { return x[0]; };",
    "foo([0])" => Real,
);
test_type!(
    return_generic_struct_access,
    "var foo = function(x) { return x.y; };",
    "foo({ y: 0 })" => Real,
);
test_type!(
    return_other_function_return,
    "function wrapper(lambda) {
        return lambda(0);
    }
    function inner(n) { return n; }",
    "wrapper(inner)" => Real,
);
test_type!(
    return_advanced_generic,
    "var foo = function(a, b) {
        return a[b];
    }
    var bar = function(x, y) {
        return x + y * 2;
    }",
    "foo([\"hello\"], 0)" => Str,
    "foo([ { a: true } ], bar(1, 2));" => adt!(a: Bool)
);
test_type!(
    return_array_with_arg,
    "function foo(x) { return [x]; }",
    "foo(0)" => array!(Real)
);
test_type!(
    return_struct_with_arg,
    "function foo(x) { return { x: x }; }",
    "foo(0)" => adt!(x: Real)
);
test_type!(
    multi_use_echo,
    "function echo(a) {
        return a;
    }",
    "echo(true)" => Bool,
    "echo(0)" => Real,
);
test_type!(
    return_onto_known_type,
    "var foo = function() {
        return 0;
    }
    var bar = 0;
    bar = foo();",
    "bar" => Real
);
test_type!(
    return_nested_return,
    "function foo() {
        return function bar() {
            return 0;
        }
    }",
    "foo" => function!(() => function!(() => Real))
);
test_type!(
    return_self,
    "function foo() constructor {
        function bar() { 
            return self;
        }
    }",
    "(new foo()).bar()" => adt!(
        foo: function!(() => Identity),
        bar: function!(() => Identity),
    )
);
test_type!(
    return_option,
    "function() {
        if true {
            return 0;
        } else {
            return undefined;
        }
    }" => function!(() => option!(Real))
);
test_type!(
    self_as_argument,
    "var echo = function(x) { return x; }",
    "echo(self)" => Identity,
);
test_type!(
    self_in_call_pattern,
    "function foo(a) {
        return a.x;
    }
    self.x = 0;",
    "self.foo(self)" => Real,
);
test_type!(
    infer_function,
    "function(x) { return x() + 1; }" => function!(
        (function!(() => Real)) => Real
    )
);
test_type!(
    infer_array,
    "function(x) { return x[0] + 1; }" => function!(
        (array!(Real)) => Real
    )
);
test_type!(
    infer_struct,
    "function(x) { return x.y + 1; }" => function!(
        (adt!(y: Real)) => Real
    )
);
test_success!(infer_multi_field_struct, "function foo(o) { return o.x + o.y; }");
test_type!(
    mutate_struct_via_function,
    "var foo = function(a) {
        a.a = 0;
    }
    var bar = {};
    foo(bar);",
    "bar" => adt!(a: Real)
);
test_type!(
    retain_all_fields_in_generic_call,
    "var foo = function(a) {
        a.a = 0;
        return a;
    }
    var bar = { a: 0, b: 0 };
    foo(bar);",
    "bar" => adt!(a: Real, b: Real)
);
test_type!(
    retain_all_fields_in_generic_call_after_return,
    "var foo = function(a) {
        a.a = 0;
        return a;
    }
    var bar = { a: 0, b: 0 };
    bar = foo(bar);",
    "bar" => adt!(a: Real, b: Real)
);
test_failure!(invalid_call_target, "var a = 0, b = a();");
test_failure!(invalid_argument, "var a = function(x) { return x + 1; }, b = a(true);");
test_failure!(missing_argument, "var a = function(x) {}, b = a();");
test_success!(missing_default_argument, "var a = function(x=0) {}, b = a();");
test_failure!(extra_argument, "var a = function() {}, b = a(0);");
test_failure!(contrasting_returns, "function() { return 0; return true; }");

// Self
test_type!(self_assignment_no_keyword, "foo = 0;", "foo" => Real);
test_type!(self_assignment_with_keyword, "self.foo = 0;", "self.foo" => Real);
test_type!(
    function_write_constant_to_self,
    "self.a = 0;
    function bar() { self.a = 0; }",
    "bar" => function!(() => Undefined),
);
test_type!(
    function_write_parameter_to_self,
    "self.a = 0;
    function bar(x) { self.a = x + 1; }",
    "bar" => function!((Real) => Undefined),
);
test_type!(function_self_extention, "function foo() { self.a = 0; }", "a" => Real,);
test_type!(
    function_self_extention_nested,
    "function foo() {
        function bar() { self.a = 0; }
    }",
    "a" => Real,
);
test_type!(
    bound_scope_in_struct,
    "var foo = {
        bar: 0,
        fizz: function() {
            return self.bar;
        }
    };",
    "foo.fizz()" => Real,
);
test_type!(
    obj_setter,
    "self.x = 0;
    self.y = 0;
    function set(obj) {
        self.x = obj.x;
        self.y = obj.y;
    }",
    "set" => function!((adt!(x: Real, y: Real)) => Undefined),
);
test_success!(
    gml_std,
    "var a = [true, false, true];
    array_insert(a, true, 0);"
);
test_type!(
    option_field,
    "self.a = undefined;
    self.b = 0;
    if true {
        self.a = 0;
        self.b = undefined;
    }",
    "a" => option!(Real),
    "b" => option!(Real),
);

// Constructors
test_type!(
    constructor,
    "var foo = function() constructor {}",
    "new foo()" => adt!()
);
test_type!(
    constructor_with_field,
    "var foo = function() constructor {
        self.x = 0;
    }",
    "new foo()" => adt!(x: Real)
);
test_type!(
    constructor_with_parameter,
    "function foo(y) constructor {
        self.x = y;
    }",
    "foo(0)" => adt!(x: Real)
);
test_type!(
    constructor_getter,
    "var foo = function() constructor {
        self.a = 0;
        function get_a() {
            return self.a;
        }
    }",
    "(new foo()).get_a()" => Real,
);
test_type!(
    inheritance,
    "var foo = function() constructor {
        self.a = 0;
    }
    var bar = function() : foo() constructor {}",
    "new bar()" => adt!(a: Real)
);
test_type!(
    inheritance_passing_arguments,
    "var foo = function(x) constructor {
        self.a = x;
    }
    var bar = function(x) : foo(x) constructor {}",
    "new bar(0)" => adt!(a: Real)
);
test_type!(
    multi_inheritance,
    "var foo = function() constructor {
        self.a = 0;
    }
    var bar = function() : foo() constructor {
        self.b = 0;
    }
    var fizz = function() : bar() constructor {}",
    "new fizz()" => adt!(a: Real, b: Real)
);
test_type!(
    alias_function,
    "function foo() constructor {
        self.x = 0;
    }
    var bar = function() {
        var new_struct = new foo();
        return new_struct;
    }",
    "bar()" => adt!(foo: function!(() => Identity), x: Real,)
);
test_type!(
    clone,
    "function foo() constructor {
        function clone() { return new foo(); }
    }",
    "(new foo()).clone()" => adt!(foo: function!(() => Identity), clone: function!(() => Identity),)
);
test_type!(
    identity_sanitization,
    "function alias() {
        return new con();
    }

    function con() constructor {
        function clone() {
            return alias();
        }
    }",
    "alias()" => adt!(con: function!(() => Identity), clone: function!(() => Identity),)
);
test_failure!(
    constructor_extention,
    "function foo() constructor {}
    var bar = new foo();
    bar.a = 0;"
);

// Out of order
test_type!(
    function_read_self_out_of_order,
    "function bar() { return self.a; }
    self.a = 0;",
    "bar" => function!(() => Real),
);
test_type!(
    function_write_self_out_of_order,
    "function bar(x) { self.a = x; }
    self.a = 0;",
    "bar" => function!((Real) => Undefined),
);
test_type!(
    function_calls_out_of_order,
    "function foo() { self.bar();}
    function bar() {}",
    "bar" => function!(() => Undefined),
);
test_type!(
    echo_out_of_order,
    "function wrapper() {
        return echo(0);
    }
    function echo(x) {
        return x;
    }",
    "wrapper()" => Real,
);
test_type!(
    self_as_argument_out_of_order,
    "self.x = 0;
    function bar() { fizz(self) }
    function fizz(o) { return o.x + 1; }",
    "fizz" => function!((adt!(x: Real)) => Real),
);

// Stress tests
test_type!(
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
    var data = build_data(build_x(0), y_fn, z);",
    "z" => array!(array!(adt!(a: adt!(b: adt!(c: Real))))),
    "data" => adt!(x: Real, y: Real, z: Real),
    "data.x + data.y + data.z" => Real,
);
test_type!(
    vec_2,
    "function Vec2(_x, _y) {
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
    }",
    "Vec2(0, 0)" => adt!(
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
test_type!(
    list,
    "#macro MINIMUM_DEFAULT_SIZE 4

    function ListFromArray(a) {
        return new __List(a, array_length(a));
    }

    function List() {
        return new __List([], 0);
    }

    function __List(arr, c) constructor {
        self.__count = c;
        self.__internal_size = array_length(arr);
        self.__buffer = arr;

        static count = function() {
            return self.__count;
        }

        static clone = function() {
            if self.is_empty() {
                return List();
            }
            var new_array = array_create(self.count(), 0);
            array_copy(new_array, 0, self.__buffer, 0, self.count());
            return ListFromArray(new_array);
        }
        
        static push = function(item) {
            self.ensure_size(self.__count + 1);
            self.__buffer[@ __count] = item;
            self.__count++;
        }

        // static pop = function() {
        //     if self.__count > 0 {
        //         var ret = self.get(self.__count - 1);
        //         self.__count -= 1;
        //         return ret;
        //     } else {
        //         return undefined;
        //     }
        // }

        static transfer = function(other_list) {
            self.copy_from(other_list);
            other_list.clear();
        }
        
        static copy_from = function(other_list) {
            self.ensure_size(self.__count + other_list.count());
            array_copy(self.__buffer, self.__count, other_list.__buffer, 0, other_list.__count);
            self.__count += other_list.count();
        }

        static get = function(idx) {
            return self.__buffer[idx];
        }

        static set = function(idx, item) {
            self.__buffer[@ idx] = item;
        }

        static remove = function(idx) {
            var value = self.__buffer[idx];
            self.__count--;
            var a = [];
            array_copy(a, 0, self.__buffer, 0, idx);
            array_copy(a, idx, self.__buffer, idx + 1, self.__count - idx);
            self.__buffer = a;
            return value;
        }
        
        static clear = function() {
            self.__count = 0;
        }

        static is_empty = function() {
            return self.__count == 0;
        }

        static find = function(selector) {
            for (var i = 0; i < self.count(); i++) {
                var cur = self.get(i);
                if (selector(cur)) {
                    return i;
                }
            }
            return undefined;
        }
        
        static to_array = function() {
            var a = [];
            array_copy(a, 0, self.__buffer, 0, self.__count);
            return a;
        }
            
        static ensure_size = function(requested_size) {
            while self.__internal_size < requested_size {
                self.__internal_size = self.__internal_size > 1 ? floor(self.__internal_size * (3 / 2)) : MINIMUM_DEFAULT_SIZE;
            }
        }
    }
    
    var list = new __List([0], 1);",
    "new __List([0], 1)" => adt!(
        __List: function!((array!(Real), Real) => Identity),
        __buffer: array!(Real),
        __count: Real,
        __internal_size: Real,
        count: function!(() => Real),
        clone: function!(() => Identity),
        push: function!((Real) => Undefined),
        // pop: function!(() => option!(Real)),
        transfer: function!(
            (adt!(clear: function!(() => Undefined)))
            => Undefined
        ),
        copy_from: function!(
            (adt!(__buffer: array!(Real), __count: Real, clear: function!(() => Undefined)))
            => Undefined
        ),
        get: function!((Real) => Real),
        set: function!((Real, Real) => Undefined),
        remove: function!((Real) => option!(Real)),
        clear: function!(() => Undefined),
        is_empty: function!(() => Bool),
        find: function!(
            (function!((Real) => Bool))
            => option!(Real)
        ),
        to_array: function!(() => array!(Real)),
        ensure_size: function!((Real) => Undefined)
    )
);
