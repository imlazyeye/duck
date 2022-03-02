use crate::{
    lint::{LintLevel, LintReport},
    parsing::ParseError,
    Config, DuckTask,
};
use enum_map::EnumMap;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

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
        // Load everything in and await through the early pass
        let config_arc = Arc::new(self.config.clone()); // TODO: this clone sucks
        let (path_receiver, _) = DuckTask::start_gml_discovery(project_directory);
        let (file_receiver, file_handle) = DuckTask::start_file_load(path_receiver);
        let (parse_receiver, parse_handle) = DuckTask::start_parse(file_receiver);
        let (early_receiever, _) = DuckTask::start_early_pass(config_arc.clone(), parse_receiver);
        let (iterations, global_environment) = DuckTask::start_environment_assembly(early_receiever).await?;

        // Now the late pass
        // Run the final pass...
        let late_pass_reports = DuckTask::start_late_pass(config_arc.clone(), iterations, global_environment).await?;

        // Extract any errors that were found...
        let (line_count, io_errors) = file_handle.await?;
        let parse_errors = parse_handle.await?;

        // Return the result!
        Ok(RunSummary::new(
            self.config(),
            late_pass_reports,
            parse_errors,
            io_errors,
            line_count,
        ))
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
    report_collection: EnumMap<LintLevel, Vec<(PathBuf, String, LintReport)>>,
    parse_errors: Vec<(PathBuf, String, ParseError)>,
    io_errors: Vec<std::io::Error>,
    lines_parsed: usize,
}
impl RunSummary {
    fn new(
        config: &Config,
        lint_reports: Vec<(PathBuf, String, Vec<LintReport>)>,
        parse_errors: Vec<(PathBuf, String, ParseError)>,
        io_errors: Vec<std::io::Error>,
        lines_parsed: usize,
    ) -> Self {
        let mut report_collection: EnumMap<LintLevel, Vec<(PathBuf, String, LintReport)>> = EnumMap::default();
        for (path, gml, reports) in lint_reports {
            for report in reports {
                report_collection[*config.get_lint_level_setting(report.tag(), report.default_level())].push((
                    path.clone(),
                    gml.clone(),
                    report,
                ));
            }
        }
        Self {
            report_collection,
            parse_errors,
            io_errors,
            lines_parsed,
        }
    }

    /// Returns the number of warnings present in this [RunResult].
    pub fn warning_count(&self) -> usize {
        self.report_collection[LintLevel::Warn].len()
    }

    /// Returns the number of denials present in this [RunResult].
    pub fn denial_count(&self) -> usize {
        self.report_collection[LintLevel::Deny].len()
    }

    /// Returns an iterator over all of the lint reports.
    pub fn iter_lint_reports(&self) -> impl Iterator<Item = &(PathBuf, String, LintReport)> {
        self.report_collection[LintLevel::Warn]
            .iter()
            .chain(self.report_collection[LintLevel::Deny].iter())
    }

    /// Get a reference to the run result's parse errors.
    pub fn parse_errors(&self) -> &[(PathBuf, String, ParseError)] {
        self.parse_errors.as_ref()
    }

    /// Get a reference to the run result's io errors.
    pub fn io_errors(&self) -> &[std::io::Error] {
        self.io_errors.as_ref()
    }

    /// Get the run result's lines parsed.
    pub fn lines_parsed(&self) -> usize {
        self.lines_parsed
    }
}