use crate::config::Config;
use crate::gml::Environment;
use crate::lint::{
    EarlyExpressionPass, EarlyStatementPass, LateExpressionPass, LateStatementPass, LintLevel,
};
use crate::lints::*;
use crate::parsing::{parser::Ast, ParseError, Parser};
use crate::{
    parsing::{expression::Expression, statement::Statement},
    Lint, LintReport,
};
use crate::{
    parsing::{expression::ExpressionBox, statement::StatementBox},
    utils::Span,
};
use std::path::Path;

/// Contains the core operations duck uses to parse and lint Gml.
/// These are kept seperated from [Duck] to codify that `self` should
/// not be required on these operations. This avoids situations in which
/// Arcs/Mutexs are required, reducing the amount of wasted time in our tasks.
pub struct DuckOperation;
impl DuckOperation {
    /// Parses the given String of GML, returning either a successfully constructed [Ast]
    /// or a [ParseError].
    pub fn parse_gml(source_code: &str, path: &Path) -> Result<Ast, ParseError> {
        let mut source: &'static str = Box::leak(Box::new(source_code.to_string()));
        let ast = Parser::new(source, path.to_path_buf()).into_ast();
        unsafe {
            drop(Box::from_raw(&mut source));
        }
        ast
    }

    /// Runs a [Statement] through the early pass, running any lint that implements [EarlyStatementPass],
    /// as well as collecting information into the provided [Environment].
    ///
    /// NOTE: This function is largely auto-generated! See `CONTRIBUTING.md` for more information.
    pub fn process_statement_early(
        config: &Config,
        statement_box: &StatementBox,
        environment: &mut Environment,
        reports: &mut Vec<LintReport>,
    ) {
        let statement = statement_box.statement();
        let span = statement_box.span();

        // @early statement calls. Do not remove this comment, it used for our autogeneration!
        Self::run_early_lint_on_statement::<Deprecated>(config, statement, span, reports);
        Self::run_early_lint_on_statement::<Exit>(config, statement, span, reports);
        Self::run_early_lint_on_statement::<MissingDefaultCase>(config, statement, span, reports);
        Self::run_early_lint_on_statement::<MultiVarDeclaration>(config, statement, span, reports);
        Self::run_early_lint_on_statement::<NonPascalCase>(config, statement, span, reports);
        Self::run_early_lint_on_statement::<NonScreamCase>(config, statement, span, reports);
        Self::run_early_lint_on_statement::<SingleSwitchCase>(config, statement, span, reports);
        Self::run_early_lint_on_statement::<StatementParentheticalViolation>(
            config, statement, span, reports,
        );
        Self::run_early_lint_on_statement::<TryCatch>(config, statement, span, reports);
        Self::run_early_lint_on_statement::<VarPrefixViolation>(config, statement, span, reports);
        Self::run_early_lint_on_statement::<WithLoop>(config, statement, span, reports);
        // @end early statement calls. Do not remove this comment, it used for our autogeneration!

        #[allow(clippy::single_match)]
        match statement {
            Statement::EnumDeclaration(gml_enum) => {
                environment.register_enum(gml_enum.clone());
            }
            _ => {}
        }

        // Recurse...
        statement.visit_child_statements(|stmt| {
            Self::process_statement_early(config, stmt, environment, reports)
        });
        statement.visit_child_expressions(|expr| {
            Self::process_expression_early(config, expr, environment, reports)
        });
    }

    /// Runs an [Expression] through the early pass, running any lint that implements [EarlyExpressionPass],
    /// as well as collecting information into the provided [Environment].
    ///
    /// NOTE: This function is largely auto-generated! See `CONTRIBUTING.md` for more information.
    pub fn process_expression_early(
        config: &Config,
        expression_box: &ExpressionBox,
        environment: &mut Environment,
        reports: &mut Vec<LintReport>,
    ) {
        let expression = expression_box.expression();
        let span = expression_box.span();

        // @early expression calls. Do not remove this comment, it used for our autogeneration!
        Self::run_early_lint_on_expression::<AccessorAlternative>(
            config, expression, span, reports,
        );
        Self::run_early_lint_on_expression::<AnonymousConstructor>(
            config, expression, span, reports,
        );
        Self::run_early_lint_on_expression::<AssignmentToCall>(config, expression, span, reports);
        Self::run_early_lint_on_expression::<BoolEquality>(config, expression, span, reports);
        Self::run_early_lint_on_expression::<Deprecated>(config, expression, span, reports);
        Self::run_early_lint_on_expression::<DrawSprite>(config, expression, span, reports);
        Self::run_early_lint_on_expression::<DrawText>(config, expression, span, reports);
        Self::run_early_lint_on_expression::<EnglishFlavorViolation>(
            config, expression, span, reports,
        );
        Self::run_early_lint_on_expression::<Global>(config, expression, span, reports);
        Self::run_early_lint_on_expression::<NonConstantDefaultParameter>(
            config, expression, span, reports,
        );
        Self::run_early_lint_on_expression::<NonPascalCase>(config, expression, span, reports);
        Self::run_early_lint_on_expression::<RoomGoto>(config, expression, span, reports);
        Self::run_early_lint_on_expression::<ShowDebugMessage>(config, expression, span, reports);
        Self::run_early_lint_on_expression::<SuspicousConstantUsage>(
            config, expression, span, reports,
        );
        Self::run_early_lint_on_expression::<Todo>(config, expression, span, reports);
        Self::run_early_lint_on_expression::<TooManyArguments>(config, expression, span, reports);
        // @end early expression calls. Do not remove this comment, it used for our autogeneration!

        // Recurse...
        expression.visit_child_statements(|stmt| {
            Self::process_statement_early(config, stmt, environment, reports)
        });
        expression.visit_child_expressions(|expr| {
            Self::process_expression_early(config, expr, environment, reports)
        });
    }

    /// Runs a [Statement] through the late pass, running any lint that implements [LateStatementPass].
    ///
    /// NOTE: This function is largely auto-generated! See `CONTRIBUTING.md` for more information.
    pub fn process_statement_late(
        config: &Config,
        statement_box: &StatementBox,
        environment: &Environment,
        reports: &mut Vec<LintReport>,
    ) {
        let statement = statement_box.statement();
        let span = statement_box.span();

        // @late statement calls. Do not remove this comment, it used for our autogeneration!
        Self::run_late_lint_on_statement::<MissingCaseMember>(
            config,
            statement,
            environment,
            span,
            reports,
        );
        // @end late statement calls. Do not remove this comment, it used for our autogeneration!

        // Recurse...
        statement.visit_child_statements(|stmt| {
            Self::process_statement_late(config, stmt, environment, reports)
        });
        statement.visit_child_expressions(|expr| {
            Self::process_expression_late(config, expr, environment, reports)
        });
    }

    /// Runs an [Expression] through the late pass, running any lint that implements [LateExpressionPass].
    ///
    ///  NOTE: This function is largely auto-generated! See `CONTRIBUTING.md` for more information.
    #[allow(dead_code)]
    fn process_expression_late(
        config: &Config,
        expression_box: &ExpressionBox,
        environment: &Environment,
        reports: &mut Vec<LintReport>,
    ) {
        let expression = expression_box.expression();
        #[allow(unused_variables)]
        let span = expression_box.span();

        // @late expression calls. Do not remove this comment, it used for our autogeneration!
        // @end late expression calls. Do not remove this comment, it used for our autogeneration!

        // Recurse...
        expression.visit_child_statements(|stmt| {
            Self::process_statement_late(config, stmt, environment, reports)
        });
        expression.visit_child_expressions(|expr| {
            Self::process_expression_late(config, expr, environment, reports)
        });
    }

    /// Performs a given [EarlyStatementPass] on a statement.
    fn run_early_lint_on_statement<T: Lint + EarlyStatementPass>(
        config: &Config,
        statement: &Statement,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if *config.get_level_for_lint(T::tag(), T::category()) != LintLevel::Allow {
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
        if *config.get_level_for_lint(T::tag(), T::category()) != LintLevel::Allow {
            T::visit_expression_early(config, expression, span, reports);
        }
    }

    /// Performs a given [LateStatementPass] on a statement.
    fn run_late_lint_on_statement<T: Lint + LateStatementPass>(
        config: &Config,
        statement: &Statement,
        environment: &Environment,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if *config.get_level_for_lint(T::tag(), T::category()) != LintLevel::Allow {
            T::visit_statement_late(config, environment, statement, span, reports);
        }
    }

    /// Performs a given [LateExpressionPass] on a statement.
    #[allow(dead_code)]
    fn run_late_lint_on_expression<T: Lint + LateExpressionPass>(
        config: &Config,
        expression: &Expression,
        environment: &Environment,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if *config.get_level_for_lint(T::tag(), T::category()) != LintLevel::Allow {
            T::visit_expression_late(config, environment, expression, span, reports);
        }
    }
}
