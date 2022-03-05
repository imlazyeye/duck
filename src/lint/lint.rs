use crate::{
    analyze::GlobalScope,
    parse::{Expression, Span, Statement},
    Config, FileId,
};
use codespan_reporting::diagnostic::{Diagnostic, Label};
use colored::Colorize;

/// An individual lint in duck.
///
/// Lints should be named after the *bad* action, not the good one. For example,
/// a lint that prevents switch statements from having no default case should be
/// called `MissingDefaultCase`, not, say, `DefaultCaseInSwitch`. This makes
/// tagging read more clearly (ie: `#[allow(missing_default_case)])`).
pub trait Lint {
    /// Returns the string tag for this Lint.
    fn tag() -> &'static str;

    /// Returns the default LintLevel for this Lint.
    fn default_level() -> LintLevel;

    /// Returns an explanation of what the lint does and why it is useful.
    fn explanation() -> &'static str;

    /// Generates a LintReport based on `Lint::generate_report`, but replaces
    /// its name and extends any provided suggestions into it.
    fn report<const COUNT: usize>(
        name: impl Into<String>,
        suggestions: [String; COUNT],
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        reports.push(LintReport {
            span,
            display_name: name.into(),
            suggestions: suggestions.into(),
            tag: Self::tag(),
            default_level: Self::default_level(),
            explanation: Self::explanation(),
        });
    }

    /// Creates a diagnostic based on the user's lint level for this lint.
    fn diagnostic(config: &Config) -> Diagnostic<FileId> {
        let level = config
            .lint_levels
            .iter()
            .find(|(key, _)| key.as_str() == Self::tag())
            .map_or_else(Self::default_level, |(_, level)| *level);
        match level {
            LintLevel::Allow => unreachable!(),
            LintLevel::Warn => Diagnostic::warning(),
            LintLevel::Deny => Diagnostic::error(),
        }
    }
}

/// Lints who run an early pass on statements (before type information has been
/// collected).
pub trait EarlyStatementPass {
    /// Runs on statements in the early pass.
    fn visit_statement_early(config: &Config, statement: &Statement, span: Span, reports: &mut Vec<LintReport>);
}

/// Lints who run an early pass on expressions (before type information has been
/// collected).
pub trait EarlyExpressionPass {
    /// Runs on expressions in the early pass.
    fn visit_expression_early(config: &Config, expression: &Expression, span: Span, reports: &mut Vec<LintReport>);
}

/// Lints who run a late pass on statements (after type information has been
/// collected).
pub trait LateStatementPass {
    /// Runs on statements in the late pass.
    fn visit_statement_late(
        config: &Config,
        environment: &GlobalScope,
        statement: &Statement,
        span: Span,
        reports: &mut Vec<LintReport>,
    );
}

/// Lints who run a late pass on expresions (after type information has been
/// collected).
pub trait LateExpressionPass {
    /// Runs on expressions in the late pass.
    fn visit_expression_late(
        config: &Config,
        environment: &GlobalScope,
        expression: &Expression,
        span: Span,
        reports: &mut Vec<LintReport>,
    );
}

/// The three different levels a lint can be set to, changing how it will be
/// treated.
#[derive(Debug, PartialEq, Copy, Clone, enum_map::Enum, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LintLevel {
    /// These lints will be ignored.
    Allow,
    /// These lints will be reported to the user, but will not fail the run by
    /// default.
    Warn,
    /// These lints will be reported to the user and will fail the run.
    Deny,
}
impl LintLevel {
    /// Converts a string into a lint level.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "allow" => Some(Self::Allow),
            "warn" => Some(Self::Warn),
            "deny" => Some(Self::Deny),
            _ => None,
        }
    }
    /// Converts a lint level into a string.
    pub fn to_str(&self) -> &str {
        match self {
            LintLevel::Allow => "allow",
            LintLevel::Warn => "warn",
            LintLevel::Deny => "deny",
        }
    }
}

/// The origin of lint level for a lint.
pub enum LintLevelSetting {
    /// The lint level was established by its default settings.
    Default(LintLevel),
    /// The lint level was established by a tag in the code.
    CodeSpecified(LintLevel),
    /// The lint level was established by the user's configuration.
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

/// A report returned by a lint if it fails.
#[derive(Debug)]
pub struct LintReport {
    pub(super) display_name: String,
    pub(super) tag: &'static str,
    pub(super) default_level: LintLevel,
    #[allow(dead_code)]
    pub(super) explanation: &'static str,
    pub(super) suggestions: Vec<String>,
    pub(super) span: Span,
}
impl LintReport {
    /// Generates a Diagnostic out of a lint report to be displayed to the user.
    pub fn generate_diagnostic(&self, config: &Config, file_id: FileId) -> Diagnostic<FileId> {
        let level = config.get_lint_level_setting(self.tag, self.default_level);
        let mut suggestions: Vec<String> = self
            .suggestions
            .iter()
            .map(|v| format!("{}: {}", "Hint".bold(), v))
            .collect();
        suggestions.push(format!(
            "Ignore this by placing `// #[allow({})]` above this code or by adjusting your config's settings.",
            self.tag,
        ));
        match *level {
            LintLevel::Allow => Diagnostic::note(),
            LintLevel::Warn => Diagnostic::warning(),
            LintLevel::Deny => Diagnostic::error(),
        }
        .with_message(&self.display_name)
        .with_labels(vec![Label::primary(file_id, self.span.0..self.span.1)])
        .with_notes(suggestions)

        // TODO: Point to the line in the user's config, or the lint tag that activated this
        // match level {
        //     LintLevelSetting::Default(_) => {}
        //     LintLevelSetting::CodeSpecified(_) => {
        //         diagnostic.with_labels(vec!["This lint was requested by the line above
        // it.".into()]);     }
        //     LintLevelSetting::ConfigSpecified(_) => {
        //         diagnostic.with_labels(vec!["This lint was activated by your config.".into()]);
        //     }
        // }
    }

    /// Get the lint report's tag.
    pub fn tag(&self) -> &str {
        self.tag
    }

    /// Get the lint report's default level.
    pub fn default_level(&self) -> LintLevel {
        self.default_level
    }

    /// Get the lint report's span.
    pub fn span(&self) -> Span {
        self.span
    }
}
