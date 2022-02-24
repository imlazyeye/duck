use crate::{utils::Span, Lint, LintCategory, LintReport};

#[derive(Debug, PartialEq)]
pub struct NoSpaceBeginingComment;
impl Lint for NoSpaceBeginingComment {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
			display_name: "No space begining comment".into(),
            tag: Self::tag(),
			explanation: "Comments should begin with a space after them to increase readability and consistency.",
			suggestions: vec!["Add a space to the begining of the comment".into()],
			category: Self::category(),
			span,
		}
    }

    fn category() -> LintCategory {
        LintCategory::Style
    }

    fn tag() -> &'static str {
        "no_space_begining_comment"
    }
}
