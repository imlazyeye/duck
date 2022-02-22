use super::british::BRITISH_TO_AMERICAN_KEYWORDS;
use crate::{parsing::expression::Expression, Lint, LintCategory, LintReport, Span};

#[derive(Debug, PartialEq)]
pub struct American;
impl Lint for American {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
			display_name: "Use of American spelling".into(),
			tag: "american",
			explanation: "GML has many duplicated function names for the sake of supporting both British and American spelling. For consistency, codebases should stick to one.",
			suggestions: vec![],
			category: LintCategory::Style,
			span,
		}
    }

    fn visit_expression(
        _duck: &crate::Duck,
        expression: &crate::parsing::expression::Expression,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Expression::Call(caller, _, _) = expression {
            if let Expression::Identifier(name) = caller.expression() {
                if let Some(british_spelling) =
                    // see british.rs
                    BRITISH_TO_AMERICAN_KEYWORDS.get_by_right(name.as_str())
                {
                    reports.push(Self::generate_report_with(
                        span,
                        format!("Use of American spelling: {}", name),
                        [format!("Use `{}` instead", british_spelling)],
                    ))
                }
            }
        }
    }
}
