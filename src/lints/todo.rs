use crate::{lint::EarlyExpressionPass, parsing::Expression, utils::Span, Config, Lint, LintLevel, LintReport};

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
        if let Expression::Call(caller, _, _) = expression {
            if let Expression::Identifier(identifier) = caller.expression() {
                if &identifier.name == &config.todo_keyword {
                    reports.push(Self::generate_report(span))
                }
            }
        }
    }
}
