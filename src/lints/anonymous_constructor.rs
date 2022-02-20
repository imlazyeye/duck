use crate::{
    parsing::expression::{Expression, Function},
    Duck, Lint, LintCategory, LintReport, Position,
};

pub struct AnonymousConstructor;
impl Lint for AnonymousConstructor {
    fn tag() -> &'static str {
        "anonymous_constructor"
    }

    fn display_name() -> &'static str {
        "Use of an anonymous constructor"
    }

    fn explanation() -> &'static str {
        "Constructors should be reserved for larger, higher scoped types."
    }

    fn suggestions() -> Vec<&'static str> {
        vec![
            "Change this to a named function",
            "Change this to a function that returns a struct literal",
        ]
    }

    fn category() -> crate::LintCategory {
        LintCategory::Style
    }

    fn visit_expression(
        duck: &Duck,
        expression: &Expression,
        position: &Position,
        reports: &mut Vec<LintReport>,
    ) {
        if let Expression::FunctionDeclaration(Function::Anonymous(_, Some(constructor), _, _)) =
            expression
        {
            reports.push(LintReport {
                position: position.clone(),
            })
        }
    }
}
