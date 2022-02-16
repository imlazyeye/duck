#[derive(Debug, Copy, Clone, enum_map::Enum)]
pub enum ClippieLevel {
    Allow,
    Warn,
    Deny,
}
impl ClippieLevel {
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
pub enum ClippieIssue {
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
impl ClippieIssue {
    pub fn to_str(&self) -> &str {
        match self {
            ClippieIssue::MissingCaseMembers => "missing_case_members",
            ClippieIssue::MissingDefaultCase => "missing_default_case",
            ClippieIssue::UnrecognizedEnum => "unrecognized_enum",
            ClippieIssue::AndKeyword => "and_keyword",
            ClippieIssue::OrKeyword => "or_keyword",
            ClippieIssue::NonScreamCase => "non_scream_case",
            ClippieIssue::NonPascalCase => "non_pascal_case",
            ClippieIssue::AnonymousConstructor => "anonymous_constructor",
            ClippieIssue::NoSpaceAtStartOfComment => "no_space_at_start_of_comment",
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(tag: &str) -> Option<Self> {
        match tag {
            "missing_case_members" => Some(ClippieIssue::MissingCaseMembers),
            "missing_default_case" => Some(ClippieIssue::MissingDefaultCase),
            "unrecognized_enum" => Some(ClippieIssue::UnrecognizedEnum),
            "and_keyword" => Some(ClippieIssue::AndKeyword),
            "or_keyword" => Some(ClippieIssue::OrKeyword),
            "non_scream_case" => Some(ClippieIssue::NonScreamCase),
            "non_pascal_case" => Some(ClippieIssue::NonPascalCase),
            "anonymous_constructor" => Some(ClippieIssue::AnonymousConstructor),
            "no_space_at_start_of_comment" => Some(ClippieIssue::NoSpaceAtStartOfComment),
            _ => None,
        }
    }

    pub fn error_message(&self) -> String {
        match self {
            ClippieIssue::MissingCaseMembers => "Missing required members in switch statement.",
            ClippieIssue::MissingDefaultCase => "Missing default case in switch statement.",
            ClippieIssue::UnrecognizedEnum => "Unrecognized enum name to check against.",
            ClippieIssue::AndKeyword => "Use of illegal character: `and`.",
            ClippieIssue::OrKeyword => "Use of illegal character: `or`.",
            ClippieIssue::NonScreamCase => "Identifier should be SCREAM_CASE.",
            ClippieIssue::NonPascalCase => "Identifier should be PascalCase.",
            ClippieIssue::AnonymousConstructor => "Anonymous functions should not be constructors.",
            ClippieIssue::NoSpaceAtStartOfComment => "Comments should begin with a space.",
        }
        .into()
    }

    pub fn explanation_message(&self, level: ClippieLevel) -> String {
        match level {
            ClippieLevel::Allow => format!("`#[allow({})]` on by default", self.to_str()),
            ClippieLevel::Warn => format!("`#[warn({})]` on by default", self.to_str()),
            ClippieLevel::Deny => format!("`#[deny({})]` on by default", self.to_str()),
        }
    }

    pub fn hint_message(&self) -> String {
        let suggestions = match self {
            ClippieIssue::MissingCaseMembers => vec![
                "Add cases for the missing members",
                "Remove the imtentional crash from your default case",
            ],
            ClippieIssue::MissingDefaultCase => vec!["Add a default case to the switch statement"],
            ClippieIssue::UnrecognizedEnum => {
                vec!["Correct the name in the default case to the correct enum"]
            }
            ClippieIssue::AndKeyword | ClippieIssue::OrKeyword => vec!["Use the suggested symbol"],
            ClippieIssue::NonScreamCase | ClippieIssue::NonPascalCase => {
                vec!["Use the suggested casing"]
            }
            ClippieIssue::AnonymousConstructor => vec![
                "Change this into a named function",
                "Change this into a standard function that returns a struct literal",
            ],
            ClippieIssue::NoSpaceAtStartOfComment => {
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
pub struct ClippieIssueTag(pub ClippieIssue, pub ClippieLevel);
