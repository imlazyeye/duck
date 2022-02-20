use crate::{
    parsing::{expression::Expression, statement::Statement},
    Duck, Lint, LintCategory, LintReport, Position,
};

pub struct Globalvar;
impl Lint for Globalvar {
    fn tag() -> &'static str {
        "globalvar"
    }

    fn display_name() -> &'static str {
        "Use of `globalvar`"
    }

    fn explanation() -> &'static str {
        "Globalvars are depricated and reduce readability."
    }

    fn suggestions() -> Vec<&'static str> {
        vec![
            "Use the `global` keyword",
            "Scope this variable to an individual object",
        ]
    }

    fn category() -> LintCategory {
        LintCategory::Correctness
    }

    fn visit_statement(
        duck: &Duck,
        statement: &Statement,
        position: &Position,
        reports: &mut Vec<LintReport>,
    ) {
        if let Statement::GlobalvarDeclaration(..) = statement {
            reports.push(LintReport {
                position: position.clone(),
            })
        }
    }
}
