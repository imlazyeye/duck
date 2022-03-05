use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyExpressionPass, Lint, LintLevel},
    parse::{Access, Expression, ExpressionBox},
    FileId,
};

#[derive(Debug, PartialEq)]
pub struct Global;
impl Lint for Global {
    fn explanation() -> &'static str {
        "While useful at times, global variables reduce saftey since they can be accessed or mutated anywhere, and provide no guarentee that they've already been initiailized."
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "global"
    }
}

impl EarlyExpressionPass for Global {
    fn visit_expression_early(
        expression_box: &ExpressionBox,
        config: &crate::Config,
        reports: &mut Vec<Diagnostic<FileId>>,
    ) {
        if let Expression::Access(Access::Global { .. }) = expression_box.expression() {
            reports.push(
                Self::diagnostic(config)
                    .with_message("Use of global variable")
                    .with_labels(vec![
                        Label::primary(expression_box.file_id(), expression_box.span())
                            .with_message("scope this variable to an individual object or struct"),
                    ]),
            );
        }
    }
}
