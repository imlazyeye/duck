use crate::config::Config;
use crate::duck_operation::DuckOperation;
use crate::gml::Environment;
use crate::parsing::parser::Ast;
use crate::parsing::statement::StatementBox;
use crate::parsing::ParseError;
use crate::LintReport;
use async_walkdir::{DirEntry, Filtering, WalkDir};
use futures::StreamExt;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::mpsc::{channel, Receiver};
use tokio::task::JoinHandle;

/// A collection of functions used to build the Tokio tasks that drive duck. Each uses data
/// returned from the last one, allowing you to customize exactly which parts of duck's overall
/// process you wish to run.
///
/// If you are just interested in calling duck's *entire* process, see [Duck::run].
pub struct DuckTask;
impl DuckTask {
    /// Creates a Tokio task which will walk through the provided directory in search of gml files.
    /// Passes each path it finds into the returned Receiver. Closes when all files have been sent.
    pub fn start_gml_discovery(directory: &Path) -> (Receiver<PathBuf>, JoinHandle<Vec<std::io::Error>>) {
        /// Filters DirEntry's for gml files.
        async fn filter(entry: DirEntry) -> Filtering {
            if let Some(true) = entry
                .path()
                .file_name()
                .map(|f| !f.to_string_lossy().ends_with(".gml"))
            {
                Filtering::Ignore
            } else {
                Filtering::Continue
            }
        }

        let mut io_errors = vec![];
        let mut walker = WalkDir::new(directory.join("objects"))
            .filter(filter)
            .chain(WalkDir::new(directory.join("scripts")).filter(filter));
        let (path_sender, path_receiver) = channel::<PathBuf>(1000);
        let handle = tokio::task::spawn(async move {
            loop {
                match walker.next().await {
                    Some(Ok(entry)) => path_sender.send(entry.path()).await.unwrap(),
                    Some(Err(e)) => io_errors.push(e),
                    None => break,
                }
            }
            io_errors
        });
        (path_receiver, handle)
    }

    /// Creates a Tokio task which will await paths through `path_receiever` and subsequently load
    /// their data, pumping it to the returned Receiver. Closes when the `path_receiver` channel closes.
    pub fn start_file_load(
        mut path_receiver: Receiver<PathBuf>,
    ) -> (Receiver<(PathBuf, String)>, JoinHandle<Vec<std::io::Error>>) {
        let (file_sender, file_receiver) = channel::<(PathBuf, String)>(1000);
        let handle = tokio::task::spawn(async move {
            let mut io_errors = vec![];
            while let Some(path) = path_receiver.recv().await {
                match tokio::fs::read_to_string(&path).await {
                    Ok(gml) => {
                        file_sender.send((path, gml)).await.unwrap();
                    }
                    Err(io_error) => io_errors.push(io_error),
                };
            }
            io_errors
        });
        (file_receiver, handle)
    }

    /// Creates a Tokio task which will await gml files through `file_receiever` and subsequently parse
    /// them into an [Ast], pumping them into the returned Receiver. Closes when the `file_receiever`
    /// channel closes.
    pub fn start_parse(
        mut file_receiver: Receiver<(PathBuf, String)>,
    ) -> (Receiver<ParseReport>, JoinHandle<ParseErrorCollection>) {
        let (ast_sender, ast_receiver) = channel::<(PathBuf, String, Ast)>(1000);
        let handle = tokio::task::spawn(async move {
            let mut parse_errors = vec![];
            while let Some((path, gml)) = file_receiver.recv().await {
                match DuckOperation::parse_gml(&gml, &path) {
                    Ok(ast) => ast_sender.send((path, gml, ast)).await.unwrap(),
                    Err(parse_error) => parse_errors.push((path, gml, parse_error)),
                }
            }
            parse_errors
        });
        (ast_receiver, handle)
    }

    /// Creates a Tokio task that will await [Ast]s through `ast_receiver` and run the early pass lints
    /// on them, pumping the results through the returned Receiver. Closes when the `ast_receiever` channel
    /// closes.
    pub fn start_early_pass(
        config: Arc<Config>,
        mut ast_receiever: Receiver<(PathBuf, String, Ast)>,
    ) -> (Receiver<StatementIteration>, JoinHandle<()>) {
        let (early_pass_sender, early_pass_receiver) = channel::<StatementIteration>(1000);
        let handle = tokio::task::spawn(async move {
            while let Some((path, gml, ast)) = ast_receiever.recv().await {
                for statement in ast {
                    let config = config.clone();
                    let gml = gml.clone();
                    let path = path.clone();
                    let sender = early_pass_sender.clone();
                    tokio::task::spawn(async move {
                        let mut reports = vec![];
                        let mut environment = Environment::new();
                        DuckOperation::process_statement_early(
                            config.as_ref(),
                            &statement,
                            &mut environment,
                            &mut reports,
                        );
                        sender
                            .send((path, gml, statement, environment, reports))
                            .await
                            .unwrap();
                    });
                }
            }
        });
        (early_pass_receiver, handle)
    }

    /// Creates a Tokio task that will await [StatementIteration]s through `early_pass_receiever` and construct
    /// their [Environment]s into one singular [Environemnt], returning it once complete, as well as a Vec
    /// of all statements still needing a second pass.
    pub fn start_environment_assembly(
        mut early_pass_receiever: Receiver<StatementIteration>,
    ) -> JoinHandle<(Vec<StatementIteration>, Environment)> {
        tokio::task::spawn(async move {
            let mut pass_two_queue = vec![];
            let mut global_environment = Environment::new();
            while let Some((path, gml, statement, environment, reports)) =
                early_pass_receiever.recv().await
            {
                global_environment.copy_from(&environment);
                pass_two_queue.push((path, gml, statement, environment, reports));
            }
            (pass_two_queue, global_environment)
        })
    }

    /// Creates Tokio tasks for all of the provided `StatementIteration`s, running the late lint pass on them.
    /// Returns a handle to another Tokio task which will collect their finalized [LatePassReport]s.
    pub fn start_late_pass(
        config: Arc<Config>,
        iterations: Vec<StatementIteration>,
        global_environemnt: Environment,
    ) -> JoinHandle<LintReportCollection> {
        let (lint_report_sender, mut lint_report_reciever) =
            channel::<(PathBuf, String, Vec<LintReport>)>(1000);
        let global_environment = Arc::new(global_environemnt);
        for (path, gml, statement, _environment, mut lint_reports) in iterations {
            let sender = lint_report_sender.clone();
            let global_environment = global_environment.clone();
            let config = config.clone();
            tokio::task::spawn(async move {
                DuckOperation::process_statement_late(
                    config.as_ref(),
                    &statement,
                    global_environment.as_ref(),
                    &mut lint_reports,
                );
                sender.send((path, gml, lint_reports)).await.unwrap();
            });
        }
        tokio::task::spawn(async move {
            let mut lint_reports: LintReportCollection = vec![];
            while let Some(values) = lint_report_reciever.recv().await {
                lint_reports.push(values);
            }
            lint_reports
        })
    }
}

pub type ParseReport = (PathBuf, String, Ast);
pub type ParseErrorCollection = Vec<(PathBuf, String, ParseError)>;
pub type StatementIteration = (PathBuf, String, StatementBox, Environment, Vec<LintReport>);
pub type LintReportCollection = Vec<(PathBuf, String, Vec<LintReport>)>;
