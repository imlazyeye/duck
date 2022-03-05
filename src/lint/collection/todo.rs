use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyExpressionPass, Lint, LintLevel},
    parse::{Call, Expression, ExpressionBox},
    Config, FileId,
};

#[derive(Debug, PartialEq)]
pub struct Todo;
impl Lint for Todo {
    fn explanation() -> &'static str {
        "Todo markers are useful for work-in-progress code, but often are not intended to be permanently in place."
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "todo"
    }
}

impl EarlyExpressionPass for Todo {
    fn visit_expression_early(expression_box: &ExpressionBox, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let Expression::Call(Call { left, .. }) = expression_box.expression() {
            if let Expression::Identifier(identifier) = left.expression() {
                if identifier.name == config.todo_keyword {
                    reports.push(
                        Self::diagnostic(config)
                            .with_message("Use of todo marker")
                            .with_labels(vec![
                                Label::primary(left.file_id(), left.span()).with_message("remove this todo marker"),
                            ]),
                    );
                }
            }
        }
    }
}
