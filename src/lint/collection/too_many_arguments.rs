use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyExpressionPass, Lint, LintLevel},
    parse::{Expression, ExpressionBox, Function, OptionalInitilization},
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

impl EarlyExpressionPass for TooManyArguments {
    fn visit_expression_early(expression_box: &ExpressionBox, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let Expression::FunctionDeclaration(Function { parameters, .. }) = expression_box.expression() {
            if parameters.len() > config.max_arguments {
                let start = parameters.first().unwrap().name_expression().span().start();
                let end = match parameters.last().unwrap() {
                    OptionalInitilization::Uninitialized(expr) => expr.span().end(),
                    OptionalInitilization::Initialized(stmt) => stmt.span().end(),
                };
                reports.push(
                    Self::diagnostic(config)
                        .with_message("Too many arguments")
                        .with_labels(vec![Label::primary(expression_box.file_id(), start..end).with_message(
                            format!(
                                "using {} arguments, but the maximum is set to {}",
                                parameters.len(),
                                config.max_arguments
                            ),
                        )]),
                );
            }
        }
    }
}
