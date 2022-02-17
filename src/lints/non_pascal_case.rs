use crate::{Duck, Lint, LintCategory, LintReport};

pub struct NonPascalCase;
impl Lint for NonPascalCase {
    fn tag() -> &'static str {
        "non_pascal_case"
    }

    fn display_name() -> &'static str {
        "Identifier should be SCREAM_CASE"
    }

    fn explanation() -> &'static str {
        "Pascal case is the ideal casing for \"types\" to distinguish them from other values."
    }

    fn suggestions() -> Vec<&'static str> {
        vec!["Change your casing to PascalCase"]
    }

    fn category() -> LintCategory {
        LintCategory::Style
    }

    fn run(duck: &Duck) -> Vec<LintReport> {
        let mut reports = vec![];
        for e in duck.enums() {
            let name = e.name();
            let ideal_name = Duck::pascal_case(name);
            if name != ideal_name {
                reports.push(LintReport {
                    position: e.position().clone(),
                })
            }
        }
        for constructor in duck.constructors() {
            if let Some(name) = constructor.name() {
                let ideal_name = Duck::pascal_case(name);
                if name != &ideal_name {
                    reports.push(LintReport {
                        position: constructor.position().clone(),
                    })
                }
            }
        }
        reports
    }
}
