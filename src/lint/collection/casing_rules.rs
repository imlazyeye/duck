use crate::{
    lint::{EarlyExprPass, EarlyStmtPass, Lint, LintLevel},
    parse::{
        Access, Expr, ExprKind, Function, Globalvar, Identifier, Literal, LocalVariables, Macro, Stmt, StmtKind,
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

impl EarlyExprPass for CasingRules {
    fn visit_expr_early(expr: &Expr, config: &Config, reports: &mut Vec<Diagnostic<crate::FileId>>) {
        match expr.kind() {
            ExprKind::Function(Function {
                name: Some(name),
                constructor: Some(_),
                ..
            }) => Self::check_for(
                name,
                config.casing_rules.constructor_rule,
                expr.file_id(),
                config,
                reports,
            ),
            ExprKind::Function(Function {
                name: Some(name),
                constructor: None,
                ..
            }) => Self::check_for(name, config.casing_rules.function_rule, expr.file_id(), config, reports),
            ExprKind::Literal(Literal::Struct(members)) => {
                for member in members {
                    // an infuriating exception because gm itself is not consistent
                    if member.0.lexeme == "toString" {
                        continue;
                    }
                    Self::check_for(
                        &member.0,
                        config.casing_rules.struct_field,
                        expr.file_id(),
                        config,
                        reports,
                    );
                }
            }
            ExprKind::Access(Access::Global { right, .. }) => {
                Self::check_for(right, config.casing_rules.global_rule, expr.file_id(), config, reports);
            }
            _ => {}
        }
    }
}

impl EarlyStmtPass for CasingRules {
    fn visit_stmt_early(stmt: &Stmt, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        match stmt.kind() {
            StmtKind::Enum(gml_enum) => {
                Self::check_for(
                    &gml_enum.name,
                    config.casing_rules.enum_rule,
                    stmt.file_id(),
                    config,
                    reports,
                );
                for member in gml_enum.members.iter() {
                    Self::check_for(
                        member.name_identifier(),
                        config.casing_rules.enum_member_rule,
                        stmt.file_id(),
                        config,
                        reports,
                    );
                }
            }
            StmtKind::Macro(Macro { name, .. }) => {
                Self::check_for(name, config.casing_rules.macro_rule, stmt.file_id(), config, reports)
            }
            StmtKind::GlobalvarDeclaration(Globalvar { name }) => {
                Self::check_for(name, config.casing_rules.global_rule, stmt.file_id(), config, reports)
            }
            StmtKind::LocalVariableSeries(LocalVariables { declarations }) => {
                for member in declarations.iter() {
                    Self::check_for(
                        member.name_identifier(),
                        config.casing_rules.local_var_rule,
                        stmt.file_id(),
                        config,
                        reports,
                    );
                }
            }
            _ => {}
        }
    }
}
