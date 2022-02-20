use crate::{
    parsing::{expression::Expression, statement::Statement},
    Duck, Lint, LintCategory, LintReport, Position,
};

pub struct Exit;
impl Lint for Exit {
    fn tag() -> &'static str {
        "exit"
    }

    fn display_name() -> &'static str {
        "Use of `exit`"
    }

    fn explanation() -> &'static str {
        "`return` can always be used in place of exit, which provides more consistency across your codebase."
    }

    fn suggestions() -> Vec<&'static str> {
        vec!["Use `return` instead of `exit`"]
    }

    fn category() -> LintCategory {
        LintCategory::Style
    }

    fn visit_statement(
        duck: &Duck,
        statement: &Statement,
        position: &Position,
        reports: &mut Vec<LintReport>,
    ) {
        if *statement == Statement::Exit {
            reports.push(LintReport {
                position: position.clone(),
            })
        }
    }
}
