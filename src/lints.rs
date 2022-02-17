mod and_keyword;
mod draw_text;
mod global;
mod globalvar;
mod missing_case_members;
mod missing_default_case;
mod mod_keyword;
mod or_keyword;
mod show_debug_message;
mod todo;
mod try_catch;
mod with_loop;
mod draw_sprite;
mod constructor_without_new;
mod single_switch_case;
mod exit;
mod too_many_arguments;
mod too_many_lines;
mod non_scream_case;
mod non_pascal_case;

#[derive(Debug, Copy, Clone, enum_map::Enum)]
pub enum LintLevel {
    Allow,
    Warn,
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
}

#[derive(Debug, Copy, PartialEq, Eq, Clone, enum_map::Enum)]
pub enum Lint {
    MissingCaseMembers,
    MissingDefaultCase,
    UnrecognizedEnum,
    AndKeyword,
    OrKeyword,
    NonScreamCase,
    NonPascalCase,
    AnonymousConstructor,
    NoSpaceAtStartOfComment,
}
impl Lint {
    pub fn to_str(&self) -> &str {
        match self {
            Lint::MissingCaseMembers => "missing_case_members",
            Lint::MissingDefaultCase => "missing_default_case",
            Lint::UnrecognizedEnum => "unrecognized_enum",
            Lint::AndKeyword => "and_keyword",
            Lint::OrKeyword => "or_keyword",
            Lint::NonScreamCase => "non_scream_case",
            Lint::NonPascalCase => "non_pascal_case",
            Lint::AnonymousConstructor => "anonymous_constructor",
            Lint::NoSpaceAtStartOfComment => "no_space_at_start_of_comment",
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(tag: &str) -> Option<Self> {
        match tag {
            "missing_case_members" => Some(Lint::MissingCaseMembers),
            "missing_default_case" => Some(Lint::MissingDefaultCase),
            "unrecognized_enum" => Some(Lint::UnrecognizedEnum),
            "and_keyword" => Some(Lint::AndKeyword),
            "or_keyword" => Some(Lint::OrKeyword),
            "non_scream_case" => Some(Lint::NonScreamCase),
            "non_pascal_case" => Some(Lint::NonPascalCase),
            "anonymous_constructor" => Some(Lint::AnonymousConstructor),
            "no_space_at_start_of_comment" => Some(Lint::NoSpaceAtStartOfComment),
            _ => None,
        }
    }

    pub fn error_message(&self) -> String {
        match self {
            Lint::MissingCaseMembers => "Missing required members in switch statement.",
            Lint::MissingDefaultCase => "Missing default case in switch statement.",
            Lint::UnrecognizedEnum => "Unrecognized enum name to check against.",
            Lint::AndKeyword => "Use of illegal character: `and`.",
            Lint::OrKeyword => "Use of illegal character: `or`.",
            Lint::NonScreamCase => "Identifier should be SCREAM_CASE.",
            Lint::NonPascalCase => "Identifier should be PascalCase.",
            Lint::AnonymousConstructor => "Anonymous functions should not be constructors.",
            Lint::NoSpaceAtStartOfComment => "Comments should begin with a space.",
        }
        .into()
    }

    pub fn explanation_message(&self, level: LintLevel) -> String {
        match level {
            LintLevel::Allow => format!("`#[allow({})]` on by default", self.to_str()),
            LintLevel::Warn => format!("`#[warn({})]` on by default", self.to_str()),
            LintLevel::Deny => format!("`#[deny({})]` on by default", self.to_str()),
        }
    }

    pub fn hint_message(&self) -> String {
        let suggestions = match self {
            Lint::MissingCaseMembers => vec![
                "Add cases for the missing members",
                "Remove the imtentional crash from your default case",
            ],
            Lint::MissingDefaultCase => vec!["Add a default case to the switch statement"],
            Lint::UnrecognizedEnum => {
                vec!["Correct the name in the default case to the correct enum"]
            }
            Lint::AndKeyword | Lint::OrKeyword => vec!["Use the suggested symbol"],
            Lint::NonScreamCase | Lint::NonPascalCase => {
                vec!["Use the suggested casing"]
            }
            Lint::AnonymousConstructor => vec![
                "Change this into a named function",
                "Change this into a standard function that returns a struct literal",
            ],
            Lint::NoSpaceAtStartOfComment => {
                vec!["Add a space after the start of the comment"]
            }
        };

        let mut suggestions: Vec<String> = suggestions.into_iter().map(|s| s.to_string()).collect();
        suggestions.push(format!(
            "Ignore this by placing `// #[allow({})]` above this code",
            self.to_str()
        ));

        format!(
            "You can resolve this by doing one of the following:\n{}",
            suggestions
                .iter()
                .enumerate()
                .map(|(i, suggestion)| format!("  {}: {}\n", i + 1, suggestion))
                .collect::<String>(),
        )
    }
}

#[derive(Debug)]
pub struct LintTag(pub Lint, pub LintLevel);

/// An individual lint in duck.
///
/// Lints should be named after the *bad* action, not the good one. For example,
/// a lint that prevents switch statements from having no default case should be
/// called `MissingDefaultCase`, not, say, `DefaultCaseInSwitch`. This makes tagging
/// read more clearly (ie: `#[allow(missing_default_case)])`).
pub trait XLint {
    /// The string representation of this lint used for referencing it in code.
    /// For example, the lint `"MissingDefaultCase"` should return a string like
    /// `"missing_default_case"`.
    fn tag(&self) -> &str;

    /// The title of the lint as displayed when it fires into the output.
    fn display_name(&self) -> &str;

    /// A justification for this lint, expressing why it may be desirable to enable.
    fn explanation(&self) -> &str;

    /// A collection of suggestions on how to avoid this lint that will be displayed to the user
    /// when this lint fires.
    fn suggestions(&self) -> Vec<&str>;

    /// The [LintCategory] this lint belongs to.
    fn category(&self) -> LintCategory;
}

/// The category a lint falls into. This effects duck's default permission level for all lints.
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
impl LintCategory {
    /// Retrieves the default [LintLevel] all lints in a given category have
    pub fn default_level(&self) -> LintLevel {
        match self {
            LintCategory::Correctness => LintLevel::Deny,
            LintCategory::Suspicious => LintLevel::Warn,
            LintCategory::Style => LintLevel::Warn,
            LintCategory::Pedantic => LintLevel::Allow,
        }
    }
}
