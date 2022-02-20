use crate::{
    lints::{
        AndKeyword, AnonymousConstructor, ConstructorWithoutNew, DrawSprite, DrawText, Exit,
        Global, Globalvar, MissingCaseMember, MissingDefaultCase, ModKeyword,
        NoSpaceBeginingComment, NonPascalCase, NonScreamCase, OrKeyword, RoomGoto,
        ShowDebugMessage, SingleSwitchCase, Todo, TooManyArguments, TooManyLines, TryCatch,
        WithLoop,
    },
    parsing::{expression::{Expression, AccessScope}, statement::Statement},
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
    pub fn get_user_provided_level(
        &self,
        lint_tag: &str,
        position: &Position,
    ) -> Option<LintLevel> {
        // Check if the line above this position has a lint tag
        if let Some(tag) = self
            .lint_tags
            // that clone there... look, we're all just doing our best here, okay?
            .get(&(position.file_name.clone(), position.line))
        {
            // Check if its the right one?
            if tag.0 == lint_tag {
                return Some(tag.1);
            }
        }

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

    pub fn lint_statement(
        &self,
        statement: &Statement,
        position: &Position,
        reports: &mut Vec<LintReport>,
    ) {
        // Run every lint...
        AndKeyword::visit_statement(self, statement, position, reports);
        AnonymousConstructor::visit_statement(self, statement, position, reports);
        ConstructorWithoutNew::visit_statement(self, statement, position, reports);
        DrawSprite::visit_statement(self, statement, position, reports);
        DrawText::visit_statement(self, statement, position, reports);
        Exit::visit_statement(self, statement, position, reports);
        Global::visit_statement(self, statement, position, reports);
        Globalvar::visit_statement(self, statement, position, reports);
        MissingCaseMember::visit_statement(self, statement, position, reports);
        MissingDefaultCase::visit_statement(self, statement, position, reports);
        ModKeyword::visit_statement(self, statement, position, reports);
        NoSpaceBeginingComment::visit_statement(self, statement, position, reports);
        NonPascalCase::visit_statement(self, statement, position, reports);
        NonScreamCase::visit_statement(self, statement, position, reports);
        OrKeyword::visit_statement(self, statement, position, reports);
        RoomGoto::visit_statement(self, statement, position, reports);
        ShowDebugMessage::visit_statement(self, statement, position, reports);
        SingleSwitchCase::visit_statement(self, statement, position, reports);
        Todo::visit_statement(self, statement, position, reports);
        TooManyArguments::visit_statement(self, statement, position, reports);
        TooManyLines::visit_statement(self, statement, position, reports);
        TryCatch::visit_statement(self, statement, position, reports);
        WithLoop::visit_statement(self, statement, position, reports);

        // Recurse...
        match statement {
            Statement::MacroDeclaration(_, _, _) => {}
            Statement::EnumDeclaration(_, members) => {
                members.iter().flat_map(|(_, i)| i).for_each(|member| {
                    self.lint_expression(&*member, position, reports);
                });
            }
            Statement::GlobalvarDeclaration(_) => {}
            Statement::LocalVariableSeries(members) => {
                for member in members {
                    self.lint_expression(&*member, position, reports);
                }
            }
            Statement::TryCatch(try_stmt, condition, catch_stmt) => {
                self.lint_statement(&*try_stmt, position, reports);
                self.lint_expression(&*condition, position, reports);
                self.lint_statement(&*catch_stmt, position, reports);
            }
            Statement::For(initializer, condition, tick, body) => {
                self.lint_statement(&*initializer, position, reports);
                self.lint_expression(&*condition, position, reports);
                self.lint_statement(&*tick, position, reports);
                self.lint_statement(&*body, position, reports);
            }
            Statement::With(expression, body) => {
                self.lint_expression(&*expression, position, reports);
                self.lint_statement(&*body, position, reports);
            }
            Statement::Repeat(expression, body) => {
                self.lint_expression(&*expression, position, reports);
                self.lint_statement(&*body, position, reports);
            }
            Statement::DoUntil(body, condition) => {
                self.lint_expression(&*condition, position, reports);
                self.lint_statement(&*body, position, reports);
            }
            Statement::While(condition, body) => {
                self.lint_expression(&*condition, position, reports);
                self.lint_statement(&*body, position, reports);
            }
            Statement::If(condition, body, else_branch) => {
                self.lint_expression(&*condition, position, reports);
                self.lint_statement(&*body, position, reports);
                if let Some(else_branch) = else_branch {
                    self.lint_statement(&*else_branch, position, reports);
                }
            }
            Statement::Switch(identity, cases, default) => {
                self.lint_expression(&*identity, position, reports);
                for case in cases {
                    self.lint_expression(&*case.0, position, reports);
                    for statement in case.1.iter() {
                        self.lint_statement(&*statement, position, reports);
                    }
                }
                if let Some(default) = default {
                    for statement in default.iter() {
                        self.lint_statement(&*statement, position, reports);
                    }
                }
            }
            Statement::Block(statements) => {
                for statement in statements {
                    self.lint_statement(&*statement, position, reports);
                }
            }
            Statement::Return(value) => {
                if let Some(value) = value {
                    self.lint_expression(&*value, position, reports);
                }
            }
            Statement::Break => {}
            Statement::Continue => {}
            Statement::Exit => {}
            Statement::Expression(expression) => {
                self.lint_expression(&*expression, position, reports);
            }
        }
    }

    pub fn lint_expression(
        &self,
        expression: &Expression,
        position: &Position,
        reports: &mut Vec<LintReport>,
    ) {
        // Run every lint...
        AndKeyword::visit_expression(self, expression, position, reports);
        AnonymousConstructor::visit_expression(self, expression, position, reports);
        ConstructorWithoutNew::visit_expression(self, expression, position, reports);
        DrawSprite::visit_expression(self, expression, position, reports);
        DrawText::visit_expression(self, expression, position, reports);
        Exit::visit_expression(self, expression, position, reports);
        Global::visit_expression(self, expression, position, reports);
        Globalvar::visit_expression(self, expression, position, reports);
        MissingCaseMember::visit_expression(self, expression, position, reports);
        MissingDefaultCase::visit_expression(self, expression, position, reports);
        ModKeyword::visit_expression(self, expression, position, reports);
        NoSpaceBeginingComment::visit_expression(self, expression, position, reports);
        NonPascalCase::visit_expression(self, expression, position, reports);
        NonScreamCase::visit_expression(self, expression, position, reports);
        OrKeyword::visit_expression(self, expression, position, reports);
        RoomGoto::visit_expression(self, expression, position, reports);
        ShowDebugMessage::visit_expression(self, expression, position, reports);
        SingleSwitchCase::visit_expression(self, expression, position, reports);
        Todo::visit_expression(self, expression, position, reports);
        TooManyArguments::visit_expression(self, expression, position, reports);
        TooManyLines::visit_expression(self, expression, position, reports);
        TryCatch::visit_expression(self, expression, position, reports);
        WithLoop::visit_expression(self, expression, position, reports);

        // Recurse...
        match expression {
            Expression::FunctionDeclaration(_, parameters, constructor, body, _) => {
                for parameter in parameters.iter() {
                    if let Some(default_value) = &parameter.1 {
                        self.lint_expression(&*default_value, position, reports);
                    }
                }
                if let Some(Some(inheritance_call)) = constructor.as_ref().map(|c| &c.0) {
                    self.lint_expression(&*inheritance_call, position, reports);
                }
                self.lint_statement(&*body, position, reports);
            }
            Expression::Logical(left, _, right)
            | Expression::Equality(left, _, right)
            | Expression::Evaluation(left, _, right)
            | Expression::Assignment(left, _, right)
            | Expression::NullCoalecence(left, right) => {
                self.lint_expression(&*left, position, reports);
                self.lint_expression(&*right, position, reports);
            }
            Expression::Ternary(condition, left, right) => {
                self.lint_expression(&*condition, position, reports);
                self.lint_expression(&*left, position, reports);
                self.lint_expression(&*right, position, reports);
            }
            Expression::Unary(_, right) => {
                self.lint_expression(&*right, position, reports);
            }
            Expression::Postfix(left, _) => {
                self.lint_expression(&*left, position, reports);
            }
            Expression::Access(expression, access) => {
                self.lint_expression(&*expression, position, reports);
                match access {
                    AccessScope::Dot(other) => {
                        self.lint_expression(&*other, position, reports);
                    }
                    AccessScope::Array(x, y, _) => {
                        self.lint_expression(&*x, position, reports);
                        if let Some(y) = y {
                            self.lint_expression(&*y, position, reports);
                        }
                    }
                    AccessScope::Map(key) => {
                        self.lint_expression(&*key, position, reports);
                    }
                    AccessScope::Grid(x, y) => {
                        self.lint_expression(&*x, position, reports);
                        self.lint_expression(&*y, position, reports);
                    }
                    AccessScope::List(index) => {
                        self.lint_expression(&*index, position, reports);
                    }
                    AccessScope::Struct(key) => {
                        self.lint_expression(&*key, position, reports);
                    }
                    AccessScope::Global | AccessScope::Current => {}
                }
            }
            Expression::Call(left, arguments, _) => {
                self.lint_expression(&*left, position, reports);
                for arg in arguments {
                    self.lint_expression(&*arg, position, reports);
                }
            }
            Expression::Grouping(expression) => {
                self.lint_expression(&*expression, position, reports);
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
pub struct Position {
    pub file_name: String,
    pub line: usize,
    pub column: usize,
    pub file_string: String,
    pub snippet: String,
}
impl Position {
    pub fn new(file_contents: &str, file_name: &str, cursor: usize) -> Self {
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
            file_name: file_name.to_string(),
            line,
            column,
            file_string: format!("{}:{}:{}", file_name, line, column),
            snippet: snippet.to_string(),
        }
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
        format!(" {} {}", "-->".bold().bright_blue(), self.file_string)
    }
}
