use crate::{Duck, GmlSwitchStatementDefault, Lint, LintCategory, LintReport};

pub struct MissingCaseMember;
impl Lint for MissingCaseMember {
    fn tag() -> &'static str {
        "missing_case_member"
    }

    fn display_name() -> &'static str {
        "Missing case member"
    }

    fn explanation() -> &'static str {
        "Switch statements matching over an enum typically want to cover all possible cases if they do not implement a default case."
    }

    fn suggestions() -> Vec<&'static str> {
        vec![
            "Add cases for the missing members",
            "Remove the imtentional crash from your default case",
        ]
    }

    fn category() -> LintCategory {
        LintCategory::Correctness
    }

    fn run(duck: &Duck) -> Vec<LintReport> {
        let mut reports = vec![];
        for switch in duck.switches() {
            if let GmlSwitchStatementDefault::TypeAssert(type_name) = switch.default_case() {
                if let Some(gml_enum) = duck.enums().iter().find(|e| e.name() == type_name) {
                    let missing_members = gml_enum
                        .members()
                        .iter()
                        .filter(|member| {
                            !matches!(member.name(), "Len" | "LEN" | "count" | "COUNT")
                        })
                        .any(|member| {
                            !switch.cases().contains(&format!(
                                "{}.{}",
                                gml_enum.name(),
                                member.name()
                            ))
                        });

                    if missing_members {
                        reports.push(LintReport {
                            position: switch.position().clone(),
                        })
                    }
                }
            }
        }
        reports
    }
}
