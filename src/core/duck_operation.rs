use codespan_reporting::diagnostic::Diagnostic;

use crate::{
    analyze::{GlobalScope, GlobalScopeBuilder},
    lint::{collection::*, EarlyExprPass, EarlyStmtPass, LateExprPass, LateStmtPass, Lint, LintLevel},
    parse::{Ast, Expr, ParseVisitor, Parser, Stmt, StmtType},
    Config, FileId,
};

/// ## DuckOperation
///
/// FIXME: The doc tests here cannot compile, giving no specific errors. I have no idea why, but the
/// code does actually work!
///
/// Contains the core operations duck uses to parse and lint Gml.
/// These are kept seperated from [Duck] to codify that `self` should
/// not be required on these operations. This avoids situations in which
/// Arcs/Mutexs are required, reducing the amount of wasted time in our tasks.
///
/// ### Usage
/// To create an [Ast] out of a string of Gml, you can use the [DuckOperation]s
/// directly.
/// ```ignore
/// # use duck::*;
/// # let gml = "show_debug_message(\"Hello world!\")".to_string();
/// # let gml: &'static str = Box::leak(Box::new(gml));
/// # let path = "../hello_world.gml".to_string();
/// let mut gml_library = GmlLibrary::new();
/// let file_id = gml_library.add(path, gml);
/// match DuckOperation::parse_gml(gml, &file_id) {
///     Ok(ast) => {}
///     Err(parse_error) => println!("Failed to parse gml: {parse_error:?}"),
/// };
/// ```
///
/// You can also manually run the [Lint]s on these [Ast]s.
/// ```ignore
/// # use duck::*;
/// # let duck = Duck::default();
/// # let gml = "show_debug_message(\"Hello world!\")".to_string();
/// # let gml: &'static str = Box::leak(Box::new(gml));
/// # let path = "../hello_world.gml".to_string();
/// # let mut gml_library = GmlLibrary::new();
/// # let file_id = gml_library.add(path, gml);
/// # let ast = DuckOperation::parse_gml(gml, &file_id).unwrap();
/// # let stmt = ast.stmts().first().unwrap();
/// let mut global_scope = GlobalScope::new();
/// let mut reports = vec![];
/// let mut scope_builder = GlobalScopeBuilder::new();
/// DuckOperation::process_stmt_early(&stmt, &mut scope_builder, &mut reports, duck.config());
/// global_scope.drain(scope_builder);
/// DuckOperation::process_stmt_late(&stmt, &global_scope, &mut reports, duck.config());
/// ```
pub struct DuckOperation;
impl DuckOperation {
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
    pub fn process_stmt_early(
        stmt: &mut Stmt,
        scope_builder: &mut GlobalScopeBuilder,
        reports: &mut Vec<Diagnostic<FileId>>,
        config: &Config,
    ) {
        // @early stmt calls. Do not remove this comment!
        Self::run_early_lint_on_stmt::<CasingRules>(stmt, config, reports);
        Self::run_early_lint_on_stmt::<CollapsableIf>(stmt, config, reports);
        Self::run_early_lint_on_stmt::<ConditionWrapper>(stmt, config, reports);
        Self::run_early_lint_on_stmt::<Deprecated>(stmt, config, reports);
        Self::run_early_lint_on_stmt::<Exit>(stmt, config, reports);
        Self::run_early_lint_on_stmt::<Global>(stmt, config, reports);
        Self::run_early_lint_on_stmt::<InvalidAssignment>(stmt, config, reports);
        Self::run_early_lint_on_stmt::<MissingDefaultCase>(stmt, config, reports);
        Self::run_early_lint_on_stmt::<MultiVarDeclaration>(stmt, config, reports);
        Self::run_early_lint_on_stmt::<SingleSwitchCase>(stmt, config, reports);
        Self::run_early_lint_on_stmt::<SuspicousConstantUsage>(stmt, config, reports);
        Self::run_early_lint_on_stmt::<TryCatch>(stmt, config, reports);
        Self::run_early_lint_on_stmt::<UnassignedConstructor>(stmt, config, reports);
        Self::run_early_lint_on_stmt::<UnnecessaryGrouping>(stmt, config, reports);
        Self::run_early_lint_on_stmt::<UselessFunction>(stmt, config, reports);
        Self::run_early_lint_on_stmt::<VarPrefixViolation>(stmt, config, reports);
        Self::run_early_lint_on_stmt::<WithLoop>(stmt, config, reports);
        // @end early stmt calls. Do not remove this comment!

        #[allow(clippy::single_match)]
        match stmt.inner() {
            StmtType::EnumDeclaration(gml_enum) => {
                scope_builder.register_enum(gml_enum.clone(), stmt.location());
            }
            _ => {}
        }

        // Recurse...
        let stmt = stmt.inner_mut();
        stmt.visit_child_stmts_mut(|stmt| Self::process_stmt_early(stmt, scope_builder, reports, config));
        stmt.visit_child_exprs_mut(|expr| Self::process_expr_early(expr, scope_builder, reports, config));
    }

    /// Runs an expression through the early pass, running any lint that
    /// implements [EarlyExprPass], as well as collecting information
    /// into the provided [GlobalScopeBuilder].
    ///
    /// NOTE: This function is largely auto-generated! See `CONTRIBUTING.md` for
    /// more information.
    pub fn process_expr_early(
        expr: &mut Expr,
        scope_builder: &mut GlobalScopeBuilder,
        reports: &mut Vec<Diagnostic<FileId>>,
        config: &Config,
    ) {
        // @early expr calls. Do not remove this comment!
        Self::run_early_lint_on_expr::<AccessorAlternative>(expr, config, reports);
        Self::run_early_lint_on_expr::<AndPreference>(expr, config, reports);
        Self::run_early_lint_on_expr::<AnonymousConstructor>(expr, config, reports);
        Self::run_early_lint_on_expr::<BoolEquality>(expr, config, reports);
        Self::run_early_lint_on_expr::<CasingRules>(expr, config, reports);
        Self::run_early_lint_on_expr::<ConditionWrapper>(expr, config, reports);
        Self::run_early_lint_on_expr::<Deprecated>(expr, config, reports);
        Self::run_early_lint_on_expr::<DrawSprite>(expr, config, reports);
        Self::run_early_lint_on_expr::<DrawText>(expr, config, reports);
        Self::run_early_lint_on_expr::<EnglishFlavorViolation>(expr, config, reports);
        Self::run_early_lint_on_expr::<InvalidComparison>(expr, config, reports);
        Self::run_early_lint_on_expr::<InvalidEquality>(expr, config, reports);
        Self::run_early_lint_on_expr::<ModPreference>(expr, config, reports);
        Self::run_early_lint_on_expr::<NotPreference>(expr, config, reports);
        Self::run_early_lint_on_expr::<OrPreference>(expr, config, reports);
        Self::run_early_lint_on_expr::<RoomGoto>(expr, config, reports);
        Self::run_early_lint_on_expr::<ShowDebugMessage>(expr, config, reports);
        Self::run_early_lint_on_expr::<SingleEqualsComparison>(expr, config, reports);
        Self::run_early_lint_on_expr::<SuspicousConstantUsage>(expr, config, reports);
        Self::run_early_lint_on_expr::<Todo>(expr, config, reports);
        Self::run_early_lint_on_expr::<TooManyArguments>(expr, config, reports);
        Self::run_early_lint_on_expr::<UnnecessaryGrouping>(expr, config, reports);
        // @end early expr calls. Do not remove this comment!

        // Recurse...
        expr.visit_child_stmts_mut(|stmt| Self::process_stmt_early(stmt, scope_builder, reports, config));
        expr.visit_child_exprs_mut(|expr| Self::process_expr_early(expr, scope_builder, reports, config));
    }

    /// Runs a [Stmt] through the late pass, running any lint that
    /// implements [LateStmtPass].
    ///
    /// NOTE: This function is largely auto-generated! See `CONTRIBUTING.md` for
    /// more information.
    pub fn process_stmt_late(
        stmt: &Stmt,
        global_scope: &GlobalScope,
        reports: &mut Vec<Diagnostic<FileId>>,
        config: &Config,
    ) {
        // @late stmt calls. Do not remove this comment!
        Self::run_late_lint_on_stmt::<MissingCaseMember>(stmt, config, reports, global_scope);
        // @end late stmt calls. Do not remove this comment!

        // Recurse...
        let stmt = stmt.inner();
        stmt.visit_child_stmts(|stmt| Self::process_stmt_late(stmt, global_scope, reports, config));
        stmt.visit_child_exprs(|expr| Self::process_expr_late(expr, global_scope, reports, config));
    }

    /// Runs an expression through the late pass, running any lint that
    /// implements [LateExprPass].
    ///
    ///  NOTE: This function is largely auto-generated! See `CONTRIBUTING.md`
    /// for more information.
    fn process_expr_late(
        expr: &Expr,
        global_scope: &GlobalScope,
        reports: &mut Vec<Diagnostic<FileId>>,
        config: &Config,
    ) {
        // @late expr calls. Do not remove this comment!
        Self::run_late_lint_on_expr::<NonConstantDefaultParameter>(expr, config, reports, global_scope);
        // @end late expr calls. Do not remove this comment!

        // Recurse...
        expr.visit_child_stmts(|stmt| Self::process_stmt_late(stmt, global_scope, reports, config));
        expr.visit_child_exprs(|expr| Self::process_expr_late(expr, global_scope, reports, config));
    }

    /// Performs a given [EarlyStmtPass] on a statement.
    fn run_early_lint_on_stmt<T: Lint + EarlyStmtPass>(
        stmt: &Stmt,
        config: &Config,
        reports: &mut Vec<Diagnostic<FileId>>,
    ) {
        if stmt
            .lint_tag()
            .map_or(true, |tag| tag.0 == T::tag() && tag.1 != LintLevel::Allow)
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
        if expr
            .lint_tag()
            .map_or(true, |tag| tag.0 == T::tag() && tag.1 != LintLevel::Allow)
            && *config.get_lint_level_setting(T::tag(), T::default_level()) != LintLevel::Allow
        {
            T::visit_expr_early(expr, config, reports);
        }
    }

    /// Performs a given [LateStmtPass] on a statement.
    fn run_late_lint_on_stmt<T: Lint + LateStmtPass>(
        stmt: &Stmt,
        config: &Config,
        reports: &mut Vec<Diagnostic<FileId>>,
        global_scope: &GlobalScope,
    ) {
        if stmt
            .lint_tag()
            .map_or(true, |tag| tag.0 == T::tag() && tag.1 != LintLevel::Allow)
            && *config.get_lint_level_setting(T::tag(), T::default_level()) != LintLevel::Allow
        {
            T::visit_stmt_late(stmt, config, reports, global_scope);
        }
    }

    /// Performs a given [LateExprPass] on a statement.
    fn run_late_lint_on_expr<T: Lint + LateExprPass>(
        expr: &Expr,
        config: &Config,
        reports: &mut Vec<Diagnostic<FileId>>,
        global_scope: &GlobalScope,
    ) {
        if expr
            .lint_tag()
            .map_or(true, |tag| tag.0 == T::tag() && tag.1 != LintLevel::Allow)
            && *config.get_lint_level_setting(T::tag(), T::default_level()) != LintLevel::Allow
        {
            T::visit_expr_late(expr, config, reports, global_scope);
        }
    }
}
