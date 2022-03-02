use crate::{
    lint::{EarlyExpressionPass, Lint, LintLevel, LintReport},
    parsing::{Call, Expression},
    utils::Span,
    Config,
};

#[derive(Debug, PartialEq)]
pub struct Todo;
impl Lint for Todo {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
            display_name: "Use of todo marker".into(),
            tag: Self::tag(),
            explanation: "Todo markers are useful for work-in-progress code, but often are not intended to be permanently in place.",
            suggestions: vec!["Remove this todo marker".into()],
            default_level: Self::default_level(),
            span,
        }
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
                    reports.push(Self::generate_report(span))
                }
            }
        }
    }
}
