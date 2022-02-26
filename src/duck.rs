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

#[derive(Debug)]
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
    pub fn parse_gml(source_code: &str, path: &Path) -> Result<Ast, ParseError> {
        let mut source: &'static str = Box::leak(Box::new(source_code.to_string()));
        let ast = Parser::new(source, path.to_path_buf()).into_ast();
        unsafe {
            drop(Box::from_raw(&mut source));
        }
        ast
    }

    pub fn process_statement_early(
        config: &Config,
        statement_box: &StatementBox,
        collection: &mut GmlCollection,
        reports: &mut Vec<LintReport>,
    ) {
        let statement = statement_box.statement();
        let span = statement_box.span();

        // @early statement calls. Do not remove this comment, it used for our autogeneration!
        //
        // @end early statement calls. Do not remove this comment, it used for our autogeneration!

        #[allow(clippy::single_match)]
        match statement {
            Statement::EnumDeclaration(gml_enum) => {
                collection.register_enum(gml_enum.clone());
            }
            _ => {}
        }

        // Recurse...
        statement.visit_child_statements(|stmt| {
            Self::process_statement_early(config, stmt, collection, reports)
        });
        statement.visit_child_expressions(|expr| {
            Self::process_expression_early(config, expr, collection, reports)
        });
    }

    fn process_expression_early(
        config: &Config,
        expression_box: &ExpressionBox,
        collection: &mut GmlCollection,
        reports: &mut Vec<LintReport>,
    ) {
        let expression = expression_box.expression();
        let span = expression_box.span();

        // @early expression calls. Do not remove this comment, it used for our autogeneration!
        //
        // @end early expression calls. Do not remove this comment, it used for our autogeneration!

        // Recurse...
        expression.visit_child_statements(|stmt| {
            Self::process_statement_early(config, stmt, collection, reports)
        });
        expression.visit_child_expressions(|expr| {
            Self::process_expression_early(config, expr, collection, reports)
        });
    }

    pub fn process_statement_late(
        config: &Config,
        statement_box: &StatementBox,
        collection: &GmlCollection,
        reports: &mut Vec<LintReport>,
    ) {
        let statement = statement_box.statement();
        let span = statement_box.span();

        // @late statement calls. Do not remove this comment, it used for our autogeneration!
        Self::run_late_lint_on_statement::<MissingCaseMember>(
            config, statement, collection, span, reports,
        );
        // @end late statement calls. Do not remove this comment, it used for our autogeneration!

        // Recurse...
        statement.visit_child_statements(|stmt| {
            Self::process_statement_late(config, stmt, collection, reports)
        });
        statement.visit_child_expressions(|expr| {
            Self::process_expression_late(config, expr, collection, reports)
        });
    }

    fn process_expression_late(
        config: &Config,
        expression_box: &ExpressionBox,
        collection: &GmlCollection,
        reports: &mut Vec<LintReport>,
    ) {
        let expression = expression_box.expression();
        let span = expression_box.span();

        // @late expression calls. Do not remove this comment, it used for our autogeneration!
        //
        // @end late expression calls. Do not remove this comment, it used for our autogeneration!

        // Recurse...
        expression.visit_child_statements(|stmt| {
            Self::process_statement_late(config, stmt, collection, reports)
        });
        expression.visit_child_expressions(|expr| {
            Self::process_expression_late(config, expr, collection, reports)
        });
    }

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

    fn run_late_lint_on_statement<T: Lint + LateStatementPass>(
        config: &Config,
        statement: &Statement,
        gml_collection: &GmlCollection,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if *config.get_level_for_lint(T::tag(), T::category()) != LintLevel::Allow {
            T::visit_statement_late(config, gml_collection, statement, span, reports);
        }
    }

    fn run_late_lint_on_expression<T: Lint + LateExpressionPass>(
        config: &Config,
        expression: &Expression,
        gml_collection: &GmlCollection,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if *config.get_level_for_lint(T::tag(), T::category()) != LintLevel::Allow {
            T::visit_expression_late(config, gml_collection, expression, span, reports);
        }
    }

    /// Get a reference to the duck's config.
    pub fn config(&self) -> &Config {
        &self.config
    }
}
