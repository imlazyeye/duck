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
// test_failure!(extra_arguments, "var a = function() {}, b = a(0);");
