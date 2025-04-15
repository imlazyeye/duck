use codespan_reporting::diagnostic::{Diagnostic, Label};
use colored::Colorize;

use crate::{
    Config, FileId,
    lint::{EarlyExprPass, Lint, LintLevel},
    parse::{Expr, ExprKind, Function, Identifier, ParseVisitor, Stmt},
};

#[derive(Debug, PartialEq)]
pub struct UnusedParameter;
impl Lint for UnusedParameter {
    fn explanation() -> &'static str {
        "Functions with lots of parameters quickly become confusing and indicate a need for structural change."
    }

    fn default_level() -> LintLevel {
        LintLevel::Warn
    }

    fn tag() -> &'static str {
        "unused_parameter"
    }
}

impl EarlyExprPass for UnusedParameter {
    fn visit_expr_early(expr: &Expr, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let ExprKind::Function(Function {
            parameters,
            name: _,
            constructor,
            body,
        }) = expr.kind()
        {
            let mut parameters_to_find: Vec<&Identifier> = parameters
                .iter()
                .filter_map(|p| {
                    if p.name_identifier().lexeme.starts_with("_") {
                        None
                    } else {
                        Some(p.name_identifier())
                    }
                })
                .collect();

            if let Some(constructor) = constructor {
                if let Some(inheritance) = &constructor.inheritance {
                    find_expr(inheritance, &mut parameters_to_find);
                }
            }

            body.visit_child_stmts(|s| find_stmt(s, &mut parameters_to_find));

            for parameter in parameters_to_find.iter() {
                reports.push(
                    Self::diagnostic(config)
                        .with_message("Unused parameter")
                        .with_labels(vec![
                            Label::primary(expr.file_id(), parameter.span)
                                .with_message(format!("Unused parameter: {}", parameter)),
                        ])
                        .with_notes(vec![format!(
                            "{}: you can prefix this parameter with an underscore to mark it as intentionally ignored (_{})",
                            "help".bold(),
                            parameter
                        )]),
                );
            }
        }
    }
}

fn find_stmt(stmt: &Stmt, parameters_to_remove: &mut Vec<&Identifier>) {
    stmt.visit_child_exprs(|e| find_expr(e, parameters_to_remove));
    stmt.visit_child_stmts(|s| find_stmt(s, parameters_to_remove));
}

fn find_expr(expr: &Expr, parameters_to_remove: &mut Vec<&Identifier>) {
    match expr.kind() {
        ExprKind::Identifier(identifier) => {
            parameters_to_remove.retain(|p| p.lexeme != identifier.lexeme);
        }
        ExprKind::Function(_) => {
            // ignore this new scope!
            return;
        }
        _ => {}
    }

    expr.visit_child_exprs(|e| find_expr(e, parameters_to_remove));
    expr.visit_child_stmts(|s| find_stmt(s, parameters_to_remove));
}
