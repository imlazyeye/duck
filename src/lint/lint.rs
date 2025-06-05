use crate::{
    Config, FileId,
    parse::{Ast, Expr, Stmt},
};
use codespan_reporting::diagnostic::{Diagnostic, Severity};
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

    /// Creates a diagnostic based on the user's lint level for this lint.
    #[must_use = "Diagnostics made by lints must be put into the reports collection."]
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
        .with_notes(vec![format!(
            "{}: for more information, run `{}`",
            "Note".bold(),
            format!("duck explain {}", Self::tag()).bold(),
        )])
    }
}

/// Lints who run a pass on an entire Ast.
pub trait AstPass {
    /// Runs on the Ast in the Ast pass.
    fn visit_ast(ast: &Ast, config: &Config, reports: &mut Vec<Diagnostic<FileId>>);
}

/// Lints who run an early pass on statements (before type information has been
/// collected).
pub trait EarlyStmtPass {
    /// Runs on statements in the early pass.
    fn visit_stmt_early(stmt: &Stmt, config: &Config, reports: &mut Vec<Diagnostic<FileId>>);
}

/// Lints who run an early pass on expressions (before type information has been
/// collected).
pub trait EarlyExprPass {
    /// Runs on expressions in the early pass.
    fn visit_expr_early(expr: &Expr, config: &Config, reports: &mut Vec<Diagnostic<FileId>>);
}

/// Lints who run a late pass on statements (after type information has been
/// collected).
pub trait LateStmtPass {
    /// Runs on statements in the late pass.
    fn visit_stmt_late(stmt: &Stmt, config: &Config, reports: &mut Vec<Diagnostic<FileId>>);
}

/// Lints who run a late pass on expresions (after type information has been
/// collected).
pub trait LateExprPass {
    /// Runs on expressions in the late pass.
    fn visit_expr_late(expr: &Expr, config: &Config, reports: &mut Vec<Diagnostic<FileId>>);
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
impl From<Severity> for LintLevel {
    fn from(s: Severity) -> Self {
        match s {
            Severity::Bug => LintLevel::Deny,
            Severity::Error => LintLevel::Deny,
            Severity::Warning => LintLevel::Warn,
            Severity::Note => LintLevel::Allow,
            Severity::Help => LintLevel::Allow,
        }
    }
}

/// The origin of lint level for a lint.
pub enum LintLevelSetting {
    /// The lint level was established by its default settings.
    Default(LintLevel),
    /// The lint level was established by the user's configuration.
    ConfigSpecified(LintLevel),
}
impl core::ops::Deref for LintLevelSetting {
    type Target = LintLevel;
    fn deref(&self) -> &Self::Target {
        match self {
            LintLevelSetting::Default(level) | LintLevelSetting::ConfigSpecified(level) => level,
        }
    }
}
