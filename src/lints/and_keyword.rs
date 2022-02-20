use crate::{parsing::Token, Duck, Lint, LintCategory, LintReport, Position};

#[derive(Debug, PartialEq)]
pub struct AndKeyword;
impl Lint for AndKeyword {
    fn generate_report(position: Position) -> LintReport {
        LintReport {
			display_name: "Use of `and`",
			tag: "and_keyword",
			explanation: "GML supports both `and` and `&&` to refer to logical and -- `&&` is more consistent with other languages and is preferred.",
			suggestions: vec!["Use `&&` instead of `and`"],
			category: LintCategory::Style,
			position,
		}
    }

    fn visit_token(duck: &Duck, token: &Token, position: &Position, reports: &mut Vec<LintReport>) {
        if token == &Token::And {
            reports.push(Self::generate_report(position.clone()))
        }
    }
}
