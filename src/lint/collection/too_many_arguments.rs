use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    Config, FileId,
    lint::{EarlyExprPass, Lint, LintLevel},
    parse::{Expr, ExprKind, Field, Function},
};

#[derive(Debug, PartialEq)]
pub struct TooManyArguments;
impl Lint for TooManyArguments {
    fn explanation() -> &'static str {
        "Functions with lots of parameters quickly become confusing and indicate a need for structural change."
    }

    fn default_level() -> LintLevel {
        LintLevel::Warn
    }

    fn tag() -> &'static str {
        "too_many_arguments"
    }
}

impl EarlyExprPass for TooManyArguments {
    fn visit_expr_early(expr: &Expr, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let ExprKind::Function(Function { parameters, .. }) = expr.kind() {
            if parameters.len() > config.max_arguments {
                let start = parameters.first().unwrap().name_expr().span().start();
                let end = match parameters.last().unwrap() {
                    Field::Uninitialized(expr) => expr.span().end(),
                    Field::Initialized(stmt) => stmt.span().end(),
                };
                reports.push(
                    Self::diagnostic(config)
                        .with_message("Too many arguments")
                        .with_labels(vec![Label::primary(expr.file_id(), start..end).with_message(format!(
                            "using {} arguments, but the maximum is set to {}",
                            parameters.len(),
                            config.max_arguments
                        ))]),
                );
            }
        }
    }
}
