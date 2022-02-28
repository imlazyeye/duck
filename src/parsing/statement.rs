use super::ExpressionBox;
use crate::{
    gml::{GmlEnum, GmlSwitch},
    utils::Span,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    MacroDeclaration(String, Option<String>, String),
    EnumDeclaration(GmlEnum),
    GlobalvarDeclaration(String),
    LocalVariableSeries(Vec<(String, Option<ExpressionBox>)>),
    TryCatch(StatementBox, ExpressionBox, StatementBox, Option<StatementBox>),
    For(StatementBox, ExpressionBox, StatementBox, StatementBox),
    With(ExpressionBox, StatementBox),
    Repeat(ExpressionBox, StatementBox),
    DoUntil(StatementBox, ExpressionBox),
    While(ExpressionBox, StatementBox),
    If(ExpressionBox, StatementBox, Option<StatementBox>, bool),
    Switch(GmlSwitch),
    Block(Vec<StatementBox>),
    Return(Option<ExpressionBox>),
    Break,
    Continue,
    Exit,
    Expression(ExpressionBox),
}
impl Statement {
    pub fn into_box(self, span: Span) -> StatementBox {
        StatementBox(Box::new(self), span)
    }

    pub fn lazy_box(self) -> StatementBox {
        StatementBox(Box::new(self), Span::default())
    }

    pub fn visit_child_statements<S>(&self, mut statement_visitor: S)
    where
        S: FnMut(&StatementBox),
    {
        match self {
            Statement::TryCatch(try_stmt, _, catch_stmt, finally_stmt) => {
                statement_visitor(try_stmt);
                statement_visitor(catch_stmt);
                if let Some(finally_stmt) = finally_stmt {
                    statement_visitor(finally_stmt);
                }
            }
            Statement::For(initializer, _, tick, body) => {
                statement_visitor(initializer);
                statement_visitor(tick);
                statement_visitor(body);
            }
            Statement::With(_, body)
            | Statement::Repeat(_, body)
            | Statement::DoUntil(body, _)
            | Statement::While(_, body) => {
                statement_visitor(body);
            }
            Statement::If(_, body, else_branch, _) => {
                statement_visitor(body);
                if let Some(else_branch) = else_branch {
                    statement_visitor(else_branch);
                }
            }
            Statement::Switch(switch) => {
                for case in switch.cases() {
                    for statement in case.iter_body_statements() {
                        statement_visitor(statement);
                    }
                }
                if let Some(default) = switch.default_case() {
                    for statement in default.iter() {
                        statement_visitor(statement);
                    }
                }
            }
            Statement::Block(statements) => {
                for statement in statements {
                    statement_visitor(statement);
                }
            }
            Statement::MacroDeclaration(_, _, _)
            | Statement::EnumDeclaration(_)
            | Statement::GlobalvarDeclaration(_)
            | Statement::LocalVariableSeries(_)
            | Statement::Return(_)
            | Statement::Expression(_)
            | Statement::Break
            | Statement::Continue
            | Statement::Exit => {}
        }
    }

    pub fn visit_child_expressions<E>(&self, mut expression_visitor: E)
    where
        E: FnMut(&ExpressionBox),
    {
        match self {
            Statement::EnumDeclaration(gml_enum) => {
                gml_enum
                    .members()
                    .iter()
                    .flat_map(|member| member.initializer())
                    .for_each(|initializer| {
                        expression_visitor(initializer);
                    });
            }
            Statement::GlobalvarDeclaration(_) => {}
            Statement::LocalVariableSeries(members) => {
                for member in members.iter().flat_map(|(_, e)| e) {
                    expression_visitor(member);
                }
            }
            Statement::Switch(switch) => {
                expression_visitor(switch.matching_value());
                for case in switch.cases() {
                    expression_visitor(case.identity());
                }
            }
            Statement::Return(value) => {
                if let Some(value) = value {
                    expression_visitor(value);
                }
            }
            Statement::TryCatch(_, expression, _, _)
            | Statement::For(_, expression, _, _)
            | Statement::With(expression, _)
            | Statement::Repeat(expression, _)
            | Statement::DoUntil(_, expression)
            | Statement::While(expression, _)
            | Statement::Expression(expression)
            | Statement::If(expression, _, _, _) => {
                expression_visitor(expression);
            }
            Statement::MacroDeclaration(_, _, _)
            | Statement::Block(_)
            | Statement::Break
            | Statement::Continue
            | Statement::Exit => {}
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct StatementBox(pub Box<Statement>, pub Span);
impl StatementBox {
    pub fn statement(&self) -> &Statement {
        self.0.as_ref()
    }
    pub fn span(&self) -> Span {
        self.1
    }
}
