use crate::{
    config::Config,
    gml::Environment,
    parsing::{expression::Expression, statement::Statement},
    utils::FilePreviewUtil,
    utils::Span,
};
use colored::Colorize;

/// An individual lint in duck.
///
/// Lints should be named after the *bad* action, not the good one. For example,
/// a lint that prevents switch statements from having no default case should be
/// called `MissingDefaultCase`, not, say, `DefaultCaseInSwitch`. This makes tagging
/// read more clearly (ie: `#[allow(missing_default_case)])`).
pub trait Lint {
    /// Genreates a LintReport.
    fn generate_report(span: Span) -> LintReport;

    /// Returns the string tag for this Lint.
    fn tag() -> &'static str;

    /// Returns the LintCategory for this Lint.
    fn category() -> LintCategory;

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
}

/// Lints who run an early pass on statements (before type information has been collected).
pub trait EarlyStatementPass {
    fn visit_statement_early(
        config: &Config,
        statement: &Statement,
        span: Span,
        reports: &mut Vec<LintReport>,
    );
}

/// Lints who run an early pass on expressions (before type information has been collected).
pub trait EarlyExpressionPass {
    fn visit_expression_early(
        config: &Config,
        expression: &Expression,
        span: Span,
        reports: &mut Vec<LintReport>,
    );
}

/// Lints who run a late pass on statements (after type information has been collected).
pub trait LateStatementPass {
    fn visit_statement_late(
        config: &Config,
        environment: &Environment,
        statement: &Statement,
        span: Span,
        reports: &mut Vec<LintReport>,
    );
}

/// Lints who run a late pass on expresions (after type information has been collected).
pub trait LateExpressionPass {
    fn visit_expression_late(
        config: &Config,
        environment: &Environment,
        expression: &Expression,
        span: Span,
        reports: &mut Vec<LintReport>,
    );
}

/// The three different levels a lint can be set to, changing how it will be treated.
#[derive(Debug, PartialEq, Copy, Clone, enum_map::Enum, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LintLevel {
    /// These lints will be ignored.
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
pub enum LintLevelSetting {
    Default(LintLevel),
    CodeSpecified(LintLevel),
    ConfigSpecified(LintLevel),
}
impl core::ops::Deref for LintLevelSetting {
    type Target = LintLevel;
    fn deref(&self) -> &Self::Target {
        match self {
            LintLevelSetting::Default(level)
            | LintLevelSetting::CodeSpecified(level)
            | LintLevelSetting::ConfigSpecified(level) => level,
        }
    }
}

/// The data from a user-written tag (ie: #[allow(draw_text)])
#[derive(Debug)]
pub struct LintTag(pub String, pub LintLevel);

/// The category a lint falls into. This effects duck's default permission level for all lints.
#[derive(Debug, Copy, Clone, enum_map::Enum)]
pub enum LintCategory {
    /// Code that is outright wrong or useless.
    Correctness,
    /// Code that is irregular and likely doing something unintended.
    Suspicious,
    /// Code that could be written in a more idomatic way.
    Style,
    /// Lints that express strict opinions over GML and depend greatly on preference.
    Strict,
}
impl LintCategory {
    pub fn default_level(&self) -> LintLevel {
        match self {
            LintCategory::Correctness => LintLevel::Deny,
            LintCategory::Suspicious => LintLevel::Warn,
            LintCategory::Style => LintLevel::Warn,
            LintCategory::Strict => LintLevel::Allow,
        }
    }
}

/// A report returned by a lint if it fails.
#[derive(Debug)]
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
    pub fn generate_string(&self, config: &Config, preview: &FilePreviewUtil) -> String {
        let level = config.get_level_for_lint(self.tag, self.category);
        let level_string = match *level {
            LintLevel::Allow => "allowed".bright_black().bold(), // I dunno why you'd ever do this, but for now I don't wanna crash...
            LintLevel::Warn => "warning".yellow().bold(),
            LintLevel::Deny => "error".bright_red().bold(),
        };
        let path_message = preview.path_message();
        let snippet_message = preview.snippet_message();
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
        let note_message = match level {
            LintLevelSetting::Default(_) => "",
            LintLevelSetting::CodeSpecified(_) => {
                "\n note: This lint was requested by the line above it."
            }
            LintLevelSetting::ConfigSpecified(_) => {
                "\n note: This lint was activated by your config,"
            }
        }
        .to_string()
        .bold()
        .bright_black();
        format!(
            "{}: {}\n{path_message}\n{snippet_message}{suggestion_message}{note_message}\n",
            level_string,
            self.display_name.bright_white(),
        )
    }

    /// Get the lint report's tag.
    pub fn tag(&self) -> &str {
        self.tag
    }

    /// Get the lint report's category.
    pub fn category(&self) -> LintCategory {
        self.category
    }
}
