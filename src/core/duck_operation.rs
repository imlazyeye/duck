use codespan_reporting::diagnostic::Diagnostic;

use crate::{
    analyze::{GlobalScope, GlobalScopeBuilder},
    lint::{
        collection::*, EarlyExpressionPass, EarlyStatementPass, LateExpressionPass, LateStatementPass, Lint, LintLevel,
    },
    parse::{Ast, ExpressionBox, ParseVisitor, Parser, Statement, StatementBox},
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
/// # let statement_box = ast.statements().first().unwrap();
/// let mut global_scope = GlobalScope::new();
/// let mut reports = vec![];
/// let mut scope_builder = GlobalScopeBuilder::new();
/// DuckOperation::process_statement_early(&statement_box, &mut scope_builder, &mut reports, duck.config());
/// global_scope.drain(scope_builder);
/// DuckOperation::process_statement_late(&statement_box, &global_scope, &mut reports, duck.config());
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

    /// Runs a [Statement] through the early pass, running any lint that
    /// implements [EarlyStatementPass], as well as collecting information
    /// into the provided [GlobalScopeBuilder].
    ///
    /// NOTE: This function is largely auto-generated! See `CONTRIBUTING.md` for
    /// more information.
    pub fn process_statement_early(
        statement_box: &StatementBox,
        scope_builder: &mut GlobalScopeBuilder,
        reports: &mut Vec<Diagnostic<FileId>>,
        config: &Config,
    ) {
        let statement = statement_box.statement();

        // @early statement calls. Do not remove this comment!
        Self::run_early_lint_on_statement::<CasingRules>(statement_box, config, reports);
        Self::run_early_lint_on_statement::<CollapsableIf>(statement_box, config, reports);
        Self::run_early_lint_on_statement::<Deprecated>(statement_box, config, reports);
        Self::run_early_lint_on_statement::<Exit>(statement_box, config, reports);
        Self::run_early_lint_on_statement::<Global>(statement_box, config, reports);
        Self::run_early_lint_on_statement::<InvalidAssignment>(statement_box, config, reports);
        Self::run_early_lint_on_statement::<MissingDefaultCase>(statement_box, config, reports);
        Self::run_early_lint_on_statement::<MultiVarDeclaration>(statement_box, config, reports);
        Self::run_early_lint_on_statement::<SingleSwitchCase>(statement_box, config, reports);
        Self::run_early_lint_on_statement::<ConditionWrapper>(statement_box, config, reports);
        Self::run_early_lint_on_statement::<SuspicousConstantUsage>(statement_box, config, reports);
        Self::run_early_lint_on_statement::<TryCatch>(statement_box, config, reports);
        Self::run_early_lint_on_statement::<UnassignedConstructor>(statement_box, config, reports);
        Self::run_early_lint_on_statement::<UnnecessaryGrouping>(statement_box, config, reports);
        Self::run_early_lint_on_statement::<UselessFunction>(statement_box, config, reports);
        Self::run_early_lint_on_statement::<VarPrefixViolation>(statement_box, config, reports);
        Self::run_early_lint_on_statement::<WithLoop>(statement_box, config, reports);
        // @end early statement calls. Do not remove this comment!

        #[allow(clippy::single_match)]
        match statement {
            Statement::EnumDeclaration(gml_enum) => {
                scope_builder.register_enum(gml_enum.clone(), statement_box.location());
            }
            _ => {}
        }

        // Recurse...
        statement.visit_child_statements(|stmt| Self::process_statement_early(stmt, scope_builder, reports, config));
        statement.visit_child_expressions(|expr| Self::process_expression_early(expr, scope_builder, reports, config));
    }

    /// Runs an [Expression] through the early pass, running any lint that
    /// implements [EarlyExpressionPass], as well as collecting information
    /// into the provided [GlobalScopeBuilder].
    ///
    /// NOTE: This function is largely auto-generated! See `CONTRIBUTING.md` for
    /// more information.
    pub fn process_expression_early(
        expression_box: &ExpressionBox,
        scope_builder: &mut GlobalScopeBuilder,
        reports: &mut Vec<Diagnostic<FileId>>,
        config: &Config,
    ) {
        let expression = expression_box.expression();

        // @early expression calls. Do not remove this comment!
        Self::run_early_lint_on_expression::<AccessorAlternative>(expression_box, config, reports);
        Self::run_early_lint_on_expression::<AndPreference>(expression_box, config, reports);
        Self::run_early_lint_on_expression::<AnonymousConstructor>(expression_box, config, reports);
        Self::run_early_lint_on_expression::<BoolEquality>(expression_box, config, reports);
        Self::run_early_lint_on_expression::<CasingRules>(expression_box, config, reports);
        Self::run_early_lint_on_expression::<Deprecated>(expression_box, config, reports);
        Self::run_early_lint_on_expression::<DrawSprite>(expression_box, config, reports);
        Self::run_early_lint_on_expression::<DrawText>(expression_box, config, reports);
        Self::run_early_lint_on_expression::<EnglishFlavorViolation>(expression_box, config, reports);
        Self::run_early_lint_on_expression::<InvalidComparison>(expression_box, config, reports);
        Self::run_early_lint_on_expression::<InvalidEquality>(expression_box, config, reports);
        Self::run_early_lint_on_expression::<ModPreference>(expression_box, config, reports);
        Self::run_early_lint_on_expression::<NotPreference>(expression_box, config, reports);
        Self::run_early_lint_on_expression::<OrPreference>(expression_box, config, reports);
        Self::run_early_lint_on_expression::<RoomGoto>(expression_box, config, reports);
        Self::run_early_lint_on_expression::<ShowDebugMessage>(expression_box, config, reports);
        Self::run_early_lint_on_expression::<SingleEqualsComparison>(expression_box, config, reports);
        Self::run_early_lint_on_expression::<SuspicousConstantUsage>(expression_box, config, reports);
        Self::run_early_lint_on_expression::<Todo>(expression_box, config, reports);
        Self::run_early_lint_on_expression::<TooManyArguments>(expression_box, config, reports);
        Self::run_early_lint_on_expression::<UnnecessaryGrouping>(expression_box, config, reports);
        // @end early expression calls. Do not remove this comment!

        // Recurse...
        expression.visit_child_statements(|stmt| Self::process_statement_early(stmt, scope_builder, reports, config));
        expression.visit_child_expressions(|expr| Self::process_expression_early(expr, scope_builder, reports, config));
    }

    /// Runs a [Statement] through the late pass, running any lint that
    /// implements [LateStatementPass].
    ///
    /// NOTE: This function is largely auto-generated! See `CONTRIBUTING.md` for
    /// more information.
    pub fn process_statement_late(
        statement_box: &StatementBox,
        global_scope: &GlobalScope,
        reports: &mut Vec<Diagnostic<FileId>>,
        config: &Config,
    ) {
        let statement = statement_box.statement();

        // @late statement calls. Do not remove this comment!
        Self::run_late_lint_on_statement::<MissingCaseMember>(statement_box, config, reports, global_scope);
        // @end late statement calls. Do not remove this comment!

        // Recurse...
        statement.visit_child_statements(|stmt| Self::process_statement_late(stmt, global_scope, reports, config));
        statement.visit_child_expressions(|expr| Self::process_expression_late(expr, global_scope, reports, config));
    }

    /// Runs an [Expression] through the late pass, running any lint that
    /// implements [LateExpressionPass].
    ///
    ///  NOTE: This function is largely auto-generated! See `CONTRIBUTING.md`
    /// for more information.
    #[allow(dead_code)]
    fn process_expression_late(
        expression_box: &ExpressionBox,
        global_scope: &GlobalScope,
        reports: &mut Vec<Diagnostic<FileId>>,
        config: &Config,
    ) {
        let expression = expression_box.expression();
        #[allow(unused_variables)]
        let span = expression_box.span();

        // @late expression calls. Do not remove this comment!
        Self::run_late_lint_on_expression::<NonConstantDefaultParameter>(expression_box, config, reports, global_scope);
        // @end late expression calls. Do not remove this comment!

        // Recurse...
        expression.visit_child_statements(|stmt| Self::process_statement_late(stmt, global_scope, reports, config));
        expression.visit_child_expressions(|expr| Self::process_expression_late(expr, global_scope, reports, config));
    }

    /// Performs a given [EarlyStatementPass] on a statement.
    fn run_early_lint_on_statement<T: Lint + EarlyStatementPass>(
        statement_box: &StatementBox,
        config: &Config,
        reports: &mut Vec<Diagnostic<FileId>>,
    ) {
        if statement_box
            .lint_tag()
            .map_or(true, |tag| tag.0 == T::tag() && tag.1 != LintLevel::Allow)
            && *config.get_lint_level_setting(T::tag(), T::default_level()) != LintLevel::Allow
        {
            T::visit_statement_early(statement_box, config, reports);
        }
    }

    /// Performs a given [EarlyExpressionPass] on a statement.
    fn run_early_lint_on_expression<T: Lint + EarlyExpressionPass>(
        expression_box: &ExpressionBox,
        config: &Config,
        reports: &mut Vec<Diagnostic<FileId>>,
    ) {
        if expression_box
            .lint_tag()
            .map_or(true, |tag| tag.0 == T::tag() && tag.1 != LintLevel::Allow)
            && *config.get_lint_level_setting(T::tag(), T::default_level()) != LintLevel::Allow
        {
            T::visit_expression_early(expression_box, config, reports);
        }
    }

    /// Performs a given [LateStatementPass] on a statement.
    fn run_late_lint_on_statement<T: Lint + LateStatementPass>(
        statement_box: &StatementBox,
        config: &Config,
        reports: &mut Vec<Diagnostic<FileId>>,
        global_scope: &GlobalScope,
    ) {
        if statement_box
            .lint_tag()
            .map_or(true, |tag| tag.0 == T::tag() && tag.1 != LintLevel::Allow)
            && *config.get_lint_level_setting(T::tag(), T::default_level()) != LintLevel::Allow
        {
            T::visit_statement_late(statement_box, config, reports, global_scope);
        }
    }

    /// Performs a given [LateExpressionPass] on a statement.
    #[allow(dead_code)]
    fn run_late_lint_on_expression<T: Lint + LateExpressionPass>(
        expression_box: &ExpressionBox,
        config: &Config,
        reports: &mut Vec<Diagnostic<FileId>>,
        global_scope: &GlobalScope,
    ) {
        if expression_box
            .lint_tag()
            .map_or(true, |tag| tag.0 == T::tag() && tag.1 != LintLevel::Allow)
            && *config.get_lint_level_setting(T::tag(), T::default_level()) != LintLevel::Allow
        {
            T::visit_expression_late(expression_box, config, reports, global_scope);
        }
    }
}
