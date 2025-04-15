use crate::{
    Config, FileId,
    lint::{EarlyStmtPass, Lint, LintLevel},
    parse::{Assignment, ExprKind, Stmt, StmtKind},
};
use codespan_reporting::diagnostic::{Diagnostic, Label};

#[derive(Debug, PartialEq)]
pub struct InvalidAssignment;
impl Lint for InvalidAssignment {
    fn explanation() -> &'static str {
        "Certain assignment patterns are valid in gml but are undefined behavior and have no valid use cases."
    }

    fn default_level() -> LintLevel {
        LintLevel::Deny
    }

    fn tag() -> &'static str {
        "invalid_assignment"
    }
}

impl EarlyStmtPass for InvalidAssignment {
    fn visit_stmt_early(stmt: &Stmt, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let StmtKind::Assignment(Assignment {
            left,
            op: operator,
            right,
        }) = stmt.kind()
        {
            let is_valid = match left.kind() {
                ExprKind::Function(_)
                | ExprKind::Logical(_)
                | ExprKind::Equality(_)
                | ExprKind::Evaluation(_)
                | ExprKind::NullCoalecence(_)
                | ExprKind::Ternary(_)
                | ExprKind::Unary(_)
                | ExprKind::Postfix(_)
                | ExprKind::Grouping(_)
                | ExprKind::Call(_)
                | ExprKind::Literal(_) => false,
                ExprKind::Access(_) | ExprKind::Identifier(_) => true,
            };
            if !is_valid {
                reports.push(
                    Self::diagnostic(config)
                        .with_message("Invalid assignment target")
                        .with_labels(vec![
                            Label::primary(left.file_id(), operator.token().span.start()..right.span().end())
                                .with_message("cannot perform this assignment..."),
                            Label::secondary(left.file_id(), left.span())
                                .with_message("...onto an expression of this type"),
                        ]),
                );
            }
        }
    }
}
