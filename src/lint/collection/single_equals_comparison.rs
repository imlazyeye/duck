use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyExpressionPass, Lint, LintLevel},
    parse::{Equality, EqualityOperator, Expression, ExpressionBox, Token, TokenType},
    Config, FileId,
};

#[derive(Debug, PartialEq)]
pub struct SingleEqualsComparison;
impl Lint for SingleEqualsComparison {
    fn explanation() -> &'static str {
        "The single-equals token can be used for both assignments and equalities in gml. This is atypical of most languages, and can lead to inconsistancies or bugs in projects."
    }

    fn default_level() -> LintLevel {
        LintLevel::Warn
    }

    fn tag() -> &'static str {
        "single_equals_comparison"
    }
}

impl EarlyExpressionPass for SingleEqualsComparison {
    fn visit_expression_early(expression_box: &ExpressionBox, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let Expression::Equality(Equality {
            operator:
                EqualityOperator::Equal(Token {
                    token_type: TokenType::Equal,
                    ..
                }),
            ..
        }) = expression_box.expression()
        {
            reports.push(
                Self::diagnostic(config)
                    .with_message("Comparison with `=`")
                    .with_labels(vec![
                        Label::primary(expression_box.file_id(), expression_box.span())
                            .with_message("use `==` instead of `=`"),
                    ]),
            );
        }
    }
}
