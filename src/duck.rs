use crate::config::Config;
use crate::gml::GmlCollection;
use crate::lint::{
    EarlyExpressionPass, EarlyStatementPass, LateExpressionPass, LateStatementPass,
    LintLevelSetting,
};
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

    /// Parses the given String of GML, collecting data for Duck.
    pub fn parse_gml(&mut self, source_code: &str, path: &Path) -> Result<Ast, ParseError> {
        let mut source: &'static str = Box::leak(Box::new(source_code.to_string()));
        let ast = Parser::new(source, path.to_path_buf()).into_ast();
        unsafe {
            drop(Box::from_raw(&mut source));
        }
        ast
    }

    pub fn process_statement_early(
        &self,
        statement_box: &StatementBox,
        collection: &mut GmlCollection,
        reports: &mut Vec<LintReport>,
    ) {
        let statement = statement_box.statement();
        let span = statement_box.span();

        // @early statement calls. Do not remove this comment, it used for our autogeneration!
        self.run_early_lint_on_statement::<Deprecated>(statement, span, reports);
        self.run_early_lint_on_statement::<Exit>(statement, span, reports);
        self.run_early_lint_on_statement::<MissingDefaultCase>(statement, span, reports);
        self.run_early_lint_on_statement::<MultiVarDeclaration>(statement, span, reports);
        self.run_early_lint_on_statement::<NonPascalCase>(statement, span, reports);
        self.run_early_lint_on_statement::<NonScreamCase>(statement, span, reports);
        self.run_early_lint_on_statement::<SingleSwitchCase>(statement, span, reports);
        self.run_early_lint_on_statement::<StatementParentheticalViolation>(
            statement, span, reports,
        );
        self.run_early_lint_on_statement::<TryCatch>(statement, span, reports);
        self.run_early_lint_on_statement::<VarPrefixViolation>(statement, span, reports);
        self.run_early_lint_on_statement::<WithLoop>(statement, span, reports);
        // @end early statement calls. Do not remove this comment, it used for our autogeneration!

        #[allow(clippy::single_match)]
        match statement {
            Statement::EnumDeclaration(gml_enum) => {
                collection.register_enum(gml_enum.clone());
            }
            _ => {}
        }

        // Recurse...
        statement
            .visit_child_statements(|stmt| self.process_statement_early(stmt, collection, reports));
        statement.visit_child_expressions(|expr| {
            self.process_expression_early(expr, collection, reports)
        });
    }

    fn process_expression_early(
        &self,
        expression_box: &ExpressionBox,
        collection: &mut GmlCollection,
        reports: &mut Vec<LintReport>,
    ) {
        let expression = expression_box.expression();
        let span = expression_box.span();

        // @early expression calls. Do not remove this comment, it used for our autogeneration!
        self.run_early_lint_on_expression::<AccessorAlternative>(expression, span, reports);
        self.run_early_lint_on_expression::<AnonymousConstructor>(expression, span, reports);
        self.run_early_lint_on_expression::<AssignmentToCall>(expression, span, reports);
        self.run_early_lint_on_expression::<BoolEquality>(expression, span, reports);
        self.run_early_lint_on_expression::<Deprecated>(expression, span, reports);
        self.run_early_lint_on_expression::<DrawSprite>(expression, span, reports);
        self.run_early_lint_on_expression::<DrawText>(expression, span, reports);
        self.run_early_lint_on_expression::<EnglishFlavorViolation>(expression, span, reports);
        self.run_early_lint_on_expression::<Global>(expression, span, reports);
        self.run_early_lint_on_expression::<NonConstantDefaultParameter>(expression, span, reports);
        self.run_early_lint_on_expression::<NonPascalCase>(expression, span, reports);
        self.run_early_lint_on_expression::<RoomGoto>(expression, span, reports);
        self.run_early_lint_on_expression::<ShowDebugMessage>(expression, span, reports);
        self.run_early_lint_on_expression::<SuspicousConstantUsage>(expression, span, reports);
        self.run_early_lint_on_expression::<Todo>(expression, span, reports);
        self.run_early_lint_on_expression::<TooManyArguments>(expression, span, reports);
        // @end early expression calls. Do not remove this comment, it used for our autogeneration!

        // Recurse...
        expression
            .visit_child_statements(|stmt| self.process_statement_early(stmt, collection, reports));
        expression.visit_child_expressions(|expr| {
            self.process_expression_early(expr, collection, reports)
        });
    }

    pub fn process_statement_late(
        &self,
        statement_box: &StatementBox,
        collection: &GmlCollection,
        reports: &mut Vec<LintReport>,
    ) {
        let statement = statement_box.statement();
        let span = statement_box.span();

        // @late statement calls. Do not remove this comment, it used for our autogeneration!
        self.run_late_lint_on_statement::<MissingCaseMember>(statement, collection, span, reports);
        // @end late statement calls. Do not remove this comment, it used for our autogeneration!

        // Recurse...
        statement
            .visit_child_statements(|stmt| self.process_statement_late(stmt, collection, reports));
        statement.visit_child_expressions(|expr| {
            self.process_expression_late(expr, collection, reports)
        });
    }

    fn process_expression_late(
        &self,
        expression_box: &ExpressionBox,
        collection: &GmlCollection,
        reports: &mut Vec<LintReport>,
    ) {
        let expression = expression_box.expression();
        let span = expression_box.span();

        // @late expression calls. Do not remove this comment, it used for our autogeneration!
        // @end late expression calls. Do not remove this comment, it used for our autogeneration!

        // Recurse...
        expression
            .visit_child_statements(|stmt| self.process_statement_late(stmt, collection, reports));
        expression.visit_child_expressions(|expr| {
            self.process_expression_late(expr, collection, reports)
        });
    }

    fn run_early_lint_on_statement<T: Lint + EarlyStatementPass>(
        &self,
        statement: &Statement,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if *self.get_level_for_lint(T::tag(), T::category()) != LintLevel::Allow {
            T::visit_statement_early(self, statement, span, reports);
        }
    }

    fn run_early_lint_on_expression<T: Lint + EarlyExpressionPass>(
        &self,
        expression: &Expression,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if *self.get_level_for_lint(T::tag(), T::category()) != LintLevel::Allow {
            T::visit_expression_early(self, expression, span, reports);
        }
    }

    fn run_late_lint_on_statement<T: Lint + LateStatementPass>(
        &self,
        statement: &Statement,
        gml_collection: &GmlCollection,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if *self.get_level_for_lint(T::tag(), T::category()) != LintLevel::Allow {
            T::visit_statement_late(self, gml_collection, statement, span, reports);
        }
    }

    fn run_late_lint_on_expression<T: Lint + LateExpressionPass>(
        &self,
        expression: &Expression,
        gml_collection: &GmlCollection,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if *self.get_level_for_lint(T::tag(), T::category()) != LintLevel::Allow {
            T::visit_expression_late(self, gml_collection, expression, span, reports);
        }
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
