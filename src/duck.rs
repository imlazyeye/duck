use crate::{
    lints::{
        AndKeyword, AnonymousConstructor, ConstructorWithoutNew, Deprecated, DrawSprite, DrawText,
        Exit, Global, MissingCaseMember, MissingDefaultCase, ModKeyword, NoSpaceBeginingComment,
        NonPascalCase, NonScreamCase, OrKeyword, RoomGoto, ShowDebugMessage, SingleSwitchCase,
        Todo, TooManyArguments, TooManyLines, TryCatch, WithLoop,
    },
    parsing::{
        expression::{AccessScope, Expression},
        statement::Statement,
    },
    Lint, LintReport,
};
use colored::Colorize;
use enum_map::{enum_map, EnumMap};
use std::{collections::HashMap, path::Path};

use crate::{
    lint::LintLevel,
    parsing::{parser::Ast, ParseError, Parser},
    LintCategory, LintTag,
};

pub struct Duck {
    config: DuckConfig,
    lint_tags: HashMap<(String, usize), LintTag>,
    pub category_levels: EnumMap<LintCategory, LintLevel>,
}

impl Duck {
    #[allow(clippy::new_without_default)]
    /// Creates a new, blank Duck.
    pub fn new() -> Self {
        Self {
            config: Default::default(),
            lint_tags: HashMap::new(),
            category_levels: enum_map! {
                LintCategory::Correctness => LintLevel::Deny,
                LintCategory::Suspicious => LintLevel::Warn,
                LintCategory::Style => LintLevel::Warn,
                LintCategory::Pedantic => LintLevel::Allow,
            },
        }
    }

    /// Creates a new Duck based on a DuckConfig.
    pub fn new_with_config(config: DuckConfig) -> Self {
        let mut duck = Self::new();
        duck.config = config;
        duck
    }

    /// Parses the given String of GML, collecting data for Duck.
    pub fn parse_gml(&mut self, source_code: &str, path: &Path) -> Result<Ast, ParseError> {
        Parser::new(source_code, path.to_path_buf()).into_ast()
    }

    // /// Gets the user-specified level for the given position (if one exists)
    pub fn get_user_provided_level(&self, lint_tag: &str, position: &Span) -> Option<LintLevel> {
        // Check if the line above this position has a lint tag

        // TODO OH NO

        // if let Some(tag) = self
        //     .lint_tags
        //     // that clone there... look, we're all just doing our best here, okay?
        //     .get(&(position.file_name.clone(), position.line))
        // {
        // // Check if its the right one?
        // if tag.0 == lint_tag {
        //     return Some(tag.1);
        // }
        // }

        // Check if there is a config-based rule for this lint
        if let Some((_, level)) = self
            .config
            .lint_levels
            .iter()
            .find(|(key, _)| key == &lint_tag)
        {
            return Some(*level);
        }

        // User has specificed nada
        None
    }

    /// Get a reference to the duck's config.
    pub fn config(&self) -> &DuckConfig {
        &self.config
    }

    pub fn lint_statement(&self, statement: &Statement, span: Span, reports: &mut Vec<LintReport>) {
        // Run every lint...
        AndKeyword::visit_statement(self, statement, span, reports);
        AnonymousConstructor::visit_statement(self, statement, span, reports);
        ConstructorWithoutNew::visit_statement(self, statement, span, reports);
        DrawSprite::visit_statement(self, statement, span, reports);
        DrawText::visit_statement(self, statement, span, reports);
        Exit::visit_statement(self, statement, span, reports);
        Global::visit_statement(self, statement, span, reports);
        Deprecated::visit_statement(self, statement, span, reports);
        MissingCaseMember::visit_statement(self, statement, span, reports);
        MissingDefaultCase::visit_statement(self, statement, span, reports);
        ModKeyword::visit_statement(self, statement, span, reports);
        NoSpaceBeginingComment::visit_statement(self, statement, span, reports);
        NonPascalCase::visit_statement(self, statement, span, reports);
        NonScreamCase::visit_statement(self, statement, span, reports);
        OrKeyword::visit_statement(self, statement, span, reports);
        RoomGoto::visit_statement(self, statement, span, reports);
        ShowDebugMessage::visit_statement(self, statement, span, reports);
        SingleSwitchCase::visit_statement(self, statement, span, reports);
        Todo::visit_statement(self, statement, span, reports);
        TooManyArguments::visit_statement(self, statement, span, reports);
        TooManyLines::visit_statement(self, statement, span, reports);
        TryCatch::visit_statement(self, statement, span, reports);
        WithLoop::visit_statement(self, statement, span, reports);

        // Recurse...
        match statement {
            Statement::MacroDeclaration(_, _, _) => {}
            Statement::EnumDeclaration(_, members) => {
                members.iter().flat_map(|(_, i)| i).for_each(|member| {
                    self.lint_expression(member.expression(), span, reports);
                });
            }
            Statement::GlobalvarDeclaration(_) => {}
            Statement::LocalVariableSeries(members) => {
                for member in members {
                    self.lint_expression(member.expression(), span, reports);
                }
            }
            Statement::TryCatch(try_stmt, condition, catch_stmt) => {
                self.lint_statement(try_stmt.statement(), span, reports);
                self.lint_expression(condition.expression(), span, reports);
                self.lint_statement(catch_stmt.statement(), span, reports);
            }
            Statement::For(initializer, condition, tick, body) => {
                self.lint_statement(initializer.statement(), span, reports);
                self.lint_expression(condition.expression(), span, reports);
                self.lint_statement(tick.statement(), span, reports);
                self.lint_statement(body.statement(), span, reports);
            }
            Statement::With(expression, body) => {
                self.lint_expression(expression.expression(), span, reports);
                self.lint_statement(body.statement(), span, reports);
            }
            Statement::Repeat(expression, body) => {
                self.lint_expression(expression.expression(), span, reports);
                self.lint_statement(body.statement(), span, reports);
            }
            Statement::DoUntil(body, condition) => {
                self.lint_expression(condition.expression(), span, reports);
                self.lint_statement(body.statement(), span, reports);
            }
            Statement::While(condition, body) => {
                self.lint_expression(condition.expression(), span, reports);
                self.lint_statement(body.statement(), span, reports);
            }
            Statement::If(condition, body, else_branch) => {
                self.lint_expression(condition.expression(), span, reports);
                self.lint_statement(body.statement(), span, reports);
                if let Some(else_branch) = else_branch {
                    self.lint_statement(else_branch.statement(), span, reports);
                }
            }
            Statement::Switch(identity, cases, default) => {
                self.lint_expression(identity.expression(), span, reports);
                for case in cases {
                    self.lint_expression(case.0.expression(), span, reports);
                    for statement in case.1.iter() {
                        self.lint_statement(statement.statement(), span, reports);
                    }
                }
                if let Some(default) = default {
                    for statement in default.iter() {
                        self.lint_statement(statement.statement(), span, reports);
                    }
                }
            }
            Statement::Block(statements) => {
                for statement in statements {
                    self.lint_statement(statement.statement(), span, reports);
                }
            }
            Statement::Return(value) => {
                if let Some(value) = value {
                    self.lint_expression(value.expression(), span, reports);
                }
            }
            Statement::Expression(expression) => {
                self.lint_expression(expression.expression(), span, reports);
            }
            Statement::Break | Statement::Continue | Statement::Exit => {}
        }
    }

    pub fn lint_expression(
        &self,
        expression: &Expression,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        // Run every lint...
        AndKeyword::visit_expression(self, expression, span, reports);
        AnonymousConstructor::visit_expression(self, expression, span, reports);
        ConstructorWithoutNew::visit_expression(self, expression, span, reports);
        DrawSprite::visit_expression(self, expression, span, reports);
        DrawText::visit_expression(self, expression, span, reports);
        Exit::visit_expression(self, expression, span, reports);
        Global::visit_expression(self, expression, span, reports);
        Deprecated::visit_expression(self, expression, span, reports);
        MissingCaseMember::visit_expression(self, expression, span, reports);
        MissingDefaultCase::visit_expression(self, expression, span, reports);
        ModKeyword::visit_expression(self, expression, span, reports);
        NoSpaceBeginingComment::visit_expression(self, expression, span, reports);
        NonPascalCase::visit_expression(self, expression, span, reports);
        NonScreamCase::visit_expression(self, expression, span, reports);
        OrKeyword::visit_expression(self, expression, span, reports);
        RoomGoto::visit_expression(self, expression, span, reports);
        ShowDebugMessage::visit_expression(self, expression, span, reports);
        SingleSwitchCase::visit_expression(self, expression, span, reports);
        Todo::visit_expression(self, expression, span, reports);
        TooManyArguments::visit_expression(self, expression, span, reports);
        TooManyLines::visit_expression(self, expression, span, reports);
        TryCatch::visit_expression(self, expression, span, reports);
        WithLoop::visit_expression(self, expression, span, reports);

        // Recurse...
        match expression {
            Expression::FunctionDeclaration(_, parameters, constructor, body, _) => {
                for parameter in parameters.iter() {
                    if let Some(default_value) = &parameter.1 {
                        self.lint_expression(default_value.expression(), span, reports);
                    }
                }
                if let Some(Some(inheritance_call)) = constructor.as_ref().map(|c| &c.0) {
                    self.lint_expression(inheritance_call.expression(), span, reports);
                }
                self.lint_statement(body.statement(), span, reports);
            }
            Expression::Logical(left, _, right)
            | Expression::Equality(left, _, right)
            | Expression::Evaluation(left, _, right)
            | Expression::Assignment(left, _, right)
            | Expression::NullCoalecence(left, right) => {
                self.lint_expression(left.expression(), span, reports);
                self.lint_expression(right.expression(), span, reports);
            }
            Expression::Ternary(condition, left, right) => {
                self.lint_expression(condition.expression(), span, reports);
                self.lint_expression(left.expression(), span, reports);
                self.lint_expression(right.expression(), span, reports);
            }
            Expression::Unary(_, right) => {
                self.lint_expression(right.expression(), span, reports);
            }
            Expression::Postfix(left, _) => {
                self.lint_expression(left.expression(), span, reports);
            }
            Expression::Access(expression, access) => {
                self.lint_expression(expression.expression(), span, reports);
                match access {
                    AccessScope::Dot(other) => {
                        self.lint_expression(other.expression(), span, reports);
                    }
                    AccessScope::Array(x, y, _) => {
                        self.lint_expression(x.expression(), span, reports);
                        if let Some(y) = y {
                            self.lint_expression(y.expression(), span, reports);
                        }
                    }
                    AccessScope::Map(key) => {
                        self.lint_expression(key.expression(), span, reports);
                    }
                    AccessScope::Grid(x, y) => {
                        self.lint_expression(x.expression(), span, reports);
                        self.lint_expression(y.expression(), span, reports);
                    }
                    AccessScope::List(index) => {
                        self.lint_expression(index.expression(), span, reports);
                    }
                    AccessScope::Struct(key) => {
                        self.lint_expression(key.expression(), span, reports);
                    }
                    AccessScope::Global | AccessScope::Current => {}
                }
            }
            Expression::Call(left, arguments, _) => {
                self.lint_expression(left.expression(), span, reports);
                for arg in arguments {
                    self.lint_expression(arg.expression(), span, reports);
                }
            }
            Expression::Grouping(expression) => {
                self.lint_expression(expression.expression(), span, reports);
            }
            Expression::Literal(_) | Expression::Identifier(_) => {}
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DuckConfig {
    pub todo_keyword: Option<String>,
    pub max_arguments: Option<usize>,
    pub lint_levels: HashMap<String, LintLevel>,
}
impl Default for DuckConfig {
    fn default() -> Self {
        Self {
            todo_keyword: Default::default(),
            max_arguments: Some(7),
            lint_levels: Default::default(),
        }
    }
}

impl DuckConfig {
    /// Get a reference to the duck config's todo keyword.
    pub fn todo_keyword(&self) -> Option<&String> {
        self.todo_keyword.as_ref()
    }

    /// Get the duck config's max arguments.
    pub fn max_arguments(&self) -> Option<usize> {
        self.max_arguments
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Position<'a> {
    pub file_name: &'a str,
    pub line: usize,
    pub column: usize,
    pub snippet: &'a str,
}
impl<'a> Position<'a> {
    pub fn new(file_contents: &'a str, file_name: &'a str, cursor: usize) -> Self {
        let mut line = 1;
        let mut column = 0;
        file_contents[..cursor].chars().for_each(|c| {
            if c == '\n' {
                line += 1;
                column = 0;
            } else {
                column += 1;
            }
        });
        let line_and_after = &file_contents[cursor - column..];
        let last_index = line_and_after
            .match_indices('\n')
            .next()
            .map_or(line_and_after.len() - 1, |(i, _)| i - 1);
        let snippet = &line_and_after[..last_index];
        Self {
            file_name,
            line,
            column,
            snippet,
        }
    }

    pub fn file_string(&self) -> String {
        format!("{}:{}:{}", self.file_name, self.line, self.column)
    }

    pub fn snippet_message(&self) -> String {
        format!(
            "{}\n{}{}\n{}",
            " | ".bright_blue().bold(),
            " | ".bright_blue().bold(),
            self.snippet,
            " | ".bright_blue().bold()
        )
    }

    pub fn path_message(&self) -> String {
        format!(" {} {}", "-->".bold().bright_blue(), self.file_string())
    }
}

#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct Span(pub usize, pub usize);
