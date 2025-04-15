use crate::{
    Config, FileId, GmlLibrary,
    lint::{collection::*, *},
    parse::{Ast, Expr, ParseVisitor, Parser, Stmt},
};
use async_walkdir::{DirEntry, Filtering, WalkDir};
use codespan_reporting::diagnostic::Diagnostic;
use futures::StreamExt;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::{
    sync::mpsc::{Receiver, Sender, channel},
    task::JoinHandle,
};

/// Parses the given String of GML, returning either a successfully
/// constructed [Ast] or a [ParseError].
///
/// ### Errors
///
/// Returns a [ParseError] if the parsing was unsuccessful.
pub fn parse_gml(source_code: &'static str, file_id: &FileId) -> Result<Ast, Diagnostic<FileId>> {
    Parser::new(source_code, *file_id).into_ast()
}

/// Runs a [Stmt] through the early pass, running any lint that
/// implements [EarlyStmtPass], as well as collecting information
/// into the provided [GlobalScopeBuilder].
///
/// NOTE: This function is largely auto-generated! See `CONTRIBUTING.md` for
/// more information.
pub fn process_stmt_early(stmt: &mut Stmt, reports: &mut Vec<Diagnostic<FileId>>, config: &Config) {
    // @early stmt calls. Do not remove this comment!
    run_early_lint_on_stmt::<CasingRules>(stmt, config, reports);
    run_early_lint_on_stmt::<CollapsableIf>(stmt, config, reports);
    run_early_lint_on_stmt::<ConditionWrapper>(stmt, config, reports);
    run_early_lint_on_stmt::<Deprecated>(stmt, config, reports);
    run_early_lint_on_stmt::<Exit>(stmt, config, reports);
    run_early_lint_on_stmt::<Global>(stmt, config, reports);
    run_early_lint_on_stmt::<InvalidAssignment>(stmt, config, reports);
    run_early_lint_on_stmt::<MissingDefaultCase>(stmt, config, reports);
    run_early_lint_on_stmt::<MultiVarDeclaration>(stmt, config, reports);
    run_early_lint_on_stmt::<SingleSwitchCase>(stmt, config, reports);
    run_early_lint_on_stmt::<SuspicousConstantUsage>(stmt, config, reports);
    run_early_lint_on_stmt::<SwitchWithoutCase>(stmt, config, reports);
    run_early_lint_on_stmt::<TryCatch>(stmt, config, reports);
    run_early_lint_on_stmt::<UnassignedConstructor>(stmt, config, reports);
    run_early_lint_on_stmt::<UnnecessaryGrouping>(stmt, config, reports);
    run_early_lint_on_stmt::<UselessFunction>(stmt, config, reports);
    run_early_lint_on_stmt::<VarPrefixViolation>(stmt, config, reports);
    run_early_lint_on_stmt::<WithLoop>(stmt, config, reports);
    // @end early stmt calls. Do not remove this comment!

    // Recurse...
    let stmt = stmt.kind_mut();
    stmt.visit_child_stmts_mut(|stmt| process_stmt_early(stmt, reports, config));
    stmt.visit_child_exprs_mut(|expr| process_expr_early(expr, reports, config));
}

/// Runs an expression through the early pass, running any lint that
/// implements [EarlyExprPass], as well as collecting information
/// into the provided [GlobalScopeBuilder].
///
/// NOTE: This function is largely auto-generated! See `CONTRIBUTING.md` for
/// more information.
pub fn process_expr_early(expr: &mut Expr, reports: &mut Vec<Diagnostic<FileId>>, config: &Config) {
    // @early expr calls. Do not remove this comment!
    run_early_lint_on_expr::<AccessorAlternative>(expr, config, reports);
    run_early_lint_on_expr::<AndPreference>(expr, config, reports);
    run_early_lint_on_expr::<AnonymousConstructor>(expr, config, reports);
    run_early_lint_on_expr::<BoolEquality>(expr, config, reports);
    run_early_lint_on_expr::<CasingRules>(expr, config, reports);
    run_early_lint_on_expr::<ConditionWrapper>(expr, config, reports);
    run_early_lint_on_expr::<Deprecated>(expr, config, reports);
    run_early_lint_on_expr::<DrawSprite>(expr, config, reports);
    run_early_lint_on_expr::<DrawText>(expr, config, reports);
    run_early_lint_on_expr::<EnglishFlavorViolation>(expr, config, reports);
    run_early_lint_on_expr::<InvalidComparison>(expr, config, reports);
    run_early_lint_on_expr::<InvalidEquality>(expr, config, reports);
    run_early_lint_on_expr::<ModPreference>(expr, config, reports);
    run_early_lint_on_expr::<NonSimplifiedExpression>(expr, config, reports);
    run_early_lint_on_expr::<NotPreference>(expr, config, reports);
    run_early_lint_on_expr::<OrPreference>(expr, config, reports);
    run_early_lint_on_expr::<RoomGoto>(expr, config, reports);
    run_early_lint_on_expr::<ShowDebugMessage>(expr, config, reports);
    run_early_lint_on_expr::<SingleEqualsComparison>(expr, config, reports);
    run_early_lint_on_expr::<SuspicousConstantUsage>(expr, config, reports);
    run_early_lint_on_expr::<Todo>(expr, config, reports);
    run_early_lint_on_expr::<TooManyArguments>(expr, config, reports);
    run_early_lint_on_expr::<UnnecessaryGrouping>(expr, config, reports);
    run_early_lint_on_expr::<UnusedParameter>(expr, config, reports);
    // @end early expr calls. Do not remove this comment!

    // Recurse...
    expr.visit_child_stmts_mut(|stmt| process_stmt_early(stmt, reports, config));
    expr.visit_child_exprs_mut(|expr| process_expr_early(expr, reports, config));
}

/// Runs a [Stmt] through the late pass, running any lint that
/// implements [LateStmtPass].
///
/// NOTE: This function is largely auto-generated! See `CONTRIBUTING.md` for
/// more information.
pub fn process_stmt_late(stmt: &Stmt, reports: &mut Vec<Diagnostic<FileId>>, config: &Config) {
    // @late stmt calls. Do not remove this comment!
    run_late_lint_on_stmt::<MissingCaseMember>(stmt, config, reports);
    // @end late stmt calls. Do not remove this comment!

    // Recurse...
    let stmt = stmt.kind();
    stmt.visit_child_stmts(|stmt| process_stmt_late(stmt, reports, config));
    stmt.visit_child_exprs(|expr| process_expr_late(expr, reports, config));
}

/// Runs an expression through the late pass, running any lint that
/// implements [LateExprPass].
///
///  NOTE: This function is largely auto-generated! See `CONTRIBUTING.md`
/// for more information.
fn process_expr_late(expr: &Expr, reports: &mut Vec<Diagnostic<FileId>>, config: &Config) {
    // @late expr calls. Do not remove this comment!
    run_late_lint_on_expr::<NonConstantDefaultParameter>(expr, config, reports);
    // @end late expr calls. Do not remove this comment!

    // Recurse...
    expr.visit_child_stmts(|stmt| process_stmt_late(stmt, reports, config));
    expr.visit_child_exprs(|expr| process_expr_late(expr, reports, config));
}

/// Performs a given [EarlyStmtPass] on a statement.
fn run_early_lint_on_stmt<T: Lint + EarlyStmtPass>(
    stmt: &Stmt,
    config: &Config,
    reports: &mut Vec<Diagnostic<FileId>>,
) {
    if stmt.tag().is_none_or(|tag| !tag.eq(&("allow", Some(T::tag()))))
        && *config.get_lint_level_setting(T::tag(), T::default_level()) != LintLevel::Allow
    {
        T::visit_stmt_early(stmt, config, reports);
    }
}

/// Performs a given [EarlyExprPass] on a statement.
fn run_early_lint_on_expr<T: Lint + EarlyExprPass>(
    expr: &Expr,
    config: &Config,
    reports: &mut Vec<Diagnostic<FileId>>,
) {
    if expr.tag().is_none_or(|tag| !tag.eq(&("allow", Some(T::tag()))))
        && *config.get_lint_level_setting(T::tag(), T::default_level()) != LintLevel::Allow
    {
        T::visit_expr_early(expr, config, reports);
    }
}

/// Performs a given [LateStmtPass] on a statement.
fn run_late_lint_on_stmt<T: Lint + LateStmtPass>(stmt: &Stmt, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
    if stmt.tag().is_none_or(|tag| !tag.eq(&("allow", Some(T::tag()))))
        && *config.get_lint_level_setting(T::tag(), T::default_level()) != LintLevel::Allow
    {
        T::visit_stmt_late(stmt, config, reports);
    }
}

/// Performs a given [LateExprPass] on a statement.
fn run_late_lint_on_expr<T: Lint + LateExprPass>(expr: &Expr, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
    if expr.tag().is_none_or(|tag| !tag.eq(&("allow", Some(T::tag()))))
        && *config.get_lint_level_setting(T::tag(), T::default_level()) != LintLevel::Allow
    {
        T::visit_expr_late(expr, config, reports);
    }
}

/// Creates a Tokio task which will walk through the provided directory in
/// search of gml files. Passes each path it finds into the returned
/// Receiver. Closes when all files have been sent.
///
/// ### Panics
/// Panics if the receiver for the sender closes. This should not be possible!
pub fn start_gml_discovery(
    directory: &Path,
    files_to_ignore: Vec<String>,
) -> (Receiver<PathBuf>, JoinHandle<Vec<std::io::Error>>) {
    /// Filters DirEntry's for gml files.
    async fn filter(entry: DirEntry) -> Filtering {
        if entry.file_name().to_str().is_some_and(|f| !f.contains(".gml")) {
            Filtering::Ignore
        } else {
            Filtering::Continue
        }
    }

    let files_to_ignore: Vec<PathBuf> = files_to_ignore
        .iter()
        .map(PathBuf::from)
        .map(|v| directory.join(v).canonicalize().unwrap())
        .collect();

    let mut io_errors = vec![];
    let mut walker = WalkDir::new(directory.join("objects"))
        .filter(filter)
        .chain(WalkDir::new(directory.join("scripts")).filter(filter))
        .chain(WalkDir::new(directory.join("rooms")).filter(filter));
    let (path_sender, path_receiver) = channel::<PathBuf>(1000);
    let handle = tokio::task::spawn(async move {
        loop {
            match walker.next().await {
                Some(Ok(entry))
                    if !files_to_ignore
                        .iter()
                        .any(|v| v == &entry.path().canonicalize().unwrap()) =>
                {
                    path_sender.send(entry.path()).await.unwrap()
                }
                Some(Err(e)) => io_errors.push(e),
                None => break,
                _ => {}
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
    JoinHandle<(usize, GmlLibrary, Vec<std::io::Error>)>,
) {
    let (file_sender, file_receiver) = channel::<(FileId, &'static str)>(1000);
    let handle = tokio::task::spawn(async move {
        let mut files = GmlLibrary::new();
        let mut io_errors = vec![];
        let mut lines = 0;
        while let Some(path) = path_receiver.recv().await {
            match tokio::fs::read_to_string(&path).await {
                Ok(gml) => {
                    let gml: &'static str = Box::leak(Box::new(gml));
                    lines += gml.lines().count();
                    let file_id = files.add(path.canonicalize().unwrap().to_str().unwrap().to_string(), gml);
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
) -> (Receiver<Ast>, JoinHandle<Vec<Diagnostic<FileId>>>) {
    let (ast_sender, ast_receiver) = channel::<Ast>(1000);
    let handle = tokio::task::spawn(async move {
        let mut parse_errors = vec![];
        while let Some((file_id, gml)) = file_receiver.recv().await {
            match parse_gml(gml, &file_id) {
                Ok(ast) => ast_sender.send(ast).await.unwrap(),
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
#[allow(clippy::type_complexity)]
pub fn start_early_pass(
    config: Arc<Config>,
    mut ast_receiever: Receiver<Ast>,
) -> (
    Receiver<Stmt>,
    Sender<Vec<Diagnostic<FileId>>>,
    Receiver<Vec<Diagnostic<FileId>>>,
    JoinHandle<()>,
) {
    let (report_sender, report_receiver) = channel::<Vec<Diagnostic<FileId>>>(1000);
    let (stmt_sender, stmt_reciever) = channel::<Stmt>(1000);
    let sender = report_sender.clone();
    let handle = tokio::task::spawn(async move {
        while let Some(ast) = ast_receiever.recv().await {
            let config = config.clone();
            for mut stmt in ast.unpack() {
                let mut reports = vec![];
                process_stmt_early(&mut stmt, &mut reports, config.as_ref());
                stmt_sender.send(stmt).await.unwrap();
                sender.send(reports).await.unwrap();
            }
        }
    });
    (stmt_reciever, report_sender, report_receiver, handle)
}

/// Creates Tokio tasks for all of the provided `StmtIteration`s,
/// running the late lint pass on them. Returns a handle to another
/// Tokio task which will collect their finalized [LatePassReport]s.
///
/// ### Panics
/// Panics if the receiver for the sender closes. This should not be possible!
pub fn start_late_pass(
    config: Arc<Config>,
    mut stmt_receiver: Receiver<Stmt>,
    report_sender: Sender<Vec<Diagnostic<FileId>>>,
    mut report_receiver: Receiver<Vec<Diagnostic<FileId>>>,
) -> JoinHandle<Vec<Diagnostic<FileId>>> {
    tokio::task::spawn(async move {
        while let Some(stmt) = stmt_receiver.recv().await {
            let config = config.clone();
            let mut reports = vec![];
            process_stmt_late(&stmt, &mut reports, config.as_ref());
            report_sender.send(reports).await.unwrap();
        }
    });
    tokio::task::spawn(async move {
        let mut lint_reports = vec![];
        while let Some(mut reports) = report_receiver.recv().await {
            lint_reports.append(&mut reports);
        }
        lint_reports
    })
}

/// TODO
pub type Pass = (Stmt, Vec<Diagnostic<FileId>>);
