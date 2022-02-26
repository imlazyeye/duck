use crate::{
    lint::EarlyExpressionPass, parsing::expression::Expression, utils::Span, Config, Lint,
    LintCategory, LintReport,
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
			category: Self::category(),
			span,
		}
    }

    fn category() -> LintCategory {
        LintCategory::Strict
    }

    fn tag() -> &'static str {
        "todo"
    }
}

impl EarlyExpressionPass for Todo {
    fn visit_expression_early(
        config: &Config,
        expression: &Expression,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Some(todo_keyword) = config.todo_keyword() {
            if let Expression::Call(caller, _, _) = expression {
                if let Expression::Identifier(name) = caller.expression() {
                    if name == todo_keyword {
                        reports.push(Self::generate_report(span))
                    }
                }
            }
        }
    }
}
