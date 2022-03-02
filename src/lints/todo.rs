use crate::{
    lint::{EarlyExpressionPass, Lint, LintLevel, LintReport},
    parsing::{Call, Expression},
    utils::Span,
    Config,
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
    fn visit_expression_early(config: &Config, expression: &Expression, span: Span, reports: &mut Vec<LintReport>) {
        if let Expression::Call(Call { left, .. }) = expression {
            if let Expression::Identifier(identifier) = left.expression() {
                if identifier.name == config.todo_keyword {
                    Self::report("Use of todo marker", ["Remove this todo marker".into()], span, reports)
                }
            }
        }
    }
}
