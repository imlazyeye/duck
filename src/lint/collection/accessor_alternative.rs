use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    Config, FileId,
    lint::{EarlyExprPass, Lint, LintLevel},
    parse::{Call, Expr, ExprKind, Literal},
};

#[derive(Debug, PartialEq)]
pub struct AccessorAlternative;
impl Lint for AccessorAlternative {
    fn explanation() -> &'static str {
        "GML offers accessors as an alternative to many common functions which are preferable for their readability and brevity."
    }

    fn default_level() -> LintLevel {
        LintLevel::Warn
    }

    fn tag() -> &'static str {
        "accessor_alternative"
    }
}

impl EarlyExprPass for AccessorAlternative {
    fn visit_expr_early(expr: &Expr, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let ExprKind::Call(Call { left, arguments, .. }) = expr.kind() {
            if let ExprKind::Identifier(identifier) = left.kind() {
                reports.push(match identifier.lexeme.as_ref() {
                    "ds_list_find_value" => Self::diagnostic(config)
                        .with_message("Use of `ds_list_find_value`")
                        .with_labels(vec![
                            Label::primary(expr.file_id(), expr.span())
                                .with_message("this can be replaced with an accessor syntax (`list[| index]`"),
                        ]),

                    "ds_grid_get" => Self::diagnostic(config)
                        .with_message("Use of `ds_grid_get`")
                        .with_labels(vec![
                            Label::primary(expr.file_id(), expr.span())
                                .with_message("this can be replaced with an accessor syntax (`grid[# x, y]`"),
                        ]),
                    "ds_map_find_value" => Self::diagnostic(config)
                        .with_message("Use of `ds_map_find_value`")
                        .with_labels(vec![
                            Label::primary(expr.file_id(), expr.span())
                                .with_message("this can be replaced with an accessor syntax (`map[? key]`"),
                        ]),
                    "array_get" => Self::diagnostic(config)
                        .with_message("Use of `array_get`")
                        .with_labels(vec![
                            Label::primary(expr.file_id(), expr.span())
                                .with_message("this can be replaced with an accessor syntax (`array[index]`"),
                        ]),
                    "variable_struct_get" => {
                        let argument_one = arguments.get(1);
                        match argument_one {
                            Some(arg_one_box) => {
                                match arg_one_box.kind() {
                                    // If the string literal contains a valid lexeme, then they could use dot access
                                    ExprKind::Literal(Literal::String(string))
                                        if string.chars().all(|v| v.is_alphanumeric() || v == '_') =>
                                    {
                                        Self::diagnostic(config)
                                            .with_message("Use of `variable_struct_get`")
                                            .with_labels(vec![
                                                Label::primary(expr.file_id(), expr.span()).with_message(
                                                    "this can be replaced with a dot access syntax (`struct.property`)",
                                                ),
                                            ])
                                    }

                                    // If not, they could still use the $ accessor
                                    _ => Self::diagnostic(config)
                                        .with_message("Use of `variable_struct_get`")
                                        .with_labels(vec![Label::primary(expr.file_id(), expr.span()).with_message(
                                            "this can be replaced with an accessor syntax (`struct[$ key]`)",
                                        )]),
                                }
                            }
                            None => return, // missing argument, invalid gml -- we will validate this in the future
                        }
                    }
                    _ => return,
                });
            }
        }
    }
}
