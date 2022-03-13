use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyExprPass, Lint, LintLevel},
    parse::{Expr, ExprType, Function, OptionalInitilization},
    Config, FileId,
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
        if let ExprType::FunctionDeclaration(Function { parameters, .. }) = expr.inner() {
            if parameters.len() > config.max_arguments {
                let start = parameters.first().unwrap().name_expr().span().start();
                let end = match parameters.last().unwrap() {
                    OptionalInitilization::Uninitialized(expr) => expr.span().end(),
                    OptionalInitilization::Initialized(stmt) => stmt.span().end(),
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
