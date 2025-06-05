use codespan_reporting::diagnostic::{Diagnostic, Label};
use colored::Colorize;
use hashbrown::{HashMap, HashSet};

use crate::{
    Config, FileId,
    lint::{AstPass, EarlyExprPass, Lint, LintLevel},
    parse::{Ast, Block, Expr, ExprKind, For, Function, Identifier, ParseVisitor, Stmt, StmtKind},
};

#[derive(Debug, PartialEq)]
pub struct UnusedLocalVariable;
impl Lint for UnusedLocalVariable {
    fn explanation() -> &'static str {
        "Unused local variables are at best clutter and at worst the source of hard-to-spot bug."
    }

    fn default_level() -> LintLevel {
        LintLevel::Warn
    }

    fn tag() -> &'static str {
        "unused_local_variable"
    }
}

impl AstPass for UnusedLocalVariable {
    fn visit_ast(ast: &Ast, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        analyze_scope_on_stmts(ast.stmts(), config, reports);
    }
}

fn analyze_scope_on_stmts(stmts: &[Stmt], config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
    let mut scope: HashMap<String, (Identifier, FileId)> = HashMap::new();
    stmts.iter().for_each(|v| analyze_stmt(v, &mut scope, config, reports));

    for (_, (ident, file_id)) in scope.iter() {
        reports.push(
            UnusedLocalVariable::diagnostic(config)
                .with_message("Unused local variable")
                .with_labels(vec![
                    Label::primary(*file_id, ident.span).with_message(format!("Unused local variable: {}", ident)),
                ])
                .with_notes(vec![format!(
                    "{}: you can prefix this variable with an underscore to mark it as intentionally ignored (_{})",
                    "help".bold(),
                    ident
                )]),
        );
    }
}

fn analyze_stmt(
    stmt: &Stmt,
    scope: &mut HashMap<String, (Identifier, FileId)>,
    config: &Config,
    reports: &mut Vec<Diagnostic<FileId>>,
) {
    if let StmtKind::LocalVariables(variables) = stmt.kind() {
        variables.declarations.iter().for_each(|v| {
            if !v.name_identifier().lexeme.starts_with('_') {
                scope.insert(
                    v.name_identifier().lexeme.clone(),
                    (v.name_identifier().clone(), stmt.file_id()),
                );
            }

            if let Some(value) = v.assignment_value() {
                analyze_expr(value, scope, config, reports);
            }
        });
        return;
    }

    stmt.visit_child_stmts(|s| analyze_stmt(s, scope, config, reports));
    stmt.visit_child_exprs(|e| analyze_expr(e, scope, config, reports));
}

fn analyze_expr(
    expr: &Expr,
    scope: &mut HashMap<String, (Identifier, FileId)>,
    config: &Config,
    reports: &mut Vec<Diagnostic<FileId>>,
) {
    match expr.kind() {
        ExprKind::Function(Function { body, .. }) => {
            let StmtKind::Block(Block { body, .. }) = body.kind() else {
                unreachable!();
            };
            analyze_scope_on_stmts(body, config, reports);
            return;
        }
        ExprKind::Identifier(ident) => {
            scope.remove(&ident.lexeme);
        }
        _ => {}
    }

    expr.visit_child_exprs(|e| analyze_expr(e, scope, config, reports));
    expr.visit_child_stmts(|s| analyze_stmt(s, scope, config, reports));
}
