use crate::{
    lint::{EarlyExpressionPass, EarlyStatementPass, Lint, LintLevel},
    parse::{
        Access, Expression, ExpressionBox, Function, Globalvar, Identifier, Literal, LocalVariableSeries, Macro,
        Statement, StatementBox,
    },
    Casing, Config, FileId,
};
use codespan_reporting::diagnostic::{Diagnostic, Label};

#[derive(Debug, PartialEq)]
pub struct CasingRules;
impl Lint for CasingRules {
    fn explanation() -> &'static str {
        "Like any programming language, GML contains many different symbols that all can be styled in different ways. Picking consistent rules for each type creates a cleaner and more consistent codebase."
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "casing_rules"
    }
}

impl CasingRules {
    fn check_for(
        identifier: &Identifier,
        casing: Casing,
        file_id: FileId,
        config: &Config,
        reports: &mut Vec<Diagnostic<FileId>>,
    ) {
        if let Some(ideal) = casing.test(&identifier.lexeme) {
            reports.push(
                Self::diagnostic(config)
                    .with_message("Incorrect casing")
                    .with_labels(vec![
                        Label::primary(file_id, identifier.span)
                            .with_message(format!("`{}` should be `{}`", &identifier.lexeme, ideal)),
                    ]),
            )
        }
    }
}

impl EarlyExpressionPass for CasingRules {
    fn visit_expression_early(
        expression_box: &ExpressionBox,
        config: &Config,
        reports: &mut Vec<Diagnostic<crate::FileId>>,
    ) {
        match expression_box.expression() {
            Expression::FunctionDeclaration(Function {
                name: Some(name),
                constructor: Some(_),
                ..
            }) => Self::check_for(
                name,
                config.casing_rules.constructor_rule,
                expression_box.file_id(),
                config,
                reports,
            ),
            Expression::FunctionDeclaration(Function {
                name: Some(name),
                constructor: None,
                ..
            }) => Self::check_for(
                name,
                config.casing_rules.function_rule,
                expression_box.file_id(),
                config,
                reports,
            ),
            Expression::Literal(Literal::Struct(members)) => {
                for member in members {
                    // an infuriating exception because gm itself is not consistent
                    if member.0.lexeme == "toString" {
                        continue;
                    }
                    Self::check_for(
                        &member.0,
                        config.casing_rules.struct_field,
                        expression_box.file_id(),
                        config,
                        reports,
                    );
                }
            }
            Expression::Access(Access::Global { right, .. }) => {
                Self::check_for(
                    right.expression().as_identifier().unwrap(),
                    config.casing_rules.global_rule,
                    expression_box.file_id(),
                    config,
                    reports,
                );
            }
            _ => {}
        }
    }
}

impl EarlyStatementPass for CasingRules {
    fn visit_statement_early(statement_box: &StatementBox, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        match statement_box.statement() {
            Statement::MacroDeclaration(Macro { name, .. }) => Self::check_for(
                name,
                config.casing_rules.macro_rule,
                statement_box.file_id(),
                config,
                reports,
            ),
            Statement::EnumDeclaration(gml_enum) => {
                Self::check_for(
                    &gml_enum.name,
                    config.casing_rules.enum_rule,
                    statement_box.file_id(),
                    config,
                    reports,
                );
                for member in gml_enum.members.iter() {
                    Self::check_for(
                        member.name_identifier(),
                        config.casing_rules.enum_member_rule,
                        statement_box.file_id(),
                        config,
                        reports,
                    );
                }
            }
            Statement::GlobalvarDeclaration(Globalvar { name }) => Self::check_for(
                name,
                config.casing_rules.global_rule,
                statement_box.file_id(),
                config,
                reports,
            ),
            Statement::LocalVariableSeries(LocalVariableSeries { declarations }) => {
                for member in declarations.iter() {
                    Self::check_for(
                        member.name_identifier(),
                        config.casing_rules.local_var_rule,
                        statement_box.file_id(),
                        config,
                        reports,
                    );
                }
            }
            _ => {}
        }
    }
}
