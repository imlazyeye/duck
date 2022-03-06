use crate::{
    lint::{EarlyStatementPass, Lint, LintLevel},
    parse::{Assignment, Expression, Statement, StatementBox},
    Config, FileId,
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

impl EarlyStatementPass for InvalidAssignment {
    fn visit_statement_early(statement_box: &StatementBox, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let Statement::Assignment(Assignment { left, operator, right }) = statement_box.statement() {
            let is_valid = match left.expression() {
                Expression::FunctionDeclaration(_)
                | Expression::Logical(_)
                | Expression::Equality(_)
                | Expression::Evaluation(_)
                | Expression::NullCoalecence(_)
                | Expression::Ternary(_)
                | Expression::Unary(_)
                | Expression::Postfix(_)
                | Expression::Grouping(_)
                | Expression::Call(_)
                | Expression::Literal(_) => false,
                Expression::Access(_) | Expression::Identifier(_) => true,
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
