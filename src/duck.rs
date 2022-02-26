use crate::{config::Config, parsing::ParseError, DuckTask, LintLevel, LintReport};
use enum_map::EnumMap;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

/// ## Duck
/// The primary point of control for all of duck. For general usage, this is all you need!
///
/// ### Basic usage (tokio)
/// To generate a [RunResult] from a GameMaker Studio 2 project directory, you can use [Duck::run].
/// ```rs
/// let duck = Duck::new();
/// let my_project_path = "~/Users/me/GameMaker Studio 2/My Project";
/// let run_result = duck.run(my_project_path.into()).await;
/// ```
///
/// ### Basic usage (blocking)
/// The same result can be achieved without being forced to use async code by usinng [Duck::run_blocking].
/// ```rs
/// let duck = Duck::new();
/// let my_project_path = "~/Users/me/GameMaker Studio 2/My Project";
/// let run_result = duck.run_blocking(my_project_path.into());
/// ```
///
/// ### Manual operations
/// To create an [Ast] out of a string of Gml, you can use the [DuckOperation]s directly.
/// ```rs
/// let gml = show_debug_message("Hello world!");
/// let path = "../hello_world.gml";
/// let ast = match DuckOperation::parse_gml(gml, path.into()) {
///     Ok(ast) => ast,
///     Err(parse_error) => println!("Failed to parse gml: {parse_error:?}"),
/// };
/// ```
///
/// You can also manually run the [Lint]s on these [Ast]s.
/// ```rs
/// let lint_reports: Vec<LintReport> = vec![];
/// let environment = Environment::new();
/// DuckOperation::process_statement_early(
///     duck.config(),
///     &ast[0],
///     &mut environemnt,
///     &mut reports,
/// );
/// DuckOperation::process_statement_late(
///     duck.config(),
///     &ast[0],
///     &environemnt,
///     &mut reports,
/// );
/// ```
#[derive(Debug, Default)]
pub struct Duck {
    config: Config,
}
impl Duck {
    /// Creates a new Duck based on a DuckConfig.
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Goes through the entire process of finding, loading, parsing, and linting the GML in a given
    /// project directory. Returns every [LintReport] that was found, as well as [ParseError]s that were
    /// encountered along the way, and additionally any [std::io::Error]s that were found.
    ///
    /// If you are working in a blocking context, see [Duck::run_blocking].
    pub async fn run(&self, project_directory: &Path) -> RunResult {
        // Load everything in and await through the early pass
        let duck_arc = Arc::new(self.config.clone()); // todo this clone sucks
        let (path_receiver, _) = DuckTask::start_gml_discovery(project_directory);
        let (file_receiver, file_handle) = DuckTask::start_file_load(path_receiver);
        let (parse_receiver, parse_handle) = DuckTask::start_parse(file_receiver);
        let (early_receiever, _) = DuckTask::start_early_pass(duck_arc.clone(), parse_receiver);
        let (iterations, global_environment) =
            DuckTask::start_environment_assembly(early_receiever)
                .await
                .unwrap();

        // Now the late pass
        // Run the final pass...
        let late_pass_reports =
            DuckTask::start_late_pass(duck_arc.clone(), iterations, global_environment)
                .await
                .unwrap();

        // Extract any errors that were found...
        let io_errors = file_handle.await.unwrap();
        let parse_errors = parse_handle.await.unwrap();

        // Return the result!
        RunResult::new(
            &Config::default(),
            late_pass_reports,
            parse_errors,
            io_errors,
        )
    }

    /// The blocking counterpart to [Duck::run].
    pub fn run_blocking(&self, project_directory: &Path) -> RunResult {
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
pub struct RunResult {
    report_collection: EnumMap<LintLevel, Vec<(PathBuf, String, LintReport)>>,
    parse_errors: Vec<(PathBuf, String, ParseError)>,
    io_errors: Vec<std::io::Error>,
}
impl RunResult {
    fn new(
        config: &Config,
        lint_reports: Vec<(PathBuf, String, Vec<LintReport>)>,
        parse_errors: Vec<(PathBuf, String, ParseError)>,
        io_errors: Vec<std::io::Error>,
    ) -> Self {
        let mut report_collection: EnumMap<LintLevel, Vec<(PathBuf, String, LintReport)>> =
            EnumMap::default();
        for (path, gml, reports) in lint_reports {
            for report in reports {
                report_collection[*config.get_level_for_lint(report.tag(), report.category())]
                    .push((path.clone(), gml.clone(), report));
            }
        }
        Self {
            report_collection,
            parse_errors,
            io_errors,
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
}
