use crate::{parsing::Token, Duck, Lint, LintCategory, LintReport, Position};

pub struct AndKeyword;
impl Lint for AndKeyword {
    fn tag() -> &'static str {
        "and_keyword"
    }

    fn display_name() -> &'static str {
        "Use of `and`"
    }

    fn explanation() -> &'static str {
        "GML supports both `and` and `&&` to refer to logical and -- `&&` is more consistent with other languages and is preferred."
    }

    fn suggestions() -> Vec<&'static str> {
        vec!["Use `&&` instead of `and`"]
    }

    fn category() -> crate::LintCategory {
        LintCategory::Style
    }

    fn visit_token(duck: &Duck, token: &Token, position: &Position, reports: &mut Vec<LintReport>) {
        if token == &Token::And {
            reports.push(LintReport {
                position: position.clone(),
            })
        }
    }
}
