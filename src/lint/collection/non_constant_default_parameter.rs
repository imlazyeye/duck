use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyExpressionPass, Lint, LintLevel},
    parse::{Expression, ExpressionBox, Function},
    FileId,
};

#[derive(Debug, PartialEq)]
pub struct NonConstantDefaultParameter;
impl Lint for NonConstantDefaultParameter {
    fn explanation() -> &'static str {
        "Expressive default parameters are not supported in most languages due to their instability and tendency to hide important logic execution from the caller."
    }

    fn default_level() -> LintLevel {
        LintLevel::Warn
    }

    fn tag() -> &'static str {
        "non_constant_default_parameter"
    }
}

impl EarlyExpressionPass for NonConstantDefaultParameter {
    fn visit_expression_early(
        expression_box: &ExpressionBox,
        config: &crate::Config,
        reports: &mut Vec<Diagnostic<FileId>>,
    ) {
        if let Expression::FunctionDeclaration(Function { parameters, .. }) = expression_box.expression() {
            for param in parameters {
                if let Some(default_value_expression_box) = param.assignment_value() {
                    if !matches!(
                        default_value_expression_box.expression(),
                        Expression::Identifier(_) | Expression::Literal(_),
                    ) {
                        reports.push(
                            Self::diagnostic(config)
                                .with_message("Non constant default parameter")
                                .with_labels(vec![
                                    Label::primary(
                                        default_value_expression_box.file_id(),
                                        default_value_expression_box.span(),
                                    )
                                    .with_message("this parameter's default value is not constant"),
                                ]),
                        );
                    }
                }
            }
        }
    }
}
