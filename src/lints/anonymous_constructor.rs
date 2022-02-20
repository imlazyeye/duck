use crate::{
    parsing::expression::{Expression, Function},
    Duck, Lint, LintCategory, LintReport, Position,
};

#[derive(Debug, PartialEq)]
pub struct AnonymousConstructor;
impl Lint for AnonymousConstructor {
    fn generate_report(position: Position) -> LintReport {
        LintReport {
            display_name: "Use of an anonymous constructor",
            tag: "anonymous_constructor",
            explanation: "Constructors should be reserved for larger, higher scoped types.",
            suggestions: vec![
                "Change this to a named function",
                "Change this to a function that returns a struct literal",
            ],
            category: LintCategory::Style,
            position,
        }
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
            reports.push(Self::generate_report(position.clone()))
        }
    }
}
