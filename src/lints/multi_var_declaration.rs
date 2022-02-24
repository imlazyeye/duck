use crate::{parsing::statement::Statement, utils::Span, Duck, Lint, LintCategory, LintReport};

#[derive(Debug, PartialEq)]
pub struct MultiVarDeclaration;
impl Lint for MultiVarDeclaration {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
			display_name: "Multiple local variables declared at once".into(),
            tag: Self::tag(),
			explanation: "While GML allows you to create multiple local variables at once, it can often lead to confusing syntax that would read better with each variable seperated.",
			suggestions: vec!["Break these down into seperate declarations".into()],
			category: Self::category(),
			span,
		}
    }

    fn category() -> LintCategory {
        LintCategory::Style
    }

    fn tag() -> &'static str {
        "multi_var_declaration"
    }

    fn visit_statement(
        _duck: &Duck,
        statement: &Statement,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Statement::LocalVariableSeries(vars) = statement {
            if vars.len() > 1 {
                reports.push(Self::generate_report(span));
            }
        }
    }
}
