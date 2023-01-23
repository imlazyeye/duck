use crate::{lint::LintLevel, Config};
use codespan_reporting::{
    diagnostic::Diagnostic,
    files::{Error, Files, SimpleFile},
};
use enum_map::EnumMap;
use std::{ops::Range, path::Path, sync::Arc};

use super::driver;

/// ## Duck
/// The primary point of control for all of duck. For general usage, this is all
/// you need!
///
/// ### Basic usage (tokio)
/// To generate a [RunResult] from a GameMaker Studio 2 project directory, you
/// can use [Duck::run].
/// ```
/// use duck::*;
/// use std::path::Path;
///
/// # async {
/// let duck = Duck::default();
/// let my_project_path = Path::new("~/Users/me/GameMaker Studio 2/My Project");
/// let run_result = duck.run(my_project_path).await;
/// # };
/// ```
///
/// ### Basic usage (blocking)
/// The same result can be achieved without being forced to use async code by
/// using [Duck::run_blocking].
/// ```
/// # use duck::*;
/// # use std::path::Path;
/// # let duck = Duck::default();
/// # let my_project_path = Path::new("~/Users/me/GameMaker Studio 2/My
/// Project"); let run_result = duck.run_blocking(my_project_path);
/// ```
/// For finer control over duck's operations, see [DuckOperation].
#[derive(Debug, Default)]
pub struct Duck {
    config: Config,
}
impl Duck {
    /// Creates a new Duck based on a DuckConfig.
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Goes through the entire process of finding, loading, parsing, and
    /// linting the GML in a given project directory. Returns every
    /// [LintReport] that was found, as well as [ParseError]s that were
    /// encountered along the way, and additionally any [std::io::Error]s that
    /// were found.
    ///
    /// If you are working in a blocking context, see [Duck::run_blocking].
    ///
    /// ### Errors
    /// Returns an error if we fail to join any of the tokio tasks.
    pub async fn run(&self, project_directory: &Path) -> Result<RunSummary, tokio::task::JoinError> {
        // Load everything in and await through the early pass...
        let config_arc = Arc::new(self.config.clone()); // TODO: this clone sucks
        let (path_receiver, walker_handle) = driver::start_gml_discovery(project_directory);
        let (file_receiver, file_handle) = driver::start_file_load(path_receiver);
        let (parse_receiver, parse_handle) = driver::start_parse(file_receiver);
        let (stmt_receiever, report_sender, report_receiver, _) =
            driver::start_early_pass(config_arc.clone(), parse_receiver);
        let mut diagnostics =
            driver::start_late_pass(config_arc.clone(), stmt_receiever, report_sender, report_receiver).await?;

        // Extract any errors that were found...
        let (line_count, library, mut io_errors) = file_handle.await?;
        io_errors.append(&mut walker_handle.await?);
        diagnostics.append(&mut parse_handle.await?);

        // Return the result!
        Ok(RunSummary::new(library, diagnostics, io_errors, line_count))
    }

    /// The blocking counterpart to [Duck::run].
    ///
    /// ### Errors
    /// Returns an error if we fail to join any of the tokio tasks.
    ///
    /// ### Panics
    /// Panics if we fail to spawn a tokio runtime.
    pub fn run_blocking(&self, project_directory: &Path) -> Result<RunSummary, tokio::task::JoinError> {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(self.run(project_directory))
    }

    /// Get a reference to the duck's config.
    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// The data returned by calling [Duck::run].
pub struct RunSummary {
    library: GmlLibrary,
    diagonstic_counts: EnumMap<LintLevel, usize>,
    diagnostics: Vec<Diagnostic<FileId>>,
    io_errors: Vec<std::io::Error>,
    lines_parsed: usize,
}
impl RunSummary {
    fn new(
        library: GmlLibrary,
        diagnostics: Vec<Diagnostic<FileId>>,
        io_errors: Vec<std::io::Error>,
        lines_parsed: usize,
    ) -> Self {
        let mut diagonstic_counts: EnumMap<LintLevel, usize> = EnumMap::default();
        for report in diagnostics.iter() {
            diagonstic_counts[report.severity.into()] += 1;
        }
        Self {
            library,
            diagonstic_counts,
            diagnostics,
            io_errors,
            lines_parsed,
        }
    }

    /// Returns the number of warnings present in this [RunResult].
    pub fn warning_count(&self) -> usize {
        self.diagonstic_counts[LintLevel::Warn]
    }

    /// Returns the number of denials present in this [RunResult].
    pub fn denial_count(&self) -> usize {
        self.diagonstic_counts[LintLevel::Deny]
    }

    /// Get a reference to the run summary's diagnostics.
    pub fn diagnostics(&self) -> &[Diagnostic<usize>] {
        self.diagnostics.as_ref()
    }

    /// Get a reference to the run result's io errors.
    pub fn io_errors(&self) -> &[std::io::Error] {
        self.io_errors.as_ref()
    }

    /// Get the run result's lines parsed.
    pub fn lines_parsed(&self) -> usize {
        self.lines_parsed
    }

    /// Get a reference to the run summary's files.
    pub fn files(&self) -> &GmlLibrary {
        &self.library
    }
}

/// Holds onto the references of loaded gml to be looked up for diagnostics, and to later clean up
/// all memory.
#[derive(Debug, Default)]
pub struct GmlLibrary {
    files: Vec<SimpleFile<String, &'static str>>,
}
impl GmlLibrary {
    /// Create a new files database.
    pub fn new() -> GmlLibrary {
        GmlLibrary { files: Vec::new() }
    }

    /// Add a file to the database, returning the handle that can be used to
    /// refer to it again.
    pub fn add(&mut self, name: String, source: &'static str) -> usize {
        let file_id = self.files.len();
        self.files.push(SimpleFile::new(name, source));
        file_id
    }

    /// Get the file corresponding to the given id.
    ///
    /// ### Errors
    /// Returns an error if the file is not found.
    pub fn get(&self, file_id: usize) -> Result<&SimpleFile<String, &'static str>, Error> {
        self.files.get(file_id).ok_or(Error::FileMissing)
    }
}
impl<'a> Files<'a> for GmlLibrary {
    type FileId = FileId;
    type Name = String;
    type Source = &'a str;

    fn name(&self, file_id: usize) -> Result<String, Error> {
        Ok(self.get(file_id)?.name().clone())
    }

    fn source(&self, file_id: usize) -> Result<&str, Error> {
        Ok(self.get(file_id)?.source())
    }

    fn line_index(&self, file_id: usize, byte_index: usize) -> Result<usize, Error> {
        self.get(file_id)?.line_index((), byte_index)
    }

    fn line_range(&self, file_id: usize, line_index: usize) -> Result<Range<usize>, Error> {
        self.get(file_id)?.line_range((), line_index)
    }
}

/// A wrapper around `usize`, which `codespan-reporting` uses as an id for files. Just used to help
/// with readability. The returned data from successful parses.
pub type FileId = usize;
