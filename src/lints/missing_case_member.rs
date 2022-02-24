use crate::{utils::Span, Lint, LintCategory, LintReport};

#[derive(Debug, PartialEq)]
pub struct MissingCaseMember;
impl Lint for MissingCaseMember {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
            display_name: "Missing case member".into(),
            tag: Self::tag(),
            category: Self::category(),
            explanation:  "Switch statements matching over an enum typically want to cover all possible cases if they do not implement a default case.",
            suggestions:  vec![
            "Add cases for the missing members".into(),
            "Remove the imtentional crash from your default case".into(),
        ],
            span,
        }
    }

    fn category() -> LintCategory {
        LintCategory::Correctness
    }

    fn tag() -> &'static str {
        "missing_case_member"
    }

    // fn run(duck: &Duck) -> Vec<LintReport> {
    //     let mut reports = vec![];
    //     for switch in duck.switches() {
    //         if let GmlSwitchStatementDefault::TypeAssert(type_name) = switch.default_case() {
    //             if let Some(gml_enum) = duck.enums().iter().find(|e| e.name() == type_name) {
    //                 let missing_members = gml_enum
    //                     .members()
    //                     .iter()
    //                     .filter(|member| {
    //                         !matches!(member.name(), "Len".into() | "LEN".into() | "count".into() | "COUNT".into())
    //                     })
    //                     .any(|member| {
    //                         !switch.cases().contains(&format!(
    //                             "{}.{}".into(),
    //                             gml_enum.name(),
    //                             member.name()
    //                         ))
    //                     });

    //                 if missing_members {
    //                     reports.push(LintReport {
    //                         span: switch.span().clone(),
    //                     })
    //                 }
    //             }
    //         }
    //     }
    //     reports
    // }
}
