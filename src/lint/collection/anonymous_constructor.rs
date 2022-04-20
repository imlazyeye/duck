use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyExprPass, Lint, LintLevel},
    parse::{Expr, ExprKind, Function},
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

impl EarlyExprPass for AnonymousConstructor {
    fn visit_expr_early(expr: &Expr, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let ExprKind::Function(Function {
            name: None,
            constructor: Some(_),
            ..
        }) = expr.inner()
        {
            reports.push(
                Self::diagnostic(config)
                    .with_message("Use of an anonymous constructor")
                    .with_labels(vec![
                        Label::primary(expr.file_id(), expr.span())
                            .with_message("change this to a function that returns a struct literal, or instead convert this into a named constructor"),
                    ]),
            );
        }
    }
}
