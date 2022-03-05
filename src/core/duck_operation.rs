use crate::{
    analysis::{GlobalScope, GlobalScopeBuilder},
    lint::{
        EarlyExpressionPass, EarlyStatementPass, LateExpressionPass, LateStatementPass, Lint, LintLevel, LintReport,
    },
    lints::*,
    parsing::{Ast, Expression, ExpressionBox, ParseError, ParseVisitor, Parser, Statement, StatementBox},
    utils::Span,
    Config,
};
use std::path::Path;

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
/// # use std::path::Path;
/// # let gml = "show_debug_message(\"Hello world!\")";
/// # let path = Path::new("../hello_world.gml");
/// match DuckOperation::parse_gml("test", Path::new("test")) {
///     Ok(ast) => {},
///     Err(parse_error) => println!("Failed to parse gml: {parse_error:?}"),
/// };
/// ```
///
/// You can also manually run the [Lint]s on these [Ast]s.
/// ```ignore
/// # use duck::*;
/// # use std::path::Path;
/// # let duck = Duck::default();
/// # let path = Path::new("../test.gml");
/// # let ast = DuckOperation::parse_gml("var a = 0;", path).unwrap();
/// let mut gml_environment = GmlEnvironment::new();
/// let mut lint_reports: Vec<LintReport> = vec![];
/// let mut scope_builder = ScopeBuilder::new();
/// DuckOperation::process_statement_early(
///     &ast[0],
///     &mut scope_builder,
///     &mut lint_reports,
///     duck.config(),
/// );
/// let global_id = gml_environment.global_id();
/// let scope_id = gml_environment.new_scope(scope_builder, global_id);
/// DuckOperation::process_statement_late(
///     &ast[0],
///     &gml_environment,
///     &scope_id,
///     &mut lint_reports,
///     duck.config(),
/// );
/// ```
pub struct DuckOperation;
impl DuckOperation {
    /// Parses the given String of GML, returning either a successfully
    /// constructed [Ast] or a [ParseError].
    ///
    /// ### Errors
    ///
    /// Returns a [ParseError] if the parsing was unsuccessful.
    pub fn parse_gml(source_code: &str, path: &Path) -> Result<Ast, ParseError> {
        let mut source: &'static str = Box::leak(Box::new(source_code.to_string()));
        let ast = Parser::new(source, path.to_path_buf()).into_ast();
        // SAFETY: to dance around the borrow checker, we leak the inputted string and then
        // restore it to memory here in order to not actually leak the memory.
        unsafe {
            drop(Box::from_raw(&mut source));
        }
        ast
    }

    /// Runs a [Statement] through the early pass, running any lint that
    /// implements [EarlyStatementPass], as well as collecting information
    /// into the provided [Environment].
    ///
    /// NOTE: This function is largely auto-generated! See `CONTRIBUTING.md` for
    /// more information.
    pub fn process_statement_early(
        config: &Config,
        statement_box: &StatementBox,
        scope_builder: &mut GlobalScopeBuilder,
        reports: &mut Vec<LintReport>,
    ) {
        let statement = statement_box.statement();
        let span = statement_box.span();

        // @early statement calls. Do not remove this comment!
        Self::run_early_lint_on_statement::<AssignmentToCall>(config, statement, span, reports);
        Self::run_early_lint_on_statement::<CollapsableIf>(config, statement, span, reports);
        Self::run_early_lint_on_statement::<Deprecated>(config, statement, span, reports);
        Self::run_early_lint_on_statement::<Exit>(config, statement, span, reports);
        Self::run_early_lint_on_statement::<MissingDefaultCase>(config, statement, span, reports);
        Self::run_early_lint_on_statement::<MultiVarDeclaration>(config, statement, span, reports);
        Self::run_early_lint_on_statement::<NonPascalCase>(config, statement, span, reports);
        Self::run_early_lint_on_statement::<NonScreamCase>(config, statement, span, reports);
        Self::run_early_lint_on_statement::<SingleSwitchCase>(config, statement, span, reports);
        Self::run_early_lint_on_statement::<StatementParentheticalPreference>(config, statement, span, reports);
        Self::run_early_lint_on_statement::<SuspicousConstantUsage>(config, statement, span, reports);
        Self::run_early_lint_on_statement::<TryCatch>(config, statement, span, reports);
        Self::run_early_lint_on_statement::<VarPrefixViolation>(config, statement, span, reports);
        Self::run_early_lint_on_statement::<WithLoop>(config, statement, span, reports);
        // @end early statement calls. Do not remove this comment!

        #[allow(clippy::single_match)]
        match statement {
            Statement::EnumDeclaration(gml_enum) => {
                scope_builder.register_enum(gml_enum.clone());
            }
            _ => {}
        }

        // Recurse...
        statement.visit_child_statements(|stmt| Self::process_statement_early(config, stmt, scope_builder, reports));
        statement.visit_child_expressions(|expr| Self::process_expression_early(config, expr, scope_builder, reports));
    }

    /// Runs an [Expression] through the early pass, running any lint that
    /// implements [EarlyExpressionPass], as well as collecting information
    /// into the provided [Environment].
    ///
    /// NOTE: This function is largely auto-generated! See `CONTRIBUTING.md` for
    /// more information.
    pub fn process_expression_early(
        config: &Config,
        expression_box: &ExpressionBox,
        scope_builder: &mut GlobalScopeBuilder,
        reports: &mut Vec<LintReport>,
    ) {
        let expression = expression_box.expression();
        let span = expression_box.span();

        // @early expression calls. Do not remove this comment!
        Self::run_early_lint_on_expression::<AccessorAlternative>(config, expression, span, reports);
        Self::run_early_lint_on_expression::<AndPreference>(config, expression, span, reports);
        Self::run_early_lint_on_expression::<AnonymousConstructor>(config, expression, span, reports);
        Self::run_early_lint_on_expression::<BoolEquality>(config, expression, span, reports);
        Self::run_early_lint_on_expression::<Deprecated>(config, expression, span, reports);
        Self::run_early_lint_on_expression::<DrawSprite>(config, expression, span, reports);
        Self::run_early_lint_on_expression::<DrawText>(config, expression, span, reports);
        Self::run_early_lint_on_expression::<EnglishFlavorViolation>(config, expression, span, reports);
        Self::run_early_lint_on_expression::<Global>(config, expression, span, reports);
        Self::run_early_lint_on_expression::<ModPreference>(config, expression, span, reports);
        Self::run_early_lint_on_expression::<NonConstantDefaultParameter>(config, expression, span, reports);
        Self::run_early_lint_on_expression::<NonPascalCase>(config, expression, span, reports);
        Self::run_early_lint_on_expression::<NotPreference>(config, expression, span, reports);
        Self::run_early_lint_on_expression::<OrPreference>(config, expression, span, reports);
        Self::run_early_lint_on_expression::<RoomGoto>(config, expression, span, reports);
        Self::run_early_lint_on_expression::<ShowDebugMessage>(config, expression, span, reports);
        Self::run_early_lint_on_expression::<SuspicousConstantUsage>(config, expression, span, reports);
        Self::run_early_lint_on_expression::<Todo>(config, expression, span, reports);
        Self::run_early_lint_on_expression::<TooManyArguments>(config, expression, span, reports);
        // @end early expression calls. Do not remove this comment!

        // Recurse...
        expression.visit_child_statements(|stmt| Self::process_statement_early(config, stmt, scope_builder, reports));
        expression.visit_child_expressions(|expr| Self::process_expression_early(config, expr, scope_builder, reports));
    }

    /// Runs a [Statement] through the late pass, running any lint that
    /// implements [LateStatementPass].
    ///
    /// NOTE: This function is largely auto-generated! See `CONTRIBUTING.md` for
    /// more information.
    pub fn process_statement_late(
        config: &Config,
        statement_box: &StatementBox,
        environment: &GlobalScope,
        reports: &mut Vec<LintReport>,
    ) {
        let statement = statement_box.statement();
        let span = statement_box.span();

        // @late statement calls. Do not remove this comment!
        Self::run_late_lint_on_statement::<MissingCaseMember>(config, statement, environment, span, reports);
        // @end late statement calls. Do not remove this comment!

        // Recurse...
        statement.visit_child_statements(|stmt| Self::process_statement_late(config, stmt, environment, reports));
        statement.visit_child_expressions(|expr| Self::process_expression_late(config, expr, environment, reports));
    }

    /// Runs an [Expression] through the late pass, running any lint that
    /// implements [LateExpressionPass].
    ///
    ///  NOTE: This function is largely auto-generated! See `CONTRIBUTING.md`
    /// for more information.
    #[allow(dead_code)]
    fn process_expression_late(
        config: &Config,
        expression_box: &ExpressionBox,
        environment: &GlobalScope,
        reports: &mut Vec<LintReport>,
    ) {
        let expression = expression_box.expression();
        #[allow(unused_variables)]
        let span = expression_box.span();

        // @late expression calls. Do not remove this comment!
        // @end late expression calls. Do not remove this comment!

        // Recurse...
        expression.visit_child_statements(|stmt| Self::process_statement_late(config, stmt, environment, reports));
        expression.visit_child_expressions(|expr| Self::process_expression_late(config, expr, environment, reports));
    }

    /// Performs a given [EarlyStatementPass] on a statement.
    fn run_early_lint_on_statement<T: Lint + EarlyStatementPass>(
        config: &Config,
        statement: &Statement,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if *config.get_lint_level_setting(T::tag(), T::default_level()) != LintLevel::Allow {
            T::visit_statement_early(config, statement, span, reports);
        }
    }

    /// Performs a given [EarlyExpressionPass] on a statement.
    fn run_early_lint_on_expression<T: Lint + EarlyExpressionPass>(
        config: &Config,
        expression: &Expression,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if *config.get_lint_level_setting(T::tag(), T::default_level()) != LintLevel::Allow {
            T::visit_expression_early(config, expression, span, reports);
        }
    }

    /// Performs a given [LateStatementPass] on a statement.
    fn run_late_lint_on_statement<T: Lint + LateStatementPass>(
        config: &Config,
        statement: &Statement,
        environment: &GlobalScope,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if *config.get_lint_level_setting(T::tag(), T::default_level()) != LintLevel::Allow {
            T::visit_statement_late(config, environment, statement, span, reports);
        }
    }

    /// Performs a given [LateExpressionPass] on a statement.
    #[allow(dead_code)]
    fn run_late_lint_on_expression<T: Lint + LateExpressionPass>(
        config: &Config,
        expression: &Expression,
        environment: &GlobalScope,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if *config.get_lint_level_setting(T::tag(), T::default_level()) != LintLevel::Allow {
            T::visit_expression_late(config, environment, expression, span, reports);
        }
    }
}
