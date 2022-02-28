use crate::{utils::Span, Lint, LintLevel, LintReport};

#[derive(Debug, PartialEq)]
pub struct ConstructorWithoutNew;
impl Lint for ConstructorWithoutNew {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
            display_name: "Invokation of constructor without `new`".into(),
            tag: Self::tag(),
            explanation: "Constructors invoked without the `new` keyword do not return the newly constructed struct.",
            suggestions: vec!["Add the `new` operator before the call".into()],
            default_level: Self::default_level(),
            span,
        }
    }

    fn default_level() -> LintLevel {
        LintLevel::Deny
    }

    fn tag() -> &'static str {
        "constructor_without_new"
    }
}
