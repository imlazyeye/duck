use codespan_reporting::diagnostic::{Diagnostic, Label};
use colored::Colorize;
use itertools::Itertools;

use crate::{
    lint::{LintLevel, LintTag},
    parse::*,
    FileId,
};
use std::{iter::Peekable, ops::Range};

/// Recursively decsends Gml source, incremently returning various statements
/// and expressions.
pub struct Parser {
    lexer: Peekable<Lexer>,
    cursor: usize,
    file_id: FileId,
    comments: Vec<Token>,
    lint_tag_slot: Option<LintTag>,
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
            lint_tag_slot: None,
        }
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
            statements.push(self.statement()?);
        }
        Ok(Ast::new(statements))
    }

    /// Wraps an expression in a box.
    pub fn box_expression(&mut self, expression: impl IntoExpressionBox, span: Span) -> ExpressionBox {
        expression.into_expression_box(span, self.file_id, self.lint_tag_slot.as_ref().cloned())
    }

    /// Wraps an expression in a box.
    pub fn box_statement(&mut self, statement: impl IntoStatementBox, start_position: usize) -> StatementBox {
        statement.into_statement_box(self.span(start_position), self.file_id, self.lint_tag_slot.take())
    }

    /// Creates a [Span] from the given position up until the pilot's current
    /// position.
    fn span(&self, start: usize) -> Span {
        Span::new(start, self.cursor + 1)
    }
}

// Recursive descent (gml grammar)
impl Parser {
    pub(super) fn statement(&mut self) -> Result<StatementBox, Diagnostic<FileId>> {
        match self.peek()?.token_type {
            TokenType::Macro(name, config, body) => self.macro_declaration(name, config, body),
            TokenType::Enum => self.enum_declaration(),
            TokenType::Try => self.try_catch(),
            TokenType::For => self.for_loop(),
            TokenType::With => self.with(),
            TokenType::Repeat => self.repeat(),
            TokenType::Do => self.do_until(),
            TokenType::While => self.while_loop(),
            TokenType::If => self.if_statement(),
            TokenType::Switch => self.switch(),
            TokenType::LeftBrace | TokenType::Begin => self.block(),
            TokenType::Return => self.return_statement(),
            TokenType::Throw => self.throw(),
            TokenType::Delete => self.delete(),
            TokenType::Break => self.break_statement(),
            TokenType::Continue => self.continue_statement(),
            TokenType::Exit => self.exit(),
            TokenType::Globalvar => self.globalvar_declaration(),
            TokenType::Var => self.local_variable_series(),
            _ => self.assignment(),
        }
    }

    fn macro_declaration(
        &mut self,
        name: &str,
        config: Option<&str>,
        body: &str,
    ) -> Result<StatementBox, Diagnostic<FileId>> {
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
        Ok(self.box_statement(mac, start))
    }

    fn enum_declaration(&mut self) -> Result<StatementBox, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        self.require(TokenType::Enum)?;
        let name = self.require_identifier()?;
        let mut members = vec![];
        self.require_possibilities(&[TokenType::LeftBrace, TokenType::Begin])?;
        loop {
            if self
                .match_take_possibilities(&[TokenType::RightBrace, TokenType::End])
                .is_some()
            {
                break;
            } else {
                let member_start = self.next_token_boundary();
                let name = self.require_identifier()?;
                let span = name.span;
                let left = self.box_expression(name, span);
                let enum_member = if let Some(equal) = self.match_take(TokenType::Equal) {
                    let right = self.expression()?;
                    OptionalInitilization::Initialized(self.box_statement(
                        Assignment::new(left, AssignmentOperator::Equal(equal), right),
                        member_start,
                    ))
                } else {
                    OptionalInitilization::Uninitialized(left)
                };
                members.push(enum_member);
                self.match_take(TokenType::Comma);
            }
        }
        // GM accepts semicolons here, and as such, so do we.
        // FIXME: create an infastrucutre such that we can lint this?
        self.match_take_repeating(TokenType::SemiColon);
        Ok(self.box_statement(Enum::new_with_members(name, members), start))
    }

    fn try_catch(&mut self) -> Result<StatementBox, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        self.require(TokenType::Try)?;
        let try_body = self.block()?;
        self.require(TokenType::Catch)?;
        let catch_expr = self.expression()?;
        let catch_body = self.block()?;
        let try_catch = if self.match_take(TokenType::Finally).is_some() {
            TryCatch::new_with_finally(try_body, catch_expr, catch_body, self.block()?)
        } else {
            TryCatch::new(try_body, catch_expr, catch_body)
        };
        Ok(self.box_statement(try_catch, start))
    }

    fn for_loop(&mut self) -> Result<StatementBox, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        self.require(TokenType::For)?;
        self.match_take(TokenType::LeftParenthesis);
        let initializer = self.statement()?;
        let condition = self.expression()?;
        self.match_take_repeating(TokenType::SemiColon);
        let tick = self.statement()?;
        self.match_take(TokenType::RightParenthesis);
        let body = self.statement()?;
        Ok(self.box_statement(ForLoop::new(initializer, condition, tick, body), start))
    }

    fn with(&mut self) -> Result<StatementBox, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        self.require(TokenType::With)?;
        let condition = self.expression()?;
        let body = self.statement()?;
        Ok(self.box_statement(WithLoop::new(condition, body), start))
    }

    fn repeat(&mut self) -> Result<StatementBox, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        self.require(TokenType::Repeat)?;
        let condition = self.expression()?;
        let body = self.statement()?;
        Ok(self.box_statement(RepeatLoop::new(condition, body), start))
    }

    fn do_until(&mut self) -> Result<StatementBox, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        self.require(TokenType::Do)?;
        let body = self.statement()?;
        self.require(TokenType::Until)?;
        let condition = self.expression()?;
        self.match_take_repeating(TokenType::SemiColon);
        Ok(self.box_statement(DoUntil::new(body, condition), start))
    }

    fn while_loop(&mut self) -> Result<StatementBox, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        self.require(TokenType::While)?;
        let condition = self.expression()?;
        let body = self.statement()?;
        Ok(self.box_statement(If::new(condition, body), start))
    }

    fn if_statement(&mut self) -> Result<StatementBox, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        self.require(TokenType::If)?;
        let condition = self.expression()?;
        let then = self.match_take(TokenType::Then);
        let body = self.statement()?;
        let else_statement = if self.match_take(TokenType::Else).is_some() {
            Some(self.statement()?)
        } else {
            None
        };
        Ok(self.box_statement(
            If {
                condition,
                body,
                else_statement,
                uses_then_keyword: then.is_some(),
            },
            start,
        ))
    }

    fn switch(&mut self) -> Result<StatementBox, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        fn case_body(parser: &mut Parser) -> Result<Vec<StatementBox>, Diagnostic<FileId>> {
            let mut body = vec![];
            loop {
                match parser.peek()?.token_type {
                    TokenType::Case | TokenType::Default | TokenType::RightBrace | TokenType::End => break,
                    _ => body.push(parser.statement()?),
                }
            }
            Ok(body)
        }
        self.require(TokenType::Switch)?;
        let expression = self.expression()?;
        self.require_possibilities(&[TokenType::LeftBrace, TokenType::Begin])?;
        let mut members = vec![];
        let mut default = None;
        loop {
            match self.peek()?.token_type {
                TokenType::Case => {
                    self.take()?;
                    let identity = self.expression()?;
                    self.require(TokenType::Colon)?;
                    let body = case_body(self)?;
                    members.push(SwitchCase::new(identity, body))
                }
                TokenType::Default => {
                    self.take()?;
                    self.require(TokenType::Colon)?;
                    default = Some(case_body(self)?);
                }
                TokenType::RightBrace | TokenType::End => {
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
        Ok(self.box_statement(Switch::new(expression, members, default), start))
    }

    fn block(&mut self) -> Result<StatementBox, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        let opening_delimeter = self.require_possibilities(&[TokenType::LeftBrace, TokenType::Begin])?;
        let mut statements: Vec<StatementBox> = vec![];
        let closing_delimiter = loop {
            if let Some(token) = self.match_take_possibilities(&[TokenType::RightBrace, TokenType::End]) {
                break token;
            } else {
                statements.push(self.statement()?);
            }
        };
        self.match_take_repeating(TokenType::SemiColon);
        Ok(self.box_statement(
            Block::new(statements, Some((opening_delimeter, closing_delimiter))),
            start,
        ))
    }

    fn return_statement(&mut self) -> Result<StatementBox, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        self.require(TokenType::Return)?;
        let expression = self.expression().ok();
        self.match_take_repeating(TokenType::SemiColon);
        Ok(self.box_statement(Return::new(expression), start))
    }

    fn throw(&mut self) -> Result<StatementBox, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        self.require(TokenType::Throw)?;
        let expression = self.expression()?;
        self.match_take_repeating(TokenType::SemiColon);
        Ok(self.box_statement(Throw::new(expression), start))
    }

    fn delete(&mut self) -> Result<StatementBox, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        self.require(TokenType::Delete)?;
        let expression = self.expression()?;
        self.match_take_repeating(TokenType::SemiColon);
        Ok(self.box_statement(Delete::new(expression), start))
    }

    fn break_statement(&mut self) -> Result<StatementBox, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        self.require(TokenType::Break)?;
        self.match_take_repeating(TokenType::SemiColon);
        Ok(self.box_statement(Statement::Break, start))
    }

    fn continue_statement(&mut self) -> Result<StatementBox, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        self.require(TokenType::Continue)?;
        self.match_take_repeating(TokenType::SemiColon);
        Ok(self.box_statement(Statement::Continue, start))
    }

    fn exit(&mut self) -> Result<StatementBox, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        self.require(TokenType::Exit)?;
        self.match_take_repeating(TokenType::SemiColon);
        Ok(self.box_statement(Statement::Exit, start))
    }

    fn globalvar_declaration(&mut self) -> Result<StatementBox, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        self.require(TokenType::Globalvar)?;
        let name = self.require_identifier()?;
        self.match_take_repeating(TokenType::SemiColon);
        Ok(self.box_statement(Globalvar::new(name), start))
    }

    fn local_variable_series(&mut self) -> Result<StatementBox, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        self.require(TokenType::Var)?;
        let mut declarations = vec![];
        loop {
            let name = self.require_identifier()?;
            let span = name.span;
            let left = self.box_expression(name, span);
            let local_variable = if let Some(equal) = self.match_take(TokenType::Equal) {
                let right = self.expression()?;
                OptionalInitilization::Initialized(
                    self.box_statement(Assignment::new(left, AssignmentOperator::Equal(equal), right), start),
                )
            } else {
                OptionalInitilization::Uninitialized(left)
            };
            declarations.push(local_variable);
            if self.match_take(TokenType::Comma).is_none() {
                break;
            }
            if !matches!(
                self.soft_peek(),
                Some(Token {
                    token_type: TokenType::Identifier(..),
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
        self.match_take_repeating(TokenType::SemiColon);
        Ok(self.box_statement(LocalVariableSeries::new(declarations), start))
    }

    fn assignment(&mut self) -> Result<StatementBox, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        let expression = self.unary()?; // Unaries are the highest possibel assignment expressions

        // Check for an identifier followed by an assignment operator
        let assignment = if let Some(operator) = self.soft_peek().and_then(|token| token.as_assignment_operator()) {
            self.take()?;
            Assignment::new(expression, operator, self.expression()?)
        } else if let Expression::Equality(Equality {
            left,
            operator:
                EqualityOperator::Equal(Token {
                    token_type: TokenType::Equal,
                    span,
                }),
            right,
        }) = *expression.expression
        {
            Assignment::new(
                left,
                AssignmentOperator::Equal(Token::new(TokenType::Equal, span)),
                right,
            )
        } else {
            // We can't make an assignment out of this -- create an expression statement instead.
            return self.expression_statement(expression);
        };
        self.match_take_repeating(TokenType::SemiColon);
        Ok(self.box_statement(assignment, start))
    }

    fn expression_statement(&mut self, expression: ExpressionBox) -> Result<StatementBox, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        match expression.expression() {
            Expression::FunctionDeclaration(..)
            | Expression::Postfix(..)
            | Expression::Unary(..)
            | Expression::Grouping(..)
            | Expression::Call(..) => {}

            // Unfortunately, we can't (currently) understand if this is
            // actually a mistake or is a macro.
            // In the future, we may unfold code in an early pass that will
            // help with this.
            Expression::Identifier(..) => {}

            // Anything else is invalid.
            _ => {
                return Err(Diagnostic::error()
                    .with_message("Incomplete statement")
                    .with_labels(vec![
                        Label::primary(self.file_id, expression.span())
                            .with_message("this expression does not form a complete statement".to_string()),
                    ])
                    .with_notes(vec![format!(
                        "{}: did you mean to assign this value to something?",
                        "hint".bold()
                    )]));
            }
        }
        self.match_take_repeating(TokenType::SemiColon);
        Ok(self.box_statement(Statement::Expression(expression), start))
    }

    pub(super) fn expression(&mut self) -> Result<ExpressionBox, Diagnostic<FileId>> {
        self.null_coalecence()
    }

    fn null_coalecence(&mut self) -> Result<ExpressionBox, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        let expression = self.ternary()?;
        if self.match_take(TokenType::DoubleInterrobang).is_some() {
            let value = self.expression()?;
            let end = value.span().end();
            Ok(self.box_expression(NullCoalecence::new(expression, value), Span::new(start, end)))
        } else {
            Ok(expression)
        }
    }

    fn ternary(&mut self) -> Result<ExpressionBox, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        let expression = self.logical()?;
        if self.match_take(TokenType::Interrobang).is_some() {
            let true_value = self.expression()?;
            self.require(TokenType::Colon)?;
            let false_value = self.expression()?;
            let end = false_value.span().end();
            Ok(self.box_expression(Ternary::new(expression, true_value, false_value), Span::new(start, end)))
        } else {
            Ok(expression)
        }
    }

    fn logical(&mut self) -> Result<ExpressionBox, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        let expression = self.equality()?;
        if let Some(operator) = self.soft_peek().and_then(|token| token.as_logical_operator()) {
            self.take()?;
            let right = self.logical()?;
            let end = right.span().end();
            Ok(self.box_expression(Logical::new(expression, operator, right), Span::new(start, end)))
        } else {
            Ok(expression)
        }
    }

    fn equality(&mut self) -> Result<ExpressionBox, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        let expression = self.binary()?;
        if let Some(operator) = self.soft_peek().and_then(|token| token.as_equality_operator()) {
            self.take()?;
            let right = self.equality()?;
            let end = right.span().end();
            Ok(self.box_expression(Equality::new(expression, operator, right), Span::new(start, end)))
        } else {
            Ok(expression)
        }
    }

    fn binary(&mut self) -> Result<ExpressionBox, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        let expression = self.bitshift()?;
        if let Some(operator) = self
            .soft_peek()
            .map(|token| token.as_evaluation_operator())
            .filter(|operator| {
                matches!(
                    operator,
                    Some(EvaluationOperator::And(_))
                        | Some(EvaluationOperator::Or(_))
                        | Some(EvaluationOperator::Xor(_))
                )
            })
            .flatten()
        {
            self.take()?;
            let right = self.binary()?;
            let end = right.span().end();
            Ok(self.box_expression(Evaluation::new(expression, operator, right), Span::new(start, end)))
        } else {
            Ok(expression)
        }
    }

    fn bitshift(&mut self) -> Result<ExpressionBox, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        let expression = self.addition()?;
        if let Some(operator) = self
            .soft_peek()
            .map(|token| token.as_evaluation_operator())
            .filter(|operator| {
                matches!(
                    operator,
                    Some(EvaluationOperator::BitShiftLeft(_)) | Some(EvaluationOperator::BitShiftRight(_))
                )
            })
            .flatten()
        {
            self.take()?;
            let right = self.bitshift()?;
            let end = right.span().end();
            Ok(self.box_expression(Evaluation::new(expression, operator, right), Span::new(start, end)))
        } else {
            Ok(expression)
        }
    }

    fn addition(&mut self) -> Result<ExpressionBox, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        let expression = self.multiplication()?;
        if let Some(operator) = self
            .soft_peek()
            .map(|token| token.as_evaluation_operator())
            .filter(|operator| {
                matches!(
                    operator,
                    Some(EvaluationOperator::Plus(Token {
                        token_type: TokenType::Plus,
                        ..
                    })) | Some(EvaluationOperator::Minus(Token {
                        token_type: TokenType::Minus,
                        ..
                    }))
                )
            })
            .flatten()
        {
            self.take()?;
            let right = self.addition()?;
            let end = right.span().end();
            Ok(self.box_expression(Evaluation::new(expression, operator, right), Span::new(start, end)))
        } else {
            Ok(expression)
        }
    }

    fn multiplication(&mut self) -> Result<ExpressionBox, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        let expression = self.unary()?;
        if let Some(operator) = self
            .soft_peek()
            .map(|token| token.as_evaluation_operator())
            .filter(|operator| {
                matches!(
                    operator,
                    Some(EvaluationOperator::Star(_))
                        | Some(EvaluationOperator::Slash(_))
                        | Some(EvaluationOperator::Div(_))
                        | Some(EvaluationOperator::Modulo(_))
                )
            })
            .flatten()
        {
            self.take()?;
            let right = self.multiplication()?;
            let end = right.span().end();
            Ok(self.box_expression(Evaluation::new(expression, operator, right), Span::new(start, end)))
        } else {
            Ok(expression)
        }
    }

    fn unary(&mut self) -> Result<ExpressionBox, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        if let Some(operator) = self.peek()?.as_unary_operator() {
            self.take()?;
            let right = self.expression()?;
            let end = right.span().end();
            Ok(self.box_expression(Unary::new(operator, right), Span::new(start, end)))
        } else {
            self.postfix()
        }
    }

    fn postfix(&mut self) -> Result<ExpressionBox, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        let expression = self.function()?;
        if let Some(operator) = self.soft_peek().and_then(|token| token.as_postfix_operator()) {
            let token = self.take()?;
            Ok(self.box_expression(Postfix::new(expression, operator), Span::new(start, token.span.end())))
        } else {
            Ok(expression)
        }
    }

    fn function(&mut self) -> Result<ExpressionBox, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        // TODO: when we do static-analysis, this will be used
        let _static_token = self.match_take(TokenType::Static);
        if self.match_take(TokenType::Function).is_some() {
            let name = self.match_take_identifier()?;
            self.require(TokenType::LeftParenthesis)?;
            let mut parameters = vec![];
            let right_parenthesis = loop {
                match self.peek()?.token_type {
                    TokenType::RightParenthesis => {
                        break self.take()?;
                    }
                    _ => {
                        let parameter_start = self.next_token_boundary();
                        let name = self.require_identifier()?;
                        let end = name.span.end();
                        let name = self.box_expression(name, Span::new(parameter_start, end));
                        if let Some(token) = self.match_take(TokenType::Equal) {
                            let assignment =
                                Assignment::new(name, AssignmentOperator::Equal(token), self.expression()?);
                            parameters.push(OptionalInitilization::Initialized(
                                self.box_statement(assignment, parameter_start),
                            ));
                        } else {
                            parameters.push(OptionalInitilization::Uninitialized(name));
                        };
                        self.match_take(TokenType::Comma);
                    }
                }
            };
            let inheritance = if self.peek()?.token_type == TokenType::Colon {
                let colon = self.take()?;
                let name = self.identifier()?;
                Some((colon, self.call(Some(name), false)?))
            } else {
                None
            };
            let constructor = if self.match_take(TokenType::Constructor).is_some() {
                match inheritance {
                    Some((_, inheritance)) => Some(Constructor::WithInheritance(inheritance)),
                    None => Some(Constructor::WithoutInheritance),
                }
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
            Ok(self.box_expression(
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

    fn literal(&mut self) -> Result<ExpressionBox, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        if let Some(literal) = self.peek()?.to_literal() {
            let token = self.take()?;
            Ok(self.box_expression(literal, Span::new(start, token.span.end())))
        } else if self.match_take(TokenType::LeftSquareBracket).is_some() {
            let mut elements = vec![];
            loop {
                if let Some(token) = self.match_take(TokenType::RightSquareBracket) {
                    break Ok(self.box_expression(Literal::Array(elements), Span::new(start, token.span.end())));
                } else {
                    elements.push(self.expression()?);
                    self.match_take(TokenType::Comma);
                }
            }
        } else if self
            .match_take_possibilities(&[TokenType::LeftBrace, TokenType::Begin])
            .is_some()
        {
            let mut elements = vec![];
            loop {
                if let Some(token) = self.match_take_possibilities(&[TokenType::RightBrace, TokenType::End]) {
                    break Ok(self.box_expression(Literal::Struct(elements), Span::new(start, token.span.end())));
                } else {
                    let name = self.require_identifier()?;
                    self.require(TokenType::Colon)?;
                    elements.push((name, self.expression()?));
                    self.match_take(TokenType::Comma);
                }
            }
        } else {
            self.supreme()
        }
    }

    fn supreme(&mut self) -> Result<ExpressionBox, Diagnostic<FileId>> {
        let mut has_new = self.match_take(TokenType::New);
        let mut expression = Some(self.call(None, has_new.take().is_some())?);
        loop {
            expression = match self.soft_peek() {
                Some(Token {
                    token_type: TokenType::LeftParenthesis,
                    ..
                }) => Some(self.call(expression, has_new.take().is_some())?),
                Some(Token {
                    token_type: TokenType::LeftSquareBracket,
                    ..
                }) => Some(self.ds_access(expression)?),
                Some(Token {
                    token_type: TokenType::Dot,
                    ..
                }) => Some(self.dot_access(expression)?),
                _ => break Ok(expression.unwrap()),
            }
        }
    }

    fn call(&mut self, left: Option<ExpressionBox>, uses_new: bool) -> Result<ExpressionBox, Diagnostic<FileId>> {
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
                    token_type: TokenType::LeftParenthesis,
                    ..
                })
            ) {
                return Ok(dot);
            }
            (start, dot)
        };
        self.require(TokenType::LeftParenthesis)?;
        let mut arguments = vec![];
        let end = if let Some(token) = self.match_take(TokenType::RightParenthesis) {
            token.span.end()
        } else {
            loop {
                arguments.push(self.expression()?);
                let token = self.take()?;
                match token.token_type {
                    TokenType::Comma => {
                        if let Some(token) = self.match_take(TokenType::RightParenthesis) {
                            break token.span.end();
                        }
                    }
                    TokenType::RightParenthesis => break token.span.end(),
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
        Ok(self.box_expression(
            Call {
                left,
                arguments,
                uses_new,
            },
            Span::new(start, end),
        ))
    }

    fn dot_access(&mut self, expression: Option<ExpressionBox>) -> Result<ExpressionBox, Diagnostic<FileId>> {
        let mut start = self.next_token_boundary();
        let (access, end) = if let Some(expression) = expression {
            self.require(TokenType::Dot)?;
            start = expression.span().0;
            let right = self.grouping()?;
            let end = right.span().end();
            (
                Access::Dot {
                    left: expression,
                    right,
                },
                end,
            )
        } else {
            match self.peek()?.token_type {
                TokenType::Global => {
                    self.take()?;
                    self.require(TokenType::Dot)?;
                    let right = self.grouping()?;
                    let end = right.span().end();
                    (Access::Global { right }, end)
                }
                TokenType::SelfKeyword => {
                    let token = self.take()?;
                    if self.match_take(TokenType::Dot).is_some() {
                        let right = self.grouping()?;
                        let end = right.span().end();
                        (Access::Current { right }, end)
                    } else {
                        // Using self as a referencce!
                        return Ok(self
                            .box_expression(Identifier::new("self", token.span), Span::new(start, token.span.end())));
                    }
                }
                TokenType::Other => {
                    let token = self.take()?;
                    if self.match_take(TokenType::Dot).is_some() {
                        let right = self.grouping()?;
                        let end = right.span().end();
                        (Access::Other { right }, end)
                    } else {
                        // Using other as a reference!
                        return Ok(self
                            .box_expression(Identifier::new("other", token.span), Span::new(start, token.span.end())));
                    }
                }
                _ => {
                    let left = self.ds_access(None)?;
                    if self.match_take(TokenType::Dot).is_some() {
                        let right = self.grouping()?;
                        let end = right.span().end();
                        (Access::Dot { left, right }, end)
                    } else {
                        return Ok(left);
                    }
                }
            }
        };
        Ok(self.box_expression(access, Span::new(start, end)))
    }

    fn ds_access(&mut self, left: Option<ExpressionBox>) -> Result<ExpressionBox, Diagnostic<FileId>> {
        let (start, left) = if let Some(left) = left {
            (left.span().0, left)
        } else {
            let left = self.grouping()?;
            if !matches!(
                self.soft_peek(),
                Some(&Token {
                    token_type: TokenType::LeftSquareBracket,
                    ..
                })
            ) {
                return Ok(left);
            }
            (self.next_token_boundary(), left)
        };
        self.require(TokenType::LeftSquareBracket)?;
        let access = match self.peek()?.token_type {
            TokenType::DollarSign => {
                self.take()?;
                Access::Struct {
                    left,
                    key: self.expression()?,
                }
            }
            TokenType::Interrobang => {
                self.take()?;
                Access::Map {
                    left,
                    key: self.expression()?,
                }
            }
            TokenType::Pipe => {
                self.take()?;
                Access::List {
                    left,
                    index: self.expression()?,
                }
            }
            TokenType::Hash => {
                self.take()?;
                let index_one = self.expression()?;
                self.require(TokenType::Comma)?;
                let index_two = self.expression()?;
                Access::Grid {
                    left,
                    index_one,
                    index_two,
                }
            }
            _ => {
                let using_accessor = self.match_take(TokenType::AtSign).is_some();
                let index_one = self.expression()?;
                let index_two = if self.match_take(TokenType::Comma).is_some() {
                    Some(self.expression()?)
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
        let token = self.require(TokenType::RightSquareBracket)?;
        Ok(self.box_expression(access, Span::new(start, token.span.end())))
    }

    fn grouping(&mut self) -> Result<ExpressionBox, Diagnostic<FileId>> {
        let start = self.next_token_boundary();
        if let Some(left_token) = self.match_take(TokenType::LeftParenthesis) {
            let expression = self.expression()?;
            let right_token = self.require(TokenType::RightParenthesis)?;
            Ok(self.box_expression(
                Grouping::new(expression, (left_token, right_token)),
                Span::new(start, right_token.span.end()),
            ))
        } else {
            self.identifier()
        }
    }

    fn identifier(&mut self) -> Result<ExpressionBox, Diagnostic<FileId>> {
        // FIXME: This is our slightly ludicrous and temporary solution to the static keyword -- we just eat
        // it. Until we have static analysis, it means nothing to us!
        self.match_take(TokenType::Static);
        if let Some(identifier) = self.match_take_identifier()? {
            let span = identifier.span;
            Ok(self.box_expression(identifier, span))
        } else {
            self.unexpected_token()
        }
    }

    fn unexpected_token(&mut self) -> Result<ExpressionBox, Diagnostic<FileId>> {
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
    fn match_take(&mut self, token_type: TokenType) -> Option<Token> {
        match self.peek() {
            Ok(peek) if peek.token_type == token_type => Some(self.take().unwrap()),
            Err(_) => None,
            _ => None,
        }
    }

    /// Consumes and returns the next token if it is within the array of types.
    fn match_take_possibilities(&mut self, token_types: &[TokenType]) -> Option<Token> {
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
    fn match_take_repeating(&mut self, token_type: TokenType) {
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
    fn require(&mut self, expected_type: TokenType) -> Result<Token, Diagnostic<FileId>> {
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
    fn require_possibilities(&mut self, tokens: &[TokenType]) -> Result<Token, Diagnostic<FileId>> {
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

    /// Returns the next token as an Identifier if it is of TokenType::Identifier.
    fn require_identifier(&mut self) -> Result<Identifier, Diagnostic<FileId>> {
        let next = self.take()?;
        if let Token {
            token_type: TokenType::Identifier(v),
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
                token_type: TokenType::Identifier(_),
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
                    token_type: TokenType::Comment(_),
                    ..
                }) => self.comments.push(self.lexer.next().unwrap()),
                Some(Token {
                    token_type: TokenType::LintTag(level, tag),
                    ..
                }) => {
                    // TODO: We currently don't have an easy way to validate a lint tag. We should create an
                    // autogenerated lazy-static hashset.
                    let lint_level = match *level {
                        "allow" => LintLevel::Allow,
                        "warn" => LintLevel::Warn,
                        "deny" => LintLevel::Deny,
                        _ => {
                            // todo: returning the below error would be non trivial...
                            // return Err(ParseError::InvalidLintLevel(self.lexer.next().unwrap())),
                            self.lexer.next().unwrap();
                            break;
                        }
                    };
                    self.lint_tag_slot = Some(LintTag(tag.to_string(), lint_level));
                    self.lexer.next().unwrap();
                }
                _ => break,
            }
        }
    }
}

/// A start and end cursor measured in characters, used for expressing small sections of source
/// code.
#[derive(Debug, PartialEq, Default, Copy, Clone)]
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
#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct Location(pub FileId, pub Span);
