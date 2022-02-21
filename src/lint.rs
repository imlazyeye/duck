use colored::Colorize;

use crate::{
    parsing::{expression::Expression, statement::Statement},
    Duck, Position, Span,
};

/// An individual lint in duck.
///
/// Lints should be named after the *bad* action, not the good one. For example,
/// a lint that prevents switch statements from having no default case should be
/// called `MissingDefaultCase`, not, say, `DefaultCaseInSwitch`. This makes tagging
/// read more clearly (ie: `#[allow(missing_default_case)])`).
pub trait Lint {
    /// Genreates a LintReport.
    fn generate_report(span: Span) -> LintReport;

    /// Generates a LintReport based on `Lint::generate_report`, but replaces its name
    /// and extends any provided suggestions into it.
    fn generate_report_with<const COUNT: usize>(
        span: Span,
        name: impl Into<String>,
        additional_suggestions: [String; COUNT],
    ) -> LintReport {
        let mut report = Self::generate_report(span);
        report.display_name = name.into();
        report.suggestions.extend(additional_suggestions);
        report
    }

    /// Ran on all expressions.
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    fn visit_expression(
        duck: &Duck,
        expression: &Expression,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
    }

    /// Ran on all statements.
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    fn visit_statement(
        duck: &Duck,
        statement: &Statement,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
    }
}

/// The three different levels a lint can be set to, changing how it will be treated.
#[derive(Debug, PartialEq, Copy, Clone, enum_map::Enum, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LintLevel {
    /// These lints will be ran, but their results will not affect the outcome of duck.
    Allow,
    /// These lints will be reported to the user, but will not fail the run by default.
    Warn,
    /// These lints will be reported to the user and will fail the run.
    Deny,
}
impl LintLevel {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "allow" => Some(Self::Allow),
            "warn" => Some(Self::Warn),
            "deny" => Some(Self::Deny),
            _ => None,
        }
    }
    pub fn to_str(&self) -> &str {
        match self {
            LintLevel::Allow => "allow",
            LintLevel::Warn => "warn",
            LintLevel::Deny => "deny",
        }
    }
}

/// The data from a user-written tag (ie: #[allow(draw_text)])
#[derive(Debug)]
pub struct LintTag(pub String, pub LintLevel);

/// The category a lint falls into. This effects duck's default permission level for all lints.
#[derive(Debug, Copy, Clone, enum_map::Enum)]
pub enum LintCategory {
    /// Code that is outright wrong or useless
    Correctness,
    /// Code that is most likely wrong or useless
    Suspicious,
    /// Code that could be written in a more idomatic way
    Style,
    /// Lints that express strict opinions over GML, or may have false positives
    Pedantic,
}

/// A report returned by a lint if it fails.
pub struct LintReport {
    pub(super) display_name: String,
    pub(super) tag: &'static str,
    pub(super) category: LintCategory,
    #[allow(dead_code)]
    pub(super) explanation: &'static str,
    pub(super) suggestions: Vec<String>,
    pub span: Span,
}
impl LintReport {
    pub fn get_true_level(&self, duck: &Duck) -> LintLevel {
        let user_provided_level = duck.get_user_provided_level(self.tag);
        user_provided_level.unwrap_or_else(|| duck.category_levels[self.category])
    }
    pub fn raise(self, duck: &Duck, position: &Position) {
        let user_provided_level = duck.get_user_provided_level(self.tag);
        let actual_level = self.get_true_level(duck);
        let level_string = match actual_level {
            LintLevel::Allow => return, // allow this!
            LintLevel::Warn => "warning".yellow().bold(),
            LintLevel::Deny => "error".bright_red().bold(),
        };
        let path_message = position.path_message();
        let snippet_message = position.snippet_message();
        let show_suggestions = true;
        let suggestion_message = if show_suggestions {
            let mut suggestions: Vec<String> = self.suggestions.clone();
            suggestions.push(format!(
                "Ignore this by placing `// #[allow({})]` above this code",
                self.tag,
            ));
            format!(
                "\n\n {}: You can resolve this by doing one of the following:\n{}",
                "suggestions".bold(),
                suggestions
                    .iter()
                    .enumerate()
                    .map(|(i, suggestion)| format!("  {}: {}\n", i + 1, suggestion))
                    .collect::<String>(),
            )
        } else {
            "".into()
        };
        let note_message = format!(
            "\n {}: {}",
            "note".bold(),
            if user_provided_level.is_some() {
                "This lint was specifically requested by in line above this source code".into()
            } else {
                format!(
                    "#[{}({})] is enabled by default",
                    actual_level.to_str(),
                    self.tag,
                )
            }
        )
        .bright_black();
        println!(
            "{}: {}\n{path_message}\n{snippet_message}{suggestion_message}{note_message}\n",
            level_string,
            self.display_name.bright_white(),
        );
    }
}
