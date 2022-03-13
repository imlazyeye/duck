use crate::{
    lint::{EarlyStmtPass, Lint, LintLevel},
    parse::{Call, ExprType, Stmt, StmtType},
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

impl EarlyStmtPass for UnassignedConstructor {
    fn visit_stmt_early(stmt: &Stmt, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let StmtType::Expr(expr) = stmt.inner() {
            if let ExprType::Call(Call { uses_new: true, .. }) = expr.inner() {
                reports.push(
                    Self::diagnostic(config)
                        .with_message("Unassigned constructor")
                        .with_labels(vec![
                            Label::primary(expr.file_id(), expr.span())
                                .with_message("the newly created struct is never visibly assigned to a value"),
                        ]),
                );
            }
        }
    }
}
