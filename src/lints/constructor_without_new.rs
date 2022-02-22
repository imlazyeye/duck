use crate::{Lint, LintCategory, LintReport, Span};

#[derive(Debug, PartialEq)]
pub struct ConstructorWithoutNew;
impl Lint for ConstructorWithoutNew {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
			display_name: "Invokation of constructor without `new`".into(),
            tag: Self::tag(),
			explanation: "Constructors invoked without the `new` keyword do not return the newly constructed struct.",
			suggestions: vec!["Add the `new` operator before the call".into()],
			category: Self::category(),
			span,
		}
    }

    fn category() -> LintCategory {
        LintCategory::Correctness
    }

    fn tag() -> &'static str {
        "constructor_without_new"
    }
}
