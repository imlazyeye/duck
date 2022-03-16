use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyExprPass, Lint, LintLevel},
    parse::{Equality, EqualityOp, Expr, ExprType, Token, TokenType},
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

impl EarlyExprPass for SingleEqualsComparison {
    fn visit_expr_early(expr: &Expr, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let ExprType::Equality(Equality {
            op:
                EqualityOp::Equal(Token {
                    token_type: TokenType::Equal,
                    span,
                }),
            ..
        }) = expr.inner()
        {
            reports.push(
                Self::diagnostic(config)
                    .with_message("Comparison with `=`")
                    .with_labels(vec![
                        Label::primary(expr.file_id(), *span).with_message("use `==` instead of `=`"),
                    ]),
            );
        }
    }
}
