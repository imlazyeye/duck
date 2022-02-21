use crate::{Lint, LintCategory, LintReport, Span};

#[derive(Debug, PartialEq)]
pub struct MissingCaseMember;
impl Lint for MissingCaseMember {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
            display_name: "Missing case member",
            tag: "missing_case_member",
            category:  LintCategory::Correctness,
            explanation:  "Switch statements matching over an enum typically want to cover all possible cases if they do not implement a default case.",
            suggestions:  vec![
            "Add cases for the missing members",
            "Remove the imtentional crash from your default case",
        ],
            span,
        }
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
    //                         !matches!(member.name(), "Len" | "LEN" | "count" | "COUNT")
    //                     })
    //                     .any(|member| {
    //                         !switch.cases().contains(&format!(
    //                             "{}.{}",
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
