use crate::{utils::Span, Lint, LintReport, LintLevel};

#[derive(Debug, PartialEq)]
pub struct OrKeyword;
impl Lint for OrKeyword {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
			display_name: "Use of `or`".into(),
            tag: Self::tag(),
			explanation: "GML supports both `or` and `||` to refer to logical or -- `||` is more consistent with other languages and is preferred.",
			suggestions: vec!["Use `||` instead of `or`".into()],
			default_level: Self::default_level(),
			span,
		}
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "or_keyword"
    }

    // fn run(duck: &Duck) -> Vec<LintReport> {
    //     let mut reports = vec![];
    //     for keyword in duck.keywords() {
    //         if let (Token::Or, span) = keyword {
    //             reports.push(LintReport {
    //                 span: span,
    //             })
    //         }
    //     }
    //     reports
    // }
}
