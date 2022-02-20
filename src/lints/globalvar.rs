use crate::{
    parsing::{expression::Expression, statement::Statement},
    Duck, Lint, LintCategory, LintReport, Position,
};

#[derive(Debug, PartialEq)]
pub struct Globalvar;
impl Lint for Globalvar {
    fn generate_report(position: Position) -> LintReport {
        LintReport {
            tag: "globalvar",
            display_name: "Use of `globalvar`",
            explanation: "Globalvars are depricated and reduce readability.",
            suggestions: vec![
                "Use the `global` keyword",
                "Scope this variable to an individual object",
            ],
            category: LintCategory::Correctness,
            position,
        }
    }

    fn visit_statement(duck: &Duck, statement: &Statement, position: &Position, reports: &mut Vec<LintReport>) {
        if let Statement::GlobalvarDeclaration(..) = statement {
            reports.push(Self::generate_report(position.clone()))
        }
    }
}
