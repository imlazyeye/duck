use crate::{
    lint::EarlyStatementPass,
    parsing::{expression::Expression, statement::Statement},
    utils::Span,
    Duck, Lint, LintCategory, LintReport,
};

#[derive(Debug, PartialEq)]
pub struct StatementParentheticalViolation;
impl Lint for StatementParentheticalViolation {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
			display_name: "Statement Parentheticals".into(),
            tag: Self::tag(),
			explanation: "Parenthesis surrounding statement expressions are optional in GML, resulting in differing opinions on whether or not to use them. You can select either option via the config.",
			suggestions: vec![],
			category: Self::category(),
			span,
		}
    }

    fn category() -> LintCategory {
        LintCategory::Style
    }

    fn tag() -> &'static str {
        "statement_parenthetical_violation"
    }
}

impl EarlyStatementPass for StatementParentheticalViolation {
    fn visit_statement_early(
        duck: &Duck,
        statement: &Statement,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        let expression = match statement {
            Statement::Switch(switch) => Some(switch.matching_value()),
            Statement::If(expression, ..)
            | Statement::DoUntil(_, expression)
            | Statement::While(expression, ..)
            | Statement::With(expression, ..)
            | Statement::Repeat(expression, ..) => Some(expression),
            _ => None,
        };
        if let Some(expression) = expression {
            let has_grouping = matches!(expression.expression(), Expression::Grouping(_));
            if has_grouping && !duck.config().statement_parentheticals {
                reports.push(Self::generate_report_with(
                    span,
                    "Parenthetical in statement expression",
                    [
                        "Remove the wrapping parenthesis from this expression".into(),
                        "Change your preferences for this lint in .duck.toml".into(),
                    ],
                ))
            } else if !has_grouping && duck.config().statement_parentheticals {
                reports.push(Self::generate_report_with(
                    span,
                    "Lacking parenthetical in statement expression",
                    [
                        "Add wrapping parenthesis from this expression".into(),
                        "Change your preferences for this lint in .duck.toml".into(),
                    ],
                ))
            }
        }
    }
}
