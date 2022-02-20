use crate::{parsing::expression::Expression, Duck, Lint, LintCategory, LintReport, Position};

#[derive(Debug, PartialEq)]
pub struct ConstructorWithoutNew;
impl Lint for ConstructorWithoutNew {
    fn generate_report(position: Position) -> LintReport {
        LintReport {
			display_name: "Invokation of constructor without `new`",
			tag: "constructor_without_new",
			explanation: "Constructors invoked without the `new` keyword do not return the newly constructed struct.",
			suggestions: vec!["Add the `new` operator before the call"],
			category: LintCategory::Correctness,
			position,
		}
    }
}
