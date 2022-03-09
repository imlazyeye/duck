use crate::{
    lint::{EarlyExpressionPass, EarlyStatementPass, Lint, LintLevel},
    parse::{Expression, ExpressionBox, ParseVisitor, Statement, StatementBox},
    FileId,
};
use codespan_reporting::diagnostic::{Diagnostic, Label};

#[derive(Debug, PartialEq)]
pub struct UnnecessaryGrouping;
impl Lint for UnnecessaryGrouping {
    fn explanation() -> &'static str {
        "Parenthesis around an expression that do not change how the logic is executed are redundant and can be removed."
    }

    fn default_level() -> LintLevel {
        LintLevel::Warn
    }

    fn tag() -> &'static str {
        "unnecessary_grouping"
    }
}

impl UnnecessaryGrouping {
    fn test(expression_box: &ExpressionBox, config: &crate::Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let Expression::Grouping(grouping) = expression_box.expression() {
            let (left_token, right_token) = grouping.parenthesis();
            reports.push(
                Self::diagnostic(config)
                    .with_message("Unnecessary grouping")
                    .with_labels(vec![
                        Label::primary(expression_box.file_id(), left_token.span),
                        Label::primary(expression_box.file_id(), right_token.span),
                    ]),
            );
        }
    }
}

impl EarlyExpressionPass for UnnecessaryGrouping {
    fn visit_expression_early(
        expression_box: &ExpressionBox,
        config: &crate::Config,
        reports: &mut Vec<Diagnostic<FileId>>,
    ) {
        match expression_box.expression() {
            // These are the blessed expressions that utilize groupings in meaningful ways
            Expression::Logical(_) | Expression::Equality(_) | Expression::Evaluation(_) | Expression::Unary(_) => {}

            // This is style preference, which instead is linted by `condition_wrapper`.
            Expression::Ternary(_) => {}

            // These should not directly own groupings
            Expression::FunctionDeclaration(_)
            | Expression::NullCoalecence(_)
            | Expression::Postfix(_)
            | Expression::Access(_)
            | Expression::Call(_)
            | Expression::Grouping(_)
            | Expression::Literal(_)
            | Expression::Identifier(_) => {
                expression_box.visit_child_expressions(|expr| Self::test(expr, config, reports))
            }
        }
    }
}

impl EarlyStatementPass for UnnecessaryGrouping {
    fn visit_statement_early(
        statement_box: &StatementBox,
        config: &crate::Config,
        reports: &mut Vec<Diagnostic<FileId>>,
    ) {
        match statement_box.statement() {
            // These are a style preference, which instead is linted by `condition_wrapper`.
            Statement::TryCatch(_)
            | Statement::ForLoop(_)
            | Statement::WithLoop(_)
            | Statement::RepeatLoop(_)
            | Statement::DoUntil(_)
            | Statement::WhileLoop(_)
            | Statement::If(_)
            | Statement::Switch(_) => {}

            // These should not directly own groupings
            Statement::Return(_) | Statement::Throw(_) | Statement::Delete(_) | Statement::Assignment(_) => {
                statement_box.visit_child_expressions(|expr| Self::test(expr, config, reports))
            }
            _ => {}
        };
    }
}
