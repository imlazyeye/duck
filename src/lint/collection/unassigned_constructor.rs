use crate::{
    lint::{EarlyStatementPass, Lint, LintLevel},
    parse::{Call, Expression, Statement, StatementBox},
    Config, FileId,
};
use codespan_reporting::diagnostic::{Diagnostic, Label};

#[derive(Debug, PartialEq)]
pub struct UnassignedConstructor;
impl Lint for UnassignedConstructor {
    fn explanation() -> &'static str {
        "Invoking a constructor function without saving the new struct is often a mistake. If the constructor is saving a refernce of itself within its own declaration, this should still be given a wrapper function so that the behavior is not hidden. Avoiding this as an intentional pattern allows this lint to better alert you to mistakes."
    }

    fn default_level() -> LintLevel {
        LintLevel::Warn
    }

    fn tag() -> &'static str {
        "unassigned_constructor"
    }
}

impl EarlyStatementPass for UnassignedConstructor {
    fn visit_statement_early(statement_box: &StatementBox, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let Statement::Expression(expression_box) = statement_box.statement() {
            if let Expression::Call(Call { uses_new: true, .. }) = expression_box.expression() {
                reports.push(
                    Self::diagnostic(config)
                        .with_message("Unassigned constructor")
                        .with_labels(vec![
                            Label::primary(expression_box.file_id(), expression_box.span())
                                .with_message("the newly created struct is never visibly assigned to a value"),
                        ]),
                );
            }
        }
    }
}
