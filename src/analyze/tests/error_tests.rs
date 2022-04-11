use crate::test_failure;

pub use super::*;

// Violations
test_failure!(invalid_equality, "var a = 0 == true;");
test_failure!(undefined_variable, "var a = b;");
test_failure!(undefined_field, "var a = {}, b = a.x;");
test_failure!(invalid_array_access, "var a = 0, b = a[0];");
test_failure!(invalid_dot_access, "var a = 0, b = a.x;");
test_failure!(invalid_call_target, "var a = 0, b = a();");
test_failure!(invalid_arguments, "var a = function(x) { return x + 1; }, b = a(true);");
test_failure!(missing_arguments, "var a = function(x) {}, b = a();");
test_failure!(contrasting_returns, "function() { return 0; return true; }");
test_failure!(
    undefined_variable_later_defined,
    "self.b = self.a;
    self.a = 0;"
);
// test_failure!(reference_enum_type, "enum foo {}; bar = foo;");
test_failure!(non_real_enum_member_value, "enum foo { bar = true };");
// test_failure!(illegal_macro_declaration_location, "foo = #macro bar 0");
// test_failure!(non_constant_enum_member, "var fizz = 0; enum foo { bar = fizz };");
// test_failure!(extra_arguments, "var a = function() {}, b = a(0);");
test_failure!(double_enum_declaration, "enum foo {}; enum foo {};");
test_failure!(double_macro_declaration, "#macro foo 0\n#macro foo 0;");
