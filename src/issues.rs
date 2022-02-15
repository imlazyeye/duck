#[derive(Debug, Copy, Clone, enum_map::Enum)]
pub enum ClippieLevel {
    Allow,
    Warn,
    Deny,
}

#[derive(Debug, Copy, Clone, enum_map::Enum)]
pub enum ClippieIssue {
    MissingCaseMembers,
    MissingDefaultCase,
    UnrecognizedEnum,
    AndKeyword,
    OrKeyword,
    NonScreamCase,
    NonPascalCase,
    AnonymousConstructor,
}
impl ClippieIssue {
    pub fn code_tag(&self) -> &str {
        match self {
            ClippieIssue::MissingCaseMembers => "missing_case_members",
            ClippieIssue::MissingDefaultCase => "missing_default_case",
            ClippieIssue::UnrecognizedEnum => "unrecognized_enum",
            ClippieIssue::AndKeyword => "and_keyword",
            ClippieIssue::OrKeyword => "or_keyword",
            ClippieIssue::NonScreamCase => "non_scream_case",
            ClippieIssue::NonPascalCase => "non_pascal_case",
            ClippieIssue::AnonymousConstructor => "anonymous_constructor",
        }
    }

    pub fn from_code_tag(tag: &str) -> Option<Self> {
        match tag {
            "missing_case_members" => Some(ClippieIssue::MissingCaseMembers),
            "missing_default_case" => Some(ClippieIssue::MissingDefaultCase),
            "unrecognized_enum" => Some(ClippieIssue::UnrecognizedEnum),
            "and_keyword" => Some(ClippieIssue::AndKeyword),
            "or_keyword" => Some(ClippieIssue::OrKeyword),
            "non_scream_case" => Some(ClippieIssue::NonScreamCase),
            "non_pascal_case" => Some(ClippieIssue::NonPascalCase),
            "anonymous_constructor" => Some(ClippieIssue::AnonymousConstructor),
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
            &ClippieIssue::AnonymousConstructor => {
                "Anonymous functions should not be constructors."
            }
        }
        .into()
    }

    pub fn explanation_message(&self, level: ClippieLevel) -> String {
        match level {
            ClippieLevel::Allow => format!("`#[allow({})]` on by default", self.code_tag()),
            ClippieLevel::Warn => format!("`#[warn({})]` on by default", self.code_tag()),
            ClippieLevel::Deny => format!("`#[deny({})]` on by default", self.code_tag()),
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
        };

        let mut suggestions: Vec<String> = suggestions.into_iter().map(|s| s.to_string()).collect();
        suggestions.push(format!(
            "Ignore this by placing `// #[allow({})]` above this code",
            self.code_tag()
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
