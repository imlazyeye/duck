use crate::{
    analyze::GlobalScope,
    lint::{LateStatementPass, Lint, LintLevel, LintReport},
    parse::{Span, Statement},
};
use itertools::Itertools;

#[derive(Debug, PartialEq)]
pub struct MissingCaseMember;
impl Lint for MissingCaseMember {
    fn explanation() -> &'static str {
        "Switch statements matching over an enum typically want to cover all possible cases if they do not implement a default case."
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
        statement: &crate::parse::Statement,
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

                // We are not safe to assume that the left and right are identifiers.
                if let Some(this_identity_enum) = left.as_identifier() {
                    if this_identity_enum.name != gml_enum.name {
                        // The user has different enums in the same switch statement -- abandon this
                        // lint, and rely on `multi_type_switch`
                        return;
                    }
                } else {
                    return; // INVALID_GML: non-constant in case expression
                }
                if let Some(member_identifier) = right.as_identifier() {
                    member_names_discovered.push(member_identifier.name.as_str());
                } else {
                    return; // INVALID_GML: non-constant in case expression
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
                Self::report(
                    format!("Missing case members: {}", missing_members),
                    ["Add cases for the missing members".into()],
                    span,
                    reports,
                );
            }
        }
    }
}
