use colored::Colorize;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FilePreviewUtil<'a> {
    pub file_name: &'a str,
    pub line: usize,
    pub column: usize,
    pub snippet: &'a str,
}
impl<'a> FilePreviewUtil<'a> {
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

    pub fn file_string(&self) -> String {
        format!("{}:{}:{}", self.file_name, self.line, self.column)
    }

    pub fn snippet_message(&self) -> String {
        format!(
            "{}\n{}{}\n{}",
            " | ".bright_blue().bold(),
            " | ".bright_blue().bold(),
            self.snippet,
            " | ".bright_blue().bold()
        )
    }

    pub fn path_message(&self) -> String {
        format!(" {} {}", "-->".bold().bright_blue(), self.file_string())
    }
}

#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct Span(pub usize, pub usize);
