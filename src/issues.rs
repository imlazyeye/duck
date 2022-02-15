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
            _ => None,
        }
    }

    pub fn error_message(&self) -> String {
        match self {
            ClippieIssue::MissingCaseMembers => "Missing required members in switch statement.",
            ClippieIssue::MissingDefaultCase => "Missing default case in switch statement.",
            ClippieIssue::UnrecognizedEnum => "Unrecognized enum name to check against.",
            ClippieIssue::AndKeyword => "Use of illegal character: `and`",
            ClippieIssue::OrKeyword => "Use of illegal character: `or`",
            ClippieIssue::NonScreamCase => "Identifier should be SCREAM_CASE",
            ClippieIssue::NonPascalCase => "Identifier should be PascalCase",
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
        let final_suggestion = format!(
            "   3. Ignore this by placing `// #[allow({})]` above the switch statement",
            self.code_tag()
        );
        match self {
            #[allow(clippy::format_in_format_args)]
            ClippieIssue::MissingCaseMembers => format!(
                "{}\n{}\n{}\n{}",
                "You can resolve this by doing one of the following:",
                "   1. Add cases for the missing members",
                "   2. Remove the imtentional crash from your default case",
                final_suggestion
            ),
            #[allow(clippy::format_in_format_args)]
            ClippieIssue::MissingDefaultCase => format!(
                "{}\n{}\n{}",
                "You can resolve this by doing one of the following:",
                "   1. Add a default case to the switch statement",
                final_suggestion
            ),
            #[allow(clippy::format_in_format_args)]
            ClippieIssue::UnrecognizedEnum => format!(
                "{}\n{}\n{}",
                "You can resolve this by doing one of the following:",
                "   1. Correct the name in the default case to the correct enum",
                final_suggestion
            ),
            #[allow(clippy::format_in_format_args)]
            ClippieIssue::AndKeyword | ClippieIssue::OrKeyword => format!(
                "{}\n{}\n{}",
                "You can resolve this by doing one of the following:",
                "   1. Use the suggested symbol",
                final_suggestion
            ),
            #[allow(clippy::format_in_format_args)]
            ClippieIssue::NonScreamCase | ClippieIssue::NonPascalCase => format!(
                "{}\n{}\n{}",
                "You can resolve this by doing one of the following:",
                "   1. Use the suggested casing",
                final_suggestion
            ),
        }
    }
}
