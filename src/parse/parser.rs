use codespan_reporting::diagnostic::{Diagnostic, Label};
use colored::Colorize;
use itertools::Itertools;

use crate::{parse::*, FileId};
use std::{iter::Peekable, ops::Range};

/// Recursively decsends Gml source, incremently returning various statements
/// and expressions.
pub struct Parser {
    lexer: Peekable<Lexer>,
    cursor: usize,
    file_id: FileId,
    comments: Vec<Token>,
    use_default_ids: bool,
    tag_queue: Option<Tag>,
    active_tag: Option<Tag>,
}

// Basic features
impl Parser {
    /// Creates a new parser.
    pub fn new(source_code: &'static str, file_id: FileId) -> Self {
        Self {
            lexer: Lexer::new(source_code).peekable(),
            cursor: 0,
            file_id,
            comments: vec![],
            use_default_ids: false,
            tag_queue: None,
            active_tag: None,
        }
    }

    /// Creates a new parser that will use `0` for all id's on expressions. Useful for
    /// testing when you want to not deal with the random ids.
    pub fn new_with_default_ids(source_code: &'static str, file_id: FileId) -> Self {
        let mut parser = Self::new(source_code, file_id);
        parser.use_default_ids = true;
        parser
    }

    /// Runs the parser through the entire source, collecting everything into an
    /// Ast and returning it.
    ///
    /// ### Errors
    ///
    /// Returns a [ParseError] if any of the source code caused an error.
    pub fn into_ast(mut self) -> Result<Ast, Diagnostic<FileId>> {
        let mut statements = vec![];
        while self.soft_peek().is_some() {
            statements.push(self.stmt()?);
        }
        Ok(Ast::new(statements))
    }

    /// Creates a new expression.
    fn new_expr(&mut self, expr: impl IntoExpr, span: Span) -> Expr {
        expr.into_expr(
            if self.use_default_ids {
                ExprId::default()
            } else {
                ExprId::new()
            },
            span,
            self.file_id,
            self.tag_queue.as_ref().cloned(),
        )
    }

    /// Creates a new statement.
    fn new_stmt(&mut self, stmt: impl IntoStmt, start_position: usize) -> Stmt {
        stmt.into_stmt(
            if self.use_default_ids {
                StmtId::default()
            } else {
                StmtId::new()
            },
            self.span(start_position),
            self.file_id,
            self.active_tag.as_ref().cloned(),
        )
    }

    /// Creates a [Span] from the given position up until the pilot's current
    /// position.
    fn span(&self, start: usize) -> Span {
        Span::new(start, self.cursor + 1)
    }
}

// Recursive descent (gml grammar)
impl Parser {
    /// Parses the source gml for a new statement.
    ///
    ///  ### Errors
    ///
    /// Returns a [ParseError] if any of the source code caused an error.
    pub fn stmt(&mut self) -> Result<Stmt, Diagnostic<FileId>> {
        self.collect_upcoming_comments(); // seeks out tags
        self.active_tag = self.tag_queue.take();
        let result = match self.peek()?.token_type {
            TokenKind::Macro(name, config, body) => self.macro_declaration(name, config, body),
            TokenKind::Enum => self.enum_declaration(),
            TokenKind::Try => self.try_catch(),
            TokenKind::For => self.for_loop(),
            TokenKind::With => self.with(),
            TokenKind::Repeat => self.repeat(),
            TokenKind::Do => self.do_until(),
            TokenKind::While => self.while_loop(),
            TokenKind::If => self.if_stmt(),
            TokenKind::Switch => self.switch(),
            TokenKind::LeftBrace | TokenKind::Begin => self.block(),
            TokenKind::Return => self.return_stmt(),
            TokenKind::Throw => self.throw(),
            TokenKind::Delete => self.delete(),
            TokenKind::Break => self.break_stmt(),
            TokenKind::Continue => self.continue_stmt(),
            TokenKind::Exit => self.exit(),
            TokenKind::Globalvar => self.globalvar_declaration(),
            TokenKind::Var => self.local_variable_series(),
            _ => self.assignment(),
        };
        self.active_tag.take();
        result
    }

    fn macro_declaration(&mut self, name: &str, config: Option<&str>, body: &str) -> Result<Stmt, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        let _token = self.take()?;
        // this is all strange, and is just a sign of a known fact -- our lack of proper support for macros
        // causes weird architecture
        let macro_length = "#macro ".len();
        let name = Identifier::new(name, Span::new(start + macro_length, start + macro_length + name.len()));
        let mac = if let Some(config) = config {
            Macro::new_with_config(name, body, config)
        } else {
            Macro::new(name, body)
        };
        Ok(self.new_stmt(mac, start))
    }

    fn enum_declaration(&mut self) -> Result<Stmt, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        self.require(TokenKind::Enum)?;
        let name = self.require_identifier()?;
        let mut members = vec![];
        self.require_possibilities(&[TokenKind::LeftBrace, TokenKind::Begin])?;
        loop {
            if self
                .match_take_possibilities(&[TokenKind::RightBrace, TokenKind::End])
                .is_some()
            {
                break;
            } else {
                let member_start = self.next_token_boundary();
                let name = self.require_identifier()?;
                let span = name.span;
                let left = self.new_expr(name, span);
                let enum_member = if let Some(equal) = self.match_take(TokenKind::Equal) {
                    let right = self.expr()?;
                    Field::Initialized(self.new_stmt(
                        Assignment::new(left, AssignmentOp::Identity(equal), right),
                        member_start,
                    ))
                } else {
                    Field::Uninitialized(left)
                };
                members.push(enum_member);
                self.match_take(TokenKind::Comma);
            }
        }
        Ok(self.new_stmt(Enum::new_with_members(name, members), start))
    }

    fn try_catch(&mut self) -> Result<Stmt, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        self.require(TokenKind::Try)?;
        let try_body = self.block()?;
        self.require(TokenKind::Catch)?;
        let catch_expr = self.expr()?;
        let catch_body = self.block()?;
        let try_catch = if self.match_take(TokenKind::Finally).is_some() {
            TryCatch::new_with_finally(try_body, catch_expr, catch_body, self.block()?)
        } else {
            TryCatch::new(try_body, catch_expr, catch_body)
        };
        Ok(self.new_stmt(try_catch, start))
    }

    fn for_loop(&mut self) -> Result<Stmt, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        self.require(TokenKind::For)?;
        self.match_take(TokenKind::LeftParenthesis);
        let initializer = self.stmt()?;
        let condition = self.expr()?;
        self.require(TokenKind::SemiColon)?;
        let tick = self.stmt()?;
        self.match_take(TokenKind::RightParenthesis);
        let body = self.stmt()?;
        Ok(self.new_stmt(For::new(initializer, condition, tick, body), start))
    }

    fn with(&mut self) -> Result<Stmt, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        self.require(TokenKind::With)?;
        let condition = self.expr()?;
        let body = self.stmt()?;
        Ok(self.new_stmt(With::new(condition, body), start))
    }

    fn repeat(&mut self) -> Result<Stmt, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        self.require(TokenKind::Repeat)?;
        let condition = self.expr()?;
        let body = self.stmt()?;
        Ok(self.new_stmt(Repeat::new(condition, body), start))
    }

    fn do_until(&mut self) -> Result<Stmt, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        self.require(TokenKind::Do)?;
        let body = self.stmt()?;
        self.require(TokenKind::Until)?;
        let condition = self.expr()?;
        self.match_take_repeating(TokenKind::SemiColon);
        Ok(self.new_stmt(DoUntil::new(body, condition), start))
    }

    fn while_loop(&mut self) -> Result<Stmt, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        self.require(TokenKind::While)?;
        let condition = self.expr()?;
        let body = self.stmt()?;
        Ok(self.new_stmt(If::new(condition, body), start))
    }

    fn if_stmt(&mut self) -> Result<Stmt, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        self.require(TokenKind::If)?;
        let condition = self.expr()?;
        let then = self.match_take(TokenKind::Then);
        let body = self.stmt()?;
        let else_stmt = if self.match_take(TokenKind::Else).is_some() {
            Some(self.stmt()?)
        } else {
            None
        };
        Ok(self.new_stmt(
            If {
                condition,
                body,
                else_stmt,
                uses_then_keyword: then.is_some(),
            },
            start,
        ))
    }

    fn switch(&mut self) -> Result<Stmt, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        fn case_body(parser: &mut Parser) -> Result<Vec<Stmt>, Diagnostic<FileId>> {
            let mut body = vec![];
            loop {
                match parser.peek()?.token_type {
                    TokenKind::Case | TokenKind::Default | TokenKind::RightBrace | TokenKind::End => break,
                    _ => body.push(parser.stmt()?),
                }
            }
            Ok(body)
        }
        self.require(TokenKind::Switch)?;
        let expr = self.expr()?;
        self.require_possibilities(&[TokenKind::LeftBrace, TokenKind::Begin])?;
        let mut members = vec![];
        let mut default = None;
        loop {
            match self.peek()?.token_type {
                TokenKind::Case => {
                    self.take()?;
                    let identity = self.expr()?;
                    self.require(TokenKind::Colon)?;
                    let body = case_body(self)?;
                    members.push(SwitchCase::new(identity, body))
                }
                TokenKind::Default => {
                    self.take()?;
                    self.require(TokenKind::Colon)?;
                    default = Some(case_body(self)?);
                }
                TokenKind::RightBrace | TokenKind::End => {
                    self.take()?;
                    break;
                }
                _ => {
                    let token = self.take()?;
                    return Err(Diagnostic::error()
                        .with_message("Unexpected token in switch statement body")
                        .with_labels(vec![
                            Label::primary(self.file_id, token.span)
                                .with_message("this token is not valid in this position".to_string()),
                        ])
                        .with_notes(vec!["Expected `case`, `default`, `}`, or `end`".into()]));
                }
            }
        }
        Ok(self.new_stmt(Switch::new(expr, members, default), start))
    }

    fn block(&mut self) -> Result<Stmt, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        let opening_delimeter = self.require_possibilities(&[TokenKind::LeftBrace, TokenKind::Begin])?;
        let mut statements: Vec<Stmt> = vec![];
        let closing_delimiter = loop {
            if let Some(token) = self.match_take_possibilities(&[TokenKind::RightBrace, TokenKind::End]) {
                break token;
            } else {
                statements.push(self.stmt()?);
            }
        };
        self.match_take_repeating(TokenKind::SemiColon);
        Ok(self.new_stmt(
            Block::new(statements, Some((opening_delimeter, closing_delimiter))),
            start,
        ))
    }

    fn return_stmt(&mut self) -> Result<Stmt, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        self.require(TokenKind::Return)?;
        let expr = self.expr().ok();
        self.match_take_repeating(TokenKind::SemiColon);
        Ok(self.new_stmt(Return::new(expr), start))
    }

    fn throw(&mut self) -> Result<Stmt, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        self.require(TokenKind::Throw)?;
        let expr = self.expr()?;
        self.match_take_repeating(TokenKind::SemiColon);
        Ok(self.new_stmt(Throw::new(expr), start))
    }

    fn delete(&mut self) -> Result<Stmt, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        self.require(TokenKind::Delete)?;
        let expr = self.expr()?;
        self.match_take_repeating(TokenKind::SemiColon);
        Ok(self.new_stmt(Delete::new(expr), start))
    }

    fn break_stmt(&mut self) -> Result<Stmt, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        self.require(TokenKind::Break)?;
        self.match_take_repeating(TokenKind::SemiColon);
        Ok(self.new_stmt(StmtKind::Break, start))
    }

    fn continue_stmt(&mut self) -> Result<Stmt, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        self.require(TokenKind::Continue)?;
        self.match_take_repeating(TokenKind::SemiColon);
        Ok(self.new_stmt(StmtKind::Continue, start))
    }

    fn exit(&mut self) -> Result<Stmt, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        self.require(TokenKind::Exit)?;
        self.match_take_repeating(TokenKind::SemiColon);
        Ok(self.new_stmt(StmtKind::Exit, start))
    }

    fn globalvar_declaration(&mut self) -> Result<Stmt, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        self.require(TokenKind::Globalvar)?;
        let name = self.require_identifier()?;
        self.match_take_repeating(TokenKind::SemiColon);
        Ok(self.new_stmt(Globalvar::new(name), start))
    }

    fn local_variable_series(&mut self) -> Result<Stmt, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        self.require(TokenKind::Var)?;
        let mut declarations = vec![];
        loop {
            let name = self.require_identifier()?;
            let span = name.span;
            let left = self.new_expr(name, span);
            let local_variable = if let Some(equal) = self.match_take(TokenKind::Equal) {
                let right = self.expr()?;
                Field::Initialized(self.new_stmt(Assignment::new(left, AssignmentOp::Identity(equal), right), start))
            } else {
                Field::Uninitialized(left)
            };
            declarations.push(local_variable);
            if self.match_take(TokenKind::Comma).is_none() {
                break;
            }
            if !matches!(
                self.soft_peek(),
                Some(Token {
                    token_type: TokenKind::Identifier(..),
                    ..
                })
            ) {
                // For some reason, this is valid gml:
                // ```
                // var i = 0,
                // ```
                // Lord have mercy.
                break;
            }
        }
        self.match_take_repeating(TokenKind::SemiColon);
        Ok(self.new_stmt(LocalVariables::new(declarations), start))
    }

    fn assignment(&mut self) -> Result<Stmt, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        let expr = self.unary()?; // Unaries are the highest possibel assignment expressions

        // Check for an identifier followed by an assignment operator
        let assignment = if let Some(operator) = self.soft_peek().and_then(|token| token.as_assignment_op()) {
            self.take()?;
            Assignment::new(expr, operator, self.expr()?)
        } else if let Some(Equality {
            left,
            op:
                EqualityOp::Equal(Token {
                    token_type: TokenKind::Equal,
                    span,
                }),
            right,
        }) = expr.kind().as_equality()
        {
            Assignment::new(
                left.clone(),
                AssignmentOp::Identity(Token::new(TokenKind::Equal, *span)),
                right.clone(),
            )
        } else {
            // We can't make an assignment out of this -- create an expression statement instead.
            return self.expr_stmt(expr);
        };
        self.match_take_repeating(TokenKind::SemiColon);
        Ok(self.new_stmt(assignment, start))
    }

    fn expr_stmt(&mut self, expr: Expr) -> Result<Stmt, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        match expr.kind() {
            ExprKind::Function(..)
            | ExprKind::Postfix(..)
            | ExprKind::Unary(..) // FIXME: only some unary is valid here
            | ExprKind::Grouping(..)
            | ExprKind::Call(..) => {}

            // Unfortunately, we can't (currently) understand if this is
            // actually a mistake or is a macro.
            // In the future, we may unfold code in an early pass that will
            // help with this.
            // FIXME: maybe an allow by default lint for this?
            ExprKind::Identifier(..) => {}

            // Anything else is invalid.
            _ => {
                return Err(Diagnostic::error()
                    .with_message("Incomplete statement")
                    .with_labels(vec![
                        Label::primary(self.file_id, expr.span())
                            .with_message("this expression does not form a complete statement".to_string()),
                    ])
                    .with_notes(vec![format!(
                        "{}: did you mean to assign this value to something?",
                        "hint".bold()
                    )]));
            }
        }
        self.match_take_repeating(TokenKind::SemiColon);
        Ok(self.new_stmt(StmtKind::Expr(expr), start))
    }

    /// Parses the source gml for a new expression.
    ///
    ///  ### Errors
    ///
    /// Returns a [ParseError] if any of the source code caused an error.
    pub fn expr(&mut self) -> Result<Expr, Diagnostic<FileId>> {
        self.null_coalecence()
    }

    fn null_coalecence(&mut self) -> Result<Expr, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        let expr = self.ternary()?;
        if self.match_take(TokenKind::DoubleHook).is_some() {
            let value = self.expr()?;
            let end = value.span().end();
            Ok(self.new_expr(NullCoalecence::new(expr, value), Span::new(start, end)))
        } else {
            Ok(expr)
        }
    }

    fn ternary(&mut self) -> Result<Expr, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        let expr = self.logical()?;
        if self.match_take(TokenKind::Hook).is_some() {
            let true_value = self.expr()?;
            self.require(TokenKind::Colon)?;
            let false_value = self.expr()?;
            let end = false_value.span().end();
            Ok(self.new_expr(Ternary::new(expr, true_value, false_value), Span::new(start, end)))
        } else {
            Ok(expr)
        }
    }

    fn logical(&mut self) -> Result<Expr, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        let expr = self.equality()?;
        if let Some(operator) = self.soft_peek().and_then(|token| token.as_logical_op()) {
            self.take()?;
            let right = self.logical()?;
            let end = right.span().end();
            Ok(self.new_expr(Logical::new(expr, operator, right), Span::new(start, end)))
        } else {
            Ok(expr)
        }
    }

    fn equality(&mut self) -> Result<Expr, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        let expr = self.binary()?;
        if let Some(operator) = self.soft_peek().and_then(|token| token.as_equality_op()) {
            self.take()?;
            let right = self.equality()?;
            let end = right.span().end();
            Ok(self.new_expr(Equality::new(expr, operator, right), Span::new(start, end)))
        } else {
            Ok(expr)
        }
    }

    fn binary(&mut self) -> Result<Expr, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        let expr = self.bitshift()?;
        if let Some(operator) = self
            .soft_peek()
            .map(|token| token.as_evaluation_op())
            .filter(|operator| {
                matches!(
                    operator,
                    Some(EvaluationOp::And(_)) | Some(EvaluationOp::Or(_)) | Some(EvaluationOp::Xor(_))
                )
            })
            .flatten()
        {
            self.take()?;
            let right = self.binary()?;
            let end = right.span().end();
            Ok(self.new_expr(Evaluation::new(expr, operator, right), Span::new(start, end)))
        } else {
            Ok(expr)
        }
    }

    fn bitshift(&mut self) -> Result<Expr, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        let expr = self.addition()?;
        if let Some(operator) = self
            .soft_peek()
            .map(|token| token.as_evaluation_op())
            .filter(|operator| {
                matches!(
                    operator,
                    Some(EvaluationOp::BitShiftLeft(_)) | Some(EvaluationOp::BitShiftRight(_))
                )
            })
            .flatten()
        {
            self.take()?;
            let right = self.bitshift()?;
            let end = right.span().end();
            Ok(self.new_expr(Evaluation::new(expr, operator, right), Span::new(start, end)))
        } else {
            Ok(expr)
        }
    }

    fn addition(&mut self) -> Result<Expr, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        let expr = self.multiplication()?;
        if let Some(operator) = self
            .soft_peek()
            .map(|token| token.as_evaluation_op())
            .filter(|operator| {
                matches!(
                    operator,
                    Some(EvaluationOp::Plus(Token {
                        token_type: TokenKind::Plus,
                        ..
                    })) | Some(EvaluationOp::Minus(Token {
                        token_type: TokenKind::Minus,
                        ..
                    }))
                )
            })
            .flatten()
        {
            self.take()?;
            let right = self.addition()?;
            let end = right.span().end();
            Ok(self.new_expr(Evaluation::new(expr, operator, right), Span::new(start, end)))
        } else {
            Ok(expr)
        }
    }

    fn multiplication(&mut self) -> Result<Expr, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        let expr = self.unary()?;
        if let Some(operator) = self
            .soft_peek()
            .map(|token| token.as_evaluation_op())
            .filter(|operator| {
                matches!(
                    operator,
                    Some(EvaluationOp::Star(_))
                        | Some(EvaluationOp::Slash(_))
                        | Some(EvaluationOp::Div(_))
                        | Some(EvaluationOp::Modulo(_))
                )
            })
            .flatten()
        {
            self.take()?;
            let right = self.multiplication()?;
            let end = right.span().end();
            Ok(self.new_expr(Evaluation::new(expr, operator, right), Span::new(start, end)))
        } else {
            Ok(expr)
        }
    }

    fn unary(&mut self) -> Result<Expr, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        if let Some(operator) = self.peek()?.as_unary_op() {
            self.take()?;
            let right = self.expr()?;
            let end = right.span().end();
            Ok(self.new_expr(Unary::new(operator, right), Span::new(start, end)))
        } else {
            self.postfix()
        }
    }

    fn postfix(&mut self) -> Result<Expr, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        let expr = self.function()?;
        if let Some(operator) = self.soft_peek().and_then(|token| token.as_postfix_op()) {
            let token = self.take()?;
            Ok(self.new_expr(Postfix::new(expr, operator), Span::new(start, token.span.end())))
        } else {
            Ok(expr)
        }
    }

    fn function(&mut self) -> Result<Expr, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        // TODO: when we do static-analysis, this will be used
        let _static_token = self.match_take(TokenKind::Static);
        if self.match_take(TokenKind::Function).is_some() {
            let name = self.match_take_identifier()?;
            self.require(TokenKind::LeftParenthesis)?;
            let mut parameters = vec![];
            let right_parenthesis = loop {
                match self.peek()?.token_type {
                    TokenKind::RightParenthesis => {
                        break self.take()?;
                    }
                    _ => {
                        let parameter_start = self.next_token_boundary();
                        let name = self.require_identifier()?;
                        let end = name.span.end();
                        let name = self.new_expr(name, Span::new(parameter_start, end));
                        if let Some(token) = self.match_take(TokenKind::Equal) {
                            let assignment = Assignment::new(name, AssignmentOp::Identity(token), self.expr()?);
                            parameters.push(Field::Initialized(self.new_stmt(assignment, parameter_start)));
                        } else {
                            parameters.push(Field::Uninitialized(name));
                        };
                        self.match_take(TokenKind::Comma);
                    }
                }
            };
            let inheritance = if self.peek()?.token_type == TokenKind::Colon {
                let colon = self.take()?;
                let name = self.identifier()?;
                Some((colon, self.call(Some(name), false)?))
            } else {
                None
            };
            let constructor = if self.match_take(TokenKind::Constructor).is_some() {
                Some(Constructor {
                    inheritance: inheritance.map(|(_, v)| v),
                })
            } else {
                if let Some((colon, inheritance)) = inheritance {
                    return Err(Diagnostic::error()
                        .with_message("Invalid usage of inheritance")
                        .with_labels(vec![
                            Label::primary(self.file_id, colon.span.start()..inheritance.span().end())
                                .with_message("you are attempting to use constructor inheritance...".to_string()),
                            Label::secondary(self.file_id, Span::new(start, right_parenthesis.span.end()))
                                .with_message("...but this is not a constructor"),
                        ])
                        .with_notes(vec![format!(
                            "{}: did you mean to assign this value to something?",
                            "hint".bold()
                        )]));
                }
                None
            };
            let body = self.block()?;
            let end = body.span().end();
            Ok(self.new_expr(
                Function {
                    name,
                    parameters,
                    constructor,
                    body,
                },
                Span::new(start, end),
            ))
        } else {
            self.literal()
        }
    }

    fn literal(&mut self) -> Result<Expr, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        if let Some(literal) = self.peek()?.to_literal() {
            let token = self.take()?;
            Ok(self.new_expr(literal, Span::new(start, token.span.end())))
        } else if self.match_take(TokenKind::LeftSquareBracket).is_some() {
            let mut elements = vec![];
            loop {
                if let Some(token) = self.match_take(TokenKind::RightSquareBracket) {
                    let literal = Literal::Array(elements);
                    break Ok(self.new_expr(literal, Span::new(start, token.span.end())));
                } else {
                    elements.push(self.expr()?);
                    self.match_take(TokenKind::Comma);
                }
            }
        } else if self
            .match_take_possibilities(&[TokenKind::LeftBrace, TokenKind::Begin])
            .is_some()
        {
            let mut elements = vec![];
            loop {
                if let Some(token) = self.match_take_possibilities(&[TokenKind::RightBrace, TokenKind::End]) {
                    let literal = Literal::Struct(elements);
                    break Ok(self.new_expr(literal, Span::new(start, token.span.end())));
                } else {
                    let name = self.require_identifier()?;
                    let value = if self.match_take(TokenKind::Colon).is_some() {
                        self.expr()?
                    } else {
                        self.new_expr(name.clone(), name.span)
                    };
                    elements.push((name, value));
                    if self.match_take(TokenKind::Comma).is_none() {
                        let token = self.require_possibilities(&[TokenKind::RightBrace, TokenKind::End])?;
                        let literal = Literal::Struct(elements);
                        break Ok(self.new_expr(literal, Span::new(start, token.span.end())));
                    }
                }
            }
        } else {
            self.supreme()
        }
    }

    fn supreme(&mut self) -> Result<Expr, Diagnostic<FileId>> {
        let mut has_new = self.match_take(TokenKind::New);
        let mut expr = Some(self.call(None, has_new.take().is_some())?);
        loop {
            expr = match self.soft_peek() {
                Some(Token {
                    token_type: TokenKind::LeftParenthesis,
                    ..
                }) => Some(self.call(expr, has_new.take().is_some())?),
                Some(Token {
                    token_type: TokenKind::LeftSquareBracket,
                    ..
                }) => Some(self.ds_access(expr)?),
                Some(Token {
                    token_type: TokenKind::Dot,
                    ..
                }) => Some(self.dot_access(expr)?),
                _ => break Ok(expr.unwrap()),
            }
        }
    }

    fn call(&mut self, left: Option<Expr>, uses_new: bool) -> Result<Expr, Diagnostic<FileId>> {
        // If we've been provided a leftside expression, we *must* parse for a call.
        // Otherwise, the call is merely possible.
        let (start, left) = if let Some(left) = left {
            (left.span().0, left)
        } else {
            let start = self.next_token_boundary();
            let dot = self.dot_access(None)?;
            if !matches!(
                self.soft_peek(),
                Some(Token {
                    token_type: TokenKind::LeftParenthesis,
                    ..
                })
            ) {
                return Ok(dot);
            }
            (start, dot)
        };
        self.require(TokenKind::LeftParenthesis)?;
        let mut arguments = vec![];
        let end = if let Some(token) = self.match_take(TokenKind::RightParenthesis) {
            token.span.end()
        } else {
            loop {
                arguments.push(self.expr()?);
                let token = self.take()?;
                match token.token_type {
                    TokenKind::Comma => {
                        if let Some(token) = self.match_take(TokenKind::RightParenthesis) {
                            break token.span.end();
                        }
                    }
                    TokenKind::RightParenthesis => break token.span.end(),
                    _ => {
                        return Err(Diagnostic::error()
                            .with_message("Unexpected token in call arguments")
                            .with_labels(vec![
                                Label::primary(self.file_id, token.span)
                                    .with_message("this token is not valid in this position".to_string()),
                            ])
                            .with_notes(vec!["Expected `,` or `)`".into()]));
                    }
                }
            }
        };
        Ok(self.new_expr(
            Call {
                left,
                arguments,
                uses_new,
            },
            Span::new(start, end),
        ))
    }

    fn dot_access(&mut self, expr: Option<Expr>) -> Result<Expr, Diagnostic<FileId>> {
        let mut start = self.next_token_boundary();
        let (access, end) = if let Some(expr) = expr {
            self.require(TokenKind::Dot)?;
            start = expr.span().0;
            let right = self.require_identifier()?;
            let end = right.span.end();
            (Access::Dot { left: expr, right }, end)
        } else {
            match self.peek()?.token_type {
                TokenKind::Global => {
                    self.take()?;
                    self.require(TokenKind::Dot)?;
                    let right = self.require_identifier()?;
                    let end = right.span.end();
                    (Access::Global { right }, end)
                }
                TokenKind::SelfKeyword => {
                    let token = self.take()?;
                    if self.match_take(TokenKind::Dot).is_some() {
                        let right = self.require_identifier()?;
                        let end = right.span.end();
                        (Access::Identity { right }, end)
                    } else {
                        // Using self as a referencce!
                        return Ok(
                            self.new_expr(Identifier::new("self", token.span), Span::new(start, token.span.end()))
                        );
                    }
                }
                TokenKind::Other => {
                    let token = self.take()?;
                    if self.match_take(TokenKind::Dot).is_some() {
                        let right = self.require_identifier()?;
                        let end = right.span.end();
                        (Access::Other { right }, end)
                    } else {
                        // Using other as a reference!
                        return Ok(
                            self.new_expr(Identifier::new("other", token.span), Span::new(start, token.span.end()))
                        );
                    }
                }
                _ => {
                    let left = self.ds_access(None)?;
                    if self.match_take(TokenKind::Dot).is_some() {
                        let right = self.require_identifier()?;
                        let end = right.span.end();
                        (Access::Dot { left, right }, end)
                    } else {
                        return Ok(left);
                    }
                }
            }
        };
        Ok(self.new_expr(access, Span::new(start, end)))
    }

    fn ds_access(&mut self, left: Option<Expr>) -> Result<Expr, Diagnostic<FileId>> {
        let (start, left) = if let Some(left) = left {
            (left.span().0, left)
        } else {
            let left = self.grouping()?;
            if !matches!(
                self.soft_peek(),
                Some(&Token {
                    token_type: TokenKind::LeftSquareBracket,
                    ..
                })
            ) {
                return Ok(left);
            }
            (self.next_token_boundary(), left)
        };
        self.require(TokenKind::LeftSquareBracket)?;
        let access = match self.peek()?.token_type {
            TokenKind::DollarSign => {
                self.take()?;
                Access::Struct {
                    left,
                    key: self.expr()?,
                }
            }
            TokenKind::Hook => {
                self.take()?;
                Access::Map {
                    left,
                    key: self.expr()?,
                }
            }
            TokenKind::Pipe => {
                self.take()?;
                Access::List {
                    left,
                    index: self.expr()?,
                }
            }
            TokenKind::Hash => {
                self.take()?;
                let index_one = self.expr()?;
                self.require(TokenKind::Comma)?;
                let index_two = self.expr()?;
                Access::Grid {
                    left,
                    index_one,
                    index_two,
                }
            }
            _ => {
                let using_accessor = self.match_take(TokenKind::AtSign).is_some();
                let index_one = self.expr()?;
                let index_two = if self.match_take(TokenKind::Comma).is_some() {
                    Some(self.expr()?)
                } else {
                    None
                };
                Access::Array {
                    left,
                    index_one,
                    index_two,
                    using_accessor,
                }
            }
        };
        let token = self.require(TokenKind::RightSquareBracket)?;
        Ok(self.new_expr(access, Span::new(start, token.span.end())))
    }

    fn grouping(&mut self) -> Result<Expr, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        if let Some(left_token) = self.match_take(TokenKind::LeftParenthesis) {
            let expr = self.expr()?;
            let right_token = self.require(TokenKind::RightParenthesis)?;
            Ok(self.new_expr(
                Grouping::new(expr, (left_token, right_token)),
                Span::new(start, right_token.span.end()),
            ))
        } else {
            self.identifier()
        }
    }

    fn identifier(&mut self) -> Result<Expr, Diagnostic<FileId>> {
        // FIXME: This is our slightly ludicrous and temporary solution to the static keyword -- we just eat
        // it. Until we have static analysis, it means nothing to us!
        self.match_take(TokenKind::Static);
        if let Some(identifier) = self.match_take_identifier()? {
            let span = identifier.span;
            Ok(self.new_expr(identifier, span))
        } else {
            self.unexpected_token()
        }
    }

    fn unexpected_token(&mut self) -> Result<Expr, Diagnostic<FileId>> {
        let token = *self.peek()?;
        Err(Diagnostic::error().with_message("Invalid token").with_labels(vec![
            Label::primary(self.file_id, token.span)
                .with_message("this token is not valid in this position".to_string()),
        ]))
    }
}
// Lexing tools
impl Parser {
    /// Consumes and returns the next token if it is the given type.
    fn match_take(&mut self, token_type: TokenKind) -> Option<Token> {
        match self.peek() {
            Ok(peek) if peek.token_type == token_type => Some(self.take().unwrap()),
            Err(_) => None,
            _ => None,
        }
    }

    /// Consumes and returns the next token if it is within the array of types.
    fn match_take_possibilities(&mut self, token_types: &[TokenKind]) -> Option<Token> {
        if self
            .peek()
            .map(|token| token_types.contains(&token.token_type))
            .unwrap_or(false)
        {
            Some(self.take().unwrap())
        } else {
            None
        }
    }

    /// Continously eats next token if it is the given type.
    fn match_take_repeating(&mut self, token_type: TokenKind) {
        loop {
            match self.peek() {
                Ok(peek) if peek.token_type != token_type => break,
                Err(_) => break,
                _ => {
                    self.take().unwrap();
                }
            }
        }
    }

    /// Returns the next Token, returning an error if there is none, or if it is
    /// not of the required type.
    fn require(&mut self, expected_type: TokenKind) -> Result<Token, Diagnostic<FileId>> {
        let found_token = self.take()?;
        if found_token.token_type == expected_type {
            Ok(found_token)
        } else {
            Err(Diagnostic::error().with_message("Expected token").with_labels(vec![
                Label::primary(self.file_id, found_token.span)
                    .with_message(format!("expected this to be a {:?} token", expected_type)),
            ]))
        }
    }

    /// Returns the next Token, returning an error if there is none, or if it is
    /// not within the provided array of required types.
    fn require_possibilities(&mut self, tokens: &[TokenKind]) -> Result<Token, Diagnostic<FileId>> {
        let found_token = self.take()?;
        if tokens.contains(&found_token.token_type) {
            Ok(found_token)
        } else {
            let expected_types = tokens.iter().map(|v| format!("{v:?}")).join(", ");
            Err(Diagnostic::error().with_message("Expected token").with_labels(vec![
                Label::primary(self.file_id, found_token.span).with_message(format!(
                    "expected this to be a one of the following tokens: {}",
                    expected_types
                )),
            ]))
        }
    }

    /// Returns the next token as an Identifier if it is of TokenKind::Identifier.
    fn require_identifier(&mut self) -> Result<Identifier, Diagnostic<FileId>> {
        let next = self.take()?;
        if let Token {
            token_type: TokenKind::Identifier(v),
            span,
        } = next
        {
            Ok(Identifier::new(v, span))
        } else {
            Err(Diagnostic::error()
                .with_message("Expected identifier")
                .with_labels(vec![
                    Label::primary(self.file_id, next.span).with_message("expected this to be an identifier"),
                ]))
        }
    }

    /// Returns the inner field of the next Token if it is an Identifier.
    fn match_take_identifier(&mut self) -> Result<Option<Identifier>, Diagnostic<FileId>> {
        if matches!(
            self.peek()?,
            Token {
                token_type: TokenKind::Identifier(_),
                ..
            }
        ) {
            Ok(Some(self.require_identifier()?))
        } else {
            Ok(None)
        }
    }

    /// Get the gml tokens's cursor.
    fn next_token_boundary(&mut self) -> usize {
        self.collect_upcoming_comments();
        self.lexer.peek().map_or(self.cursor, |token| token.span.0)
    }

    /// Returns the type of the next Token if there is one. Used for situations
    /// where no tokens remaining would be valid.
    fn soft_peek(&mut self) -> Option<&Token> {
        self.collect_upcoming_comments();
        if let Some(token) = self.lexer.peek() {
            Some(token)
        } else {
            None
        }
    }

    /// Returns the type of the next Token, or returns an error if there is
    /// none.
    fn peek(&mut self) -> Result<&Token, Diagnostic<FileId>> {
        self.collect_upcoming_comments();
        let start = self.next_token_boundary();
        let next = self.lexer.peek();
        if let Some(next) = next {
            Ok(next)
        } else {
            Err(Diagnostic::error()
                .with_message("Unexpected end")
                .with_labels(vec![Label::primary(self.file_id, start..start).with_message(
                    "reached the end of the file in the middle of parsing a statement".to_string(),
                )]))
        }
    }

    /// Returns the next Token, returning an error if there is none.
    fn take(&mut self) -> Result<Token, Diagnostic<FileId>> {
        self.collect_upcoming_comments();
        let start = self.next_token_boundary();
        if let Some(token) = self.lexer.next() {
            self.cursor = token.span.0;
            Ok(token)
        } else {
            Err(Diagnostic::error()
                .with_message("Unexpected end")
                .with_labels(vec![Label::primary(self.file_id, start..start).with_message(
                    "reached the end of the file in the middle of parsing a statement".to_string(),
                )]))
        }
    }

    /// Looks ahead at the next token and collects it if it is a comment (including lint tags).
    fn collect_upcoming_comments(&mut self) {
        loop {
            match self.lexer.peek() {
                Some(Token {
                    token_type: TokenKind::Comment(_),
                    ..
                }) => self.comments.push(self.lexer.next().unwrap()),
                Some(Token {
                    token_type: TokenKind::Tag(label, parameter),
                    ..
                }) => {
                    self.tag_queue = Some(Tag(label.to_string(), parameter.map(|v| v.to_string())));
                    self.lexer.next().unwrap();
                }
                _ => break,
            }
        }
    }
}

/// A start and end cursor measured in characters, used for expressing small sections of source
/// code.
#[derive(Debug, PartialEq, Default, Copy, Clone, serde::Serialize)]
pub struct Span(usize, usize);
impl Span {
    /// Creates a new span.
    #[cfg(not(test))]
    pub fn new(start: usize, end: usize) -> Self {
        Self(start, end)
    }

    #[cfg(test)]
    pub fn new(_: usize, _: usize) -> Self {
        Self(0, 0)
    }

    /// Returns the start of the span.
    pub fn start(&self) -> usize {
        self.0
    }

    /// Returns the end of the span.
    pub fn end(&self) -> usize {
        self.1
    }
}
impl From<Span> for Range<usize> {
    fn from(span: Span) -> Self {
        span.0..span.1
    }
}

/// A location for something in gml, combining a span and a file id.
#[derive(Debug, PartialEq, Default, Copy, Clone, serde::Serialize)]
pub struct Location(pub FileId, pub Span);
