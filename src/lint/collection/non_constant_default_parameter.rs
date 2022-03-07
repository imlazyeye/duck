use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    analyze::GlobalScope,
    lint::{LateExpressionPass, Lint, LintLevel},
    parse::{Access, Evaluation, Expression, ExpressionBox, Function, Unary, UnaryOperator},
    FileId,
};

#[derive(Debug, PartialEq)]
pub struct NonConstantDefaultParameter;
impl Lint for NonConstantDefaultParameter {
    fn explanation() -> &'static str {
        "Expressive default parameters are not supported in most languages due to their instability and tendency to hide important logic execution from the caller."
    }

    fn default_level() -> LintLevel {
        LintLevel::Warn
    }

    fn tag() -> &'static str {
        "non_constant_default_parameter"
    }
}

impl NonConstantDefaultParameter {
    fn is_constant(expresion_box: &ExpressionBox, global_scope: &GlobalScope) -> bool {
        match expresion_box.expression() {
            Expression::Access(Access::Dot { left, .. }) => left
                .expression()
                .as_identifier()
                .map_or(false, |iden| global_scope.find_enum(&iden.lexeme).is_some()),
            Expression::Unary(Unary {
                operator: UnaryOperator::Positive(_),
                right,
            })
            | Expression::Unary(Unary {
                operator: UnaryOperator::Negative(_),
                right,
            }) => Self::is_constant(right, global_scope),
            Expression::Evaluation(Evaluation { left, right, .. }) => {
                Self::is_constant(left, global_scope) && Self::is_constant(right, global_scope)
            }
            Expression::Literal(_) | Expression::Identifier(_) => true,
            _ => false,
        }
    }
}

impl LateExpressionPass for NonConstantDefaultParameter {
    fn visit_expression_late(
        expression_box: &ExpressionBox,
        config: &crate::Config,
        reports: &mut Vec<Diagnostic<FileId>>,
        global_scope: &GlobalScope,
    ) {
        if let Expression::FunctionDeclaration(Function { parameters, .. }) = expression_box.expression() {
            for param in parameters {
                if let Some(default_value_expression_box) = param.assignment_value() {
                    let constant = Self::is_constant(default_value_expression_box, global_scope);
                    if !constant {
                        reports.push(
                            Self::diagnostic(config)
                                .with_message("Non constant default parameter")
                                .with_labels(vec![
                                    Label::primary(
                                        default_value_expression_box.file_id(),
                                        default_value_expression_box.span(),
                                    )
                                    .with_message("this parameter's default value is not constant"),
                                ]),
                        );
                    }
                }
            }
        }
    }
}
