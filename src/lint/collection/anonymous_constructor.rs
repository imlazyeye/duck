use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyExpressionPass, Lint, LintLevel},
    parse::{Expression, ExpressionBox, Function},
    Config, FileId,
};

#[derive(Debug, PartialEq)]
pub struct AnonymousConstructor;
impl Lint for AnonymousConstructor {
    fn explanation() -> &'static str {
        "Constructors should be reserved for larger, higher scoped types."
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "anonymous_constructor"
    }
}

impl EarlyExpressionPass for AnonymousConstructor {
    fn visit_expression_early(expression_box: &ExpressionBox, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let Expression::FunctionDeclaration(Function {
            name: None,
            constructor: Some(_),
            ..
        }) = expression_box.expression()
        {
            reports.push(
                Self::diagnostic(config)
                    .with_message("Use of an anonymous constructor")
                    .with_labels(vec![
                        Label::primary(expression_box.file_id(), expression_box.span())
                            .with_message("change this to a function that returns a struct literal, or instead convert this into a named constructor"),
                    ]),
            );
        }
    }
}
