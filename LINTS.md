# Lints

Below are all of the lints currently supported in `duck`.

| Tag | Default Level | Explanation
|---|---|---|
| accessor_alternative | LintLevel::Warn | GML offers accessors as an alternative to many common functions which are preferable for their readability and brevity.
| and_preference | LintLevel::Allow | GML supports both `and` and `&&` to refer to logical \"and\". Consistent use of one over the other yields cleaner code.
| anonymous_constructor | LintLevel::Allow | Constructors should be reserved for larger, higher scoped types.
| assignment_to_call | LintLevel::Deny | While possible to compile, assigning a value to the call of a function does not do anything.
| bool_equality | LintLevel::Allow | Comparing a bool with a bool literal is more verbose than neccesary.
| collapsable_if | LintLevel::Warn | If statements that contain nothing more than another if statement can be collapsed into a single statement.
| deprecated | LintLevel::Warn | Deprecated features are liable to be removed at any time and should be avoided.
| draw_sprite | LintLevel::Allow | Projects that implement their own rendering backend may wish to be restrictive around when and where the `draw_sprite` functions are called.
| draw_text | LintLevel::Allow | Projects that implement their own UI frameworks / localization may wish to be restrictive around when and where the `draw_text` functions are called.
| english_flavor_violation | LintLevel::Allow | GML has many duplicated function names for the sake of supporting both British and American spelling. For consistency, codebases should stick to one.
| exit | LintLevel::Allow | `return` can always be used in place of exit, which provides more consistency across your codebase.
| global | LintLevel::Allow | While useful at times, global variables reduce saftey since they can be accessed or mutated anywhere, and provide no guarentee that they've already been initiailized.
| missing_case_member | LintLevel::Warn | Switch statements matching over an enum typically want to cover all possible cases if they do not implement a default case.
| missing_default_case | LintLevel::Allow | Switch statements are often used to express all possible outcomes of a limited data set, but by not implementing a default case, no code will run to handle any alternate or unexpected values.
| mod_preference | LintLevel::Allow | GML supports both `mod` and `%` to perform modulo division. Consistent use of one over the other yields cleaner code.
| multi_var_declaration | LintLevel::Allow | While GML allows you to create multiple local variables at once, it can often lead to confusing syntax that would read better with each variable seperated.
| no_space_begining_comment | LintLevel::Allow | Comments should begin with a space after them to increase readability and consistency.
| non_constant_default_parameter | LintLevel::Warn | Expressive default parameters are not supported in most languages due to their instability and tendency to hide important logic execution from the caller.
| non_pascal_case | LintLevel::Warn |  to distinguish them from other values.
| non_scream_case | LintLevel::Warn | Scream case is the ideal casing for constants to distingusih them from other values.
| or_preference | LintLevel::Allow | GML supports both `not` and `!` to refer to unary \"not\". Consistent use of one over the other yields cleaner code.
| or_preference | LintLevel::Allow | GML supports both `or` and `||` to refer to logical \"or\" -- `||` is more consistent with other languages and is preferred.
| room_goto | LintLevel::Allow | Projects that implement their own frameworks for room management may wish to be restrictive around when and where the `room_goto` functions are called.
| show_debug_message | LintLevel::Allow | Projects often implement their own logging framework and wish to avoid unwrapped prints to the console.
| single_switch_case | LintLevel::Warn | Switch statements that only match on a single element can be reduced to an `if` statement.
| statement_parenthetical_preference | LintLevel::Allow | Parenthesis surrounding statement expressions are optional in GML, resulting in differing opinions on whether or not to use them. You can select either option via the config.
| suspicious_constant_usage | LintLevel::Deny | Using a constant outside of equalities and direct assignments is likely unintended or misunderstood code.
| todo | LintLevel::Allow | Todo markers are useful for work-in-progress code, but often are not intended to be permanently in place.
| too_many_arguments | LintLevel::Warn | Functions with lots of parameters quickly become confusing and indicate a need for structural change.
| too_many_lines | LintLevel::Warn | Functions with lots of lines are harder to work with due to the volume of code that must be read to understand them.
| try_catch | LintLevel::Allow | GML's try/catch will collect all errors as opposed to the precise ones wanted, allowing them to accidently catch errors that should not be surpressed.
| var_prefix_violation | LintLevel::Allow | It is common practice in GML to prefix local variables (longer than one charcter) with an underscore as it helps to visually distinguish them from instance (or global) variables. You can select either option via the config.
| with_loop | LintLevel::Allow | The `with` loop allows your code's context to suddenly change, both making it more difficult to read (as a given line of code is no longer promised to be executing in the scope expected from the file), but also making it more difficult to track down all of the places an object is modified.
