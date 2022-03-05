use colored::Colorize;

/// Utility for creating pretty previews out of files and spans.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FilePreviewUtil<'a> {
    /// The name of the file.
    pub file_name: &'a str,
    /// The line this preview pertains to.
    pub line: usize,
    /// The column this preview pertains to.
    pub column: usize,
    /// The snippet of the code in question.
    pub snippet: &'a str,
}
impl<'a> FilePreviewUtil<'a> {
    /// Creates a new utility.
    pub fn new(file_contents: &'a str, file_name: &'a str, cursor: usize) -> Self {
        let mut line = 1;
        let mut column = 0;
        file_contents[..cursor].chars().for_each(|c| {
            if c == '\n' {
                line += 1;
                column = 0;
            } else {
                column += 1;
            }
        });
        let line_and_after = &file_contents[cursor - column..];
        let last_index = line_and_after
            .match_indices('\n')
            .next()
            .map_or(line_and_after.len() - 1, |(i, _)| i - 1);
        let snippet = &line_and_after[..last_index];
        Self {
            file_name,
            line,
            column,
            snippet,
        }
    }

    /// Returns the name of the file, formatted with the line and column included.
    pub fn file_string(&self) -> String {
        format!("{}:{}:{}", self.file_name, self.line, self.column)
    }

    /// Creates a user-friendly display of the code this preview covers.
    pub fn snippet_message(&self) -> String {
        format!(
            "{}\n{}{}\n{}",
            " | ".bright_blue().bold(),
            " | ".bright_blue().bold(),
            self.snippet,
            " | ".bright_blue().bold()
        )
    }

    /// Creates a user-friendly display of the path, line, and column.
    pub fn path_message(&self) -> String {
        format!(" {} {}", "-->".bold().bright_blue(), self.file_string())
    }
}