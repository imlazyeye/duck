use crate::{
    analyze::{GlobalScope, GlobalScopeBuilder},
    core::DuckOperation,
    lint::LintReport,
    parse::{Ast, ParseError, ParseErrorReport, StatementBox},
    Config,
};
use async_walkdir::{DirEntry, Filtering, WalkDir};
use codespan_reporting::files::SimpleFiles;
use futures::StreamExt;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::{
    sync::mpsc::{channel, Receiver},
    task::JoinHandle,
};

/// A collection of functions used to build the Tokio tasks that drive duck.
/// Each uses data returned from the last one, allowing you to customize exactly
/// which parts of duck's overall process you wish to run.
///
/// If you are just interested in calling duck's *entire* process, see
/// [Duck::run].
pub struct DuckTask;
impl DuckTask {
    /// Creates a Tokio task which will walk through the provided directory in
    /// search of gml files. Passes each path it finds into the returned
    /// Receiver. Closes when all files have been sent.
    ///
    /// ### Panics
    /// Panics if the receiver for the sender closes. This should not be possible!
    pub fn start_gml_discovery(directory: &Path) -> (Receiver<PathBuf>, JoinHandle<Vec<std::io::Error>>) {
        /// Filters DirEntry's for gml files.
        async fn filter(entry: DirEntry) -> Filtering {
            if let Some(true) = entry.path().file_name().map(|f| !f.to_string_lossy().ends_with(".gml")) {
                Filtering::Ignore
            } else {
                Filtering::Continue
            }
        }

        let mut io_errors = vec![];
        let mut walker = WalkDir::new(directory.join("objects"))
            .filter(filter)
            .chain(WalkDir::new(directory.join("scripts")).filter(filter))
            .chain(WalkDir::new(directory.join("rooms")).filter(filter));
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

    /// Creates a Tokio task which will await paths through `path_receiever` and
    /// subsequently load their data, pumping it to the returned Receiver.
    /// Closes when the `path_receiver` channel closes. Additionally returns the total number of
    /// lines that were found.
    ///
    /// ### Panics
    /// Panics if the receiver for the sender closes. This should not be possible!
    #[allow(clippy::type_complexity)] // yeah yeah i'll make it better eventually
    pub fn start_file_load(
        mut path_receiver: Receiver<PathBuf>,
    ) -> (
        Receiver<(FileId, &'static str)>,
        JoinHandle<(usize, SimpleFiles<String, &'static str>, Vec<std::io::Error>)>,
    ) {
        let (file_sender, file_receiver) = channel::<(FileId, &'static str)>(1000);
        let handle = tokio::task::spawn(async move {
            let mut files = SimpleFiles::new();
            let mut io_errors = vec![];
            let mut lines = 0;
            while let Some(path) = path_receiver.recv().await {
                match tokio::fs::read_to_string(&path).await {
                    Ok(gml) => {
                        let gml: &'static str = Box::leak(Box::new(gml));
                        lines += gml.lines().count();
                        let file_id = files.add(path.to_str().unwrap().to_string(), gml);
                        file_sender.send((file_id, gml)).await.unwrap();
                    }
                    Err(io_error) => io_errors.push(io_error),
                };
            }
            (lines, files, io_errors)
        });
        (file_receiver, handle)
    }

    /// Creates a Tokio task which will await gml files through `file_receiever`
    /// and subsequently parse them into an [Ast], pumping them into the
    /// returned Receiver. Closes when the `file_receiever` channel closes.
    ///
    /// ### Panics
    /// Panics if the receiver for the sender closes. This should not be possible!
    pub fn start_parse(
        mut file_receiver: Receiver<(FileId, &'static str)>,
    ) -> (Receiver<ParseReport>, JoinHandle<Vec<ParseErrorReport>>) {
        let (ast_sender, ast_receiver) = channel::<(FileId, Ast)>(1000);
        let handle = tokio::task::spawn(async move {
            let mut parse_errors = vec![];
            while let Some((file_id, gml)) = file_receiver.recv().await {
                match DuckOperation::parse_gml(gml, &file_id) {
                    Ok(ast) => ast_sender.send((file_id, ast)).await.unwrap(),
                    Err(parse_error) => parse_errors.push(parse_error),
                }
            }
            parse_errors
        });
        (ast_receiver, handle)
    }

    /// Creates a Tokio task that will await [Ast]s through `ast_receiver` and
    /// run the early pass lints on them, pumping the results through the
    /// returned Receiver. Closes when the `ast_receiever` channel closes.
    ///
    /// ### Panics
    /// Panics if the receiver for the sender closes. This should not be possible!
    pub fn start_early_pass(
        config: Arc<Config>,
        mut ast_receiever: Receiver<(FileId, Ast)>,
    ) -> (Receiver<EarlyPassEntry>, JoinHandle<()>) {
        let (early_pass_sender, early_pass_receiver) = channel::<EarlyPassEntry>(1000);
        let handle = tokio::task::spawn(async move {
            while let Some((path, ast)) = ast_receiever.recv().await {
                for statement in ast {
                    let config = config.clone();
                    let file_id = path.clone();
                    let sender = early_pass_sender.clone();
                    tokio::task::spawn(async move {
                        let mut reports = vec![];
                        let mut scope_builder = GlobalScopeBuilder::new();
                        DuckOperation::process_statement_early(
                            config.as_ref(),
                            &statement,
                            &mut scope_builder,
                            &mut reports,
                        );
                        sender.send((file_id, statement, scope_builder, reports)).await.unwrap();
                    });
                }
            }
        });
        (early_pass_receiver, handle)
    }

    /// Creates a Tokio task that will await [StatementIteration]s through
    /// `early_pass_receiever` and construct their [Environment]s into one
    /// singular [Environemnt], returning it once complete, as well as a Vec
    /// of all statements still needing a second pass.
    pub fn start_environment_assembly(
        mut early_pass_receiever: Receiver<EarlyPassEntry>,
    ) -> JoinHandle<(Vec<LatePassEntry>, GlobalScope)> {
        tokio::task::spawn(async move {
            let mut pass_two_queue = vec![];
            let mut global_scope = GlobalScope::new();
            while let Some((file_id, statement, scope_builder, reports)) = early_pass_receiever.recv().await {
                global_scope.drain(scope_builder);
                pass_two_queue.push((file_id, statement, reports));
            }
            (pass_two_queue, global_scope)
        })
    }

    /// Creates Tokio tasks for all of the provided `StatementIteration`s,
    /// running the late lint pass on them. Returns a handle to another
    /// Tokio task which will collect their finalized [LatePassReport]s.
    ///
    /// ### Panics
    /// Panics if the receiver for the sender closes. This should not be possible!
    pub fn start_late_pass(
        config: Arc<Config>,
        iterations: Vec<LatePassEntry>,
        global_environemnt: GlobalScope,
    ) -> JoinHandle<LintReportCollection> {
        let (lint_report_sender, mut lint_report_reciever) = channel::<(FileId, Vec<LintReport>)>(1000);
        let global_environment = Arc::new(global_environemnt);
        for (file_id, statement, mut lint_reports) in iterations {
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
                sender.send((file_id, lint_reports)).await.unwrap();
            });
        }
        tokio::task::spawn(async move {
            let mut lint_reports: LintReportCollection = vec![];
            while let Some((file_id, reports)) = lint_report_reciever.recv().await {
                lint_reports.push((file_id, reports));
            }
            lint_reports
        })
    }
}

/// A wrapper around `usize`, which `codespan-reporting` uses as an id for files. Just used to help
/// with readability. The returned data from successful parses.
pub type FileId = usize;
/// The returend data from a parse.
pub type ParseReport = (FileId, Ast);
/// The returned data from unsuccessful parses.
pub type ParseErrorCollection = Vec<ParseError>;
/// An individual statement's data to be sent to the early pass.
pub type EarlyPassEntry = (FileId, StatementBox, GlobalScopeBuilder, Vec<LintReport>);
/// An individual statement's data to be sent to the late pass.
pub type LatePassEntry = (FileId, StatementBox, Vec<LintReport>);
/// A collection of [LintReports] and corresponding data.. Each entry in the
/// collection correlates to a single statement, containing the path to the file
/// it is from, the source of that file, and a collection of each [LintReport]
/// it triggered.
pub type LintReportCollection = Vec<(FileId, Vec<LintReport>)>; // TODO: make FileId live in the Boxes
