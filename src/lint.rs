use crate::{
    parsing::{expression::Expression, statement::Statement, Token},
    Duck, Position,
};

/// An individual lint in duck.
///
/// Lints should be named after the *bad* action, not the good one. For example,
/// a lint that prevents switch statements from having no default case should be
/// called `MissingDefaultCase`, not, say, `DefaultCaseInSwitch`. This makes tagging
/// read more clearly (ie: `#[allow(missing_default_case)])`).
pub trait Lint {
    /// The string representation of this lint used for referencing it in code.
    /// For example, the lint `"MissingDefaultCase"` should return a string like
    /// `"missing_default_case"`.
    fn tag() -> &'static str;

    /// The title of the lint as displayed when it fires into the output.
    fn display_name() -> &'static str;

    /// A justification for this lint, expressing why it may be desirable to enable.
    fn explanation() -> &'static str;

    /// A collection of suggestions on how to avoid this lint that will be displayed to the user
    /// when this lint fires.
    fn suggestions() -> Vec<&'static str>;

    /// The [LintCategory] this lint belongs to.
    fn category() -> LintCategory;

    /// Ran on all tokens.
    fn visit_token(
        duck: &Duck,
        expression: &Token,
        position: &Position,
        reports: &mut Vec<LintReport>,
    ) {
    }

    /// Ran on all expressions.
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    fn visit_expression(
        duck: &Duck,
        expression: &Expression,
        position: &Position,
        reports: &mut Vec<LintReport>,
    ) {
    }

    /// Ran on all statements.
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    fn visit_statement(
        duck: &Duck,
        statement: &Statement,
        position: &Position,
        reports: &mut Vec<LintReport>,
    ) {
    }
}

/// The three different levels a lint can be set to, changing how it will be treated.
#[derive(Debug, Copy, Clone, enum_map::Enum, serde::Serialize, serde::Deserialize)]
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
    pub position: Position,
}
