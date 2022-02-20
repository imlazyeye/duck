use crate::{
    parsing::{expression::Expression, statement::Statement},
    Duck, Lint, LintCategory, LintReport, Position,
};

pub struct MissingDefaultCase;
impl Lint for MissingDefaultCase {
    fn tag() -> &'static str {
        "missing_default_case"
    }

    fn display_name() -> &'static str {
        "Missing default case"
    }

    fn explanation() -> &'static str {
        "Switch statements are often used to express all possible outcomes of a limited data set, but by not implementing a default case, no code will run to handle any alternate or unexpected values."
    }

    fn suggestions() -> Vec<&'static str> {
        vec!["Add a default case to the switch statement"]
    }

    fn category() -> LintCategory {
        LintCategory::Pedantic
    }

    fn visit_statement(
        duck: &Duck,
        statement: &Statement,
        position: &Position,
        reports: &mut Vec<LintReport>,
    ) {
        if let Statement::Switch(_, _, None) = statement {
            reports.push(LintReport {
                position: position.clone(),
            })
        }
    }
}
