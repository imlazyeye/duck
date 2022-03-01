use crate::{
    analysis::GlobalScope, lint::LateStatementPass, parsing::Statement, utils::Span, Lint, LintLevel, LintReport,
};
use itertools::Itertools;

#[derive(Debug, PartialEq)]
pub struct MissingCaseMember;
impl Lint for MissingCaseMember {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
            display_name: "Missing case member".into(),
            tag: Self::tag(),
            default_level: Self::default_level(),
            explanation: "Switch statements matching over an enum typically want to cover all possible cases if they do not implement a default case.",
            suggestions: vec![
                "Add cases for the missing members".into(),
                "Remove the intentional crash from your default case".into(),
            ],
            span,
        }
    }

    fn tag() -> &'static str {
        "missing_case_member"
    }

    fn default_level() -> LintLevel {
        LintLevel::Warn
    }
}

impl LateStatementPass for MissingCaseMember {
    fn visit_statement_late(
        config: &crate::Config,
        environment: &GlobalScope,
        statement: &crate::parsing::Statement,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Statement::Switch(switch) = statement {
            // Ignore switches that don't pertain to this lint
            // TODO: Check for user supplied crash calls here, and enable the lint if they're in the default
            // body!
            if switch.cases().is_empty() || !switch.all_case_members_dot_access() || switch.default_case().is_some() {
                return;
            }

            // See if this is potentially switching over an enum
            let gml_enum = if let Some(enum_name) = switch.potential_enum_type() {
                if let Some(gml_enum) = environment.find_enum(enum_name) {
                    gml_enum
                } else {
                    // We don't recognize this enum -- abort
                    return;
                }
            } else {
                return;
            };

            // Let's assume the user isn't matching over multiple types (`multi_type_switch`
            // will catch that) and check to make sure that every member of the
            // enum is present. We could check here for EXTRA values -- as in, a
            // case that has a member of the enum that does not exist -- but
            // that won't compile in GM anyway, so we will ignore the possibility.
            let mut member_names_discovered = vec![];
            for case in switch.cases().iter() {
                // Retrieve the dot access (we made sure this `unwrap` is safe with
                // `all_case_members_dot_access` earlier!)
                let (left, right) = case.identity().expression().as_dot_access().unwrap();

                // We are not safe to assume that the left and right are identifiers. It would
                // be invalid gml if they weren't, but we don't want to panic
                // regardless.

                if let Some(this_identity_enum) = left.as_identifier() {
                    if this_identity_enum.name != gml_enum.name {
                        // The user has different enums in the same switch statement -- abandon this
                        // lint, and rely on `multi_type_switch`
                        return;
                    }
                } else {
                    return; // invalid gml -- abandon this lint
                }
                if let Some(member_identifier) = right.as_identifier() {
                    member_names_discovered.push(member_identifier.name.as_str());
                } else {
                    return; // invalid gml -- abandon this lint
                };
            }

            // We have now collected all of members in this switch. Let's gather any missing
            // members of the enum, and reduce them down into a string that
            // lists them out.
            let ignore_name = &config.length_enum_member_name;
            let missing_members = gml_enum
                .members
                .iter()
                .map(|member| member.name())
                .filter(|member| ignore_name != member && !member_names_discovered.contains(member))
                .join(", ");

            // If we have any, make a report!
            if !missing_members.is_empty() {
                reports.push(Self::generate_report_with(
                    span,
                    format!("Missing case members: {}", missing_members),
                    [],
                ));
            }
        }
    }
}
