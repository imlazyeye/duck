use crate::config::Config;
use crate::lint::LintLevelSetting;
use crate::parsing::expression::ExpressionBox;
use crate::parsing::statement::StatementBox;
use crate::utils::Span;
use crate::{lints::*, LintCategory};
use crate::{
    parsing::{expression::Expression, statement::Statement},
    Lint, LintReport,
};
use std::path::Path;

use crate::{
    lint::LintLevel,
    parsing::{parser::Ast, ParseError, Parser},
};

pub struct Duck {
    config: Config,
}

impl Duck {
    #[allow(clippy::new_without_default)]
    /// Creates a new, blank Duck.
    pub fn new() -> Self {
        Self {
            config: Default::default(),
        }
    }

    /// Creates a new Duck based on a DuckConfig.
    pub fn new_with_config(config: Config) -> Self {
        let mut duck = Self::new();
        duck.config = config;
        duck
    }

    pub fn lint_gml(
        &mut self,
        gml_source: String,
        path: &Path,
        reports: &mut Vec<LintReport>,
    ) -> Result<(), ParseError> {
        let mut source: &'static str = Box::leak(Box::new(gml_source));
        self.parse_gml(source, path)?
            .into_iter()
            .for_each(|statement| {
                self.process_statement(&statement, reports);
            });
        unsafe {
            drop(Box::from_raw(&mut source));
        }
        Ok(())
    }

    fn try_run_lint_on_statement<T: Lint>(
        &self,
        statement: &Statement,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if *self.get_level_for_lint(T::tag(), T::category()) != LintLevel::Allow {
            T::visit_statement(self, statement, span, reports);
        }
    }

    fn try_run_lint_on_expression<T: Lint>(
        &self,
        expression: &Expression,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if *self.get_level_for_lint(T::tag(), T::category()) != LintLevel::Allow {
            T::visit_expression(self, expression, span, reports);
        }
    }

    pub fn process_statement(&self, statement_box: &StatementBox, reports: &mut Vec<LintReport>) {
        let statement = statement_box.statement();
        let span = statement_box.span();

        // @statement calls. Do not remove this comment, it used for our autogeneration!
        self.try_run_lint_on_statement::<Deprecated>(statement, span, reports);
        self.try_run_lint_on_statement::<Exit>(statement, span, reports);
        self.try_run_lint_on_statement::<MissingDefaultCase>(statement, span, reports);
        self.try_run_lint_on_statement::<MultiVarDeclaration>(statement, span, reports);
        self.try_run_lint_on_statement::<NonPascalCase>(statement, span, reports);
        self.try_run_lint_on_statement::<NonScreamCase>(statement, span, reports);
        self.try_run_lint_on_statement::<SingleSwitchCase>(statement, span, reports);
        self.try_run_lint_on_statement::<StatementParentheticalViolation>(statement, span, reports);
        self.try_run_lint_on_statement::<TryCatch>(statement, span, reports);
        self.try_run_lint_on_statement::<VarPrefixViolation>(statement, span, reports);
        self.try_run_lint_on_statement::<WithLoop>(statement, span, reports);
        // @end statement calls. Do not remove this comment, it used for our autogeneration!

        // Recurse...
        statement.visit_child_statements(|stmt| self.process_statement(stmt, reports));
        statement.visit_child_expressions(|expr| self.process_expression(expr, reports));
    }

    fn process_expression(&self, expression_box: &ExpressionBox, reports: &mut Vec<LintReport>) {
        let expression = expression_box.expression();
        let span = expression_box.span();

        // @expression calls. Do not remove this comment, it used for our autogeneration!
        self.try_run_lint_on_expression::<AccessorAlternative>(expression, span, reports);
        self.try_run_lint_on_expression::<AnonymousConstructor>(expression, span, reports);
        self.try_run_lint_on_expression::<AssignmentToCall>(expression, span, reports);
        self.try_run_lint_on_expression::<BoolEquality>(expression, span, reports);
        self.try_run_lint_on_expression::<Deprecated>(expression, span, reports);
        self.try_run_lint_on_expression::<DrawSprite>(expression, span, reports);
        self.try_run_lint_on_expression::<DrawText>(expression, span, reports);
        self.try_run_lint_on_expression::<EnglishFlavorViolation>(expression, span, reports);
        self.try_run_lint_on_expression::<Global>(expression, span, reports);
        self.try_run_lint_on_expression::<NonConstantDefaultParameter>(expression, span, reports);
        self.try_run_lint_on_expression::<NonPascalCase>(expression, span, reports);
        self.try_run_lint_on_expression::<RoomGoto>(expression, span, reports);
        self.try_run_lint_on_expression::<ShowDebugMessage>(expression, span, reports);
        self.try_run_lint_on_expression::<SuspicousConstantUsage>(expression, span, reports);
        self.try_run_lint_on_expression::<Todo>(expression, span, reports);
        self.try_run_lint_on_expression::<TooManyArguments>(expression, span, reports);
        // @end expression calls. Do not remove this comment, it used for our autogeneration!

        // Recurse...
        expression.visit_child_statements(|stmt| self.process_statement(stmt, reports));
        expression.visit_child_expressions(|expr| self.process_expression(expr, reports));
    }

    /// Parses the given String of GML, collecting data for Duck.
    pub fn parse_gml(&mut self, source_code: &'static str, path: &Path) -> Result<Ast, ParseError> {
        Parser::new(source_code, path.to_path_buf()).into_ast()
    }

    // /// Gets the user-desired level for the lint tag.
    pub fn get_level_for_lint(
        &self,
        lint_tag: &str,
        lint_category: LintCategory,
    ) -> LintLevelSetting {
        // Check if there is a config-based rule for this lint
        if let Some((_, level)) = self
            .config
            .lint_levels
            .iter()
            .find(|(key, _)| key == &lint_tag)
        {
            return LintLevelSetting::ConfigSpecified(*level);
        }

        // User has specificed nada
        LintLevelSetting::Default(lint_category.default_level())
    }

    /// Get a reference to the duck's config.
    pub fn config(&self) -> &Config {
        &self.config
    }
}
