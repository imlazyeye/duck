use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyExpressionPass, Lint, LintLevel},
    parse::{Equality, EqualityOperator, Expression, ExpressionBox, Literal},
    Config, FileId,
};

#[derive(Debug, PartialEq)]
pub struct BoolEquality;
impl Lint for BoolEquality {
    fn explanation() -> &'static str {
        "Comparing a bool with a bool literal is more verbose than neccesary."
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "bool_equality"
    }
}

impl EarlyExpressionPass for BoolEquality {
    fn visit_expression_early(expression_box: &ExpressionBox, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let Expression::Equality(Equality {
            left,
            operator: EqualityOperator::Equal(_),
            right,
        }) = expression_box.expression()
        {
            if let Some(literal) = right.expression().as_literal() {
                reports.push(match literal {
                    Literal::True => Self::diagnostic(config)
                        .with_message("Equality check with `true`")
                        .with_labels(vec![
                            Label::primary(right.file_id(), right.span()).with_message("this can be omitted"),
                        ]),
                    Literal::False => Self::diagnostic(config)
                        .with_message("Equality check with `false`")
                        .with_labels(vec![
                            Label::primary(right.file_id(), right.span()).with_message("this can be omitted..."),
                            Label::secondary(left.file_id(), left.span().0..left.span().0)
                                .with_message("...if you add a not operator here (`!`, `not`)"),
                        ]),
                    _ => return,
                });
            }
        }
    }
}
