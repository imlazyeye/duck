use crate::{parsing::statement::Statement, Duck, Lint, LintCategory, LintReport, Span};

#[derive(Debug, PartialEq)]
pub struct MultiVarDeclaration;
impl Lint for MultiVarDeclaration {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
			display_name: "Multiple local variables declared at once".into(),
			tag: "multi_var_declaration",
			explanation: "While GML allows you to create multiple local variables at once, it can often lead to confusing syntax that would read better with each variable seperated.",
			suggestions: vec!["Break these down into seperate declarations".into()],
			category: LintCategory::Style,
			span,
		}
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
