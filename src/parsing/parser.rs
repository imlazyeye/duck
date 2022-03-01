use std::path::PathBuf;

use crate::{
    gml::{
        Assignment, AssignmentOperator, DoUntil, Enum, ForLoop, Globalvar, Identifier, If, LocalVariable,
        LocalVariableSeries, Macro, RepeatLoop, Switch, SwitchCase, TryCatch, WithLoop,
    },
    parsing::{expression::EvaluationOperator, ParseError},
    utils::Span,
};

use super::{
    expression::{Constructor, Expression, ExpressionBox, Literal, Parameter, Scope},
    statement::{Statement, StatementBox},
    token_pilot::TokenPilot,
    IntoExpressionBox, IntoStatementBox, Token,
};

/// A collection of statements.
pub type Ast = Vec<StatementBox>;

/// Recursively decsends Gml source, incremently returning various statements
/// and expressions.
pub struct Parser<'a> {
    pilot: TokenPilot<'a>,

    // rust analyzer mishaps below
    #[allow(dead_code)]
    source_code: &'a str,
    #[allow(dead_code)]
    resource_path: PathBuf,
}

impl<'a> Parser<'a> {
    /// Creates a new parser.
    pub fn new(source_code: &'a str, resource_path: PathBuf) -> Self {
        Self {
            pilot: TokenPilot::new(source_code),
            resource_path,
            source_code,
        }
    }

    /// Runs the parser through the entire source, collecting everything into an
    /// Ast and returning it.
    pub fn into_ast(mut self) -> Result<Ast, ParseError> {
        let mut statements: Ast = vec![];
        while self.pilot.soft_peek().is_some() {
            statements.push(self.statement()?);
        }
        Ok(statements)
    }

    /// Creates a [Span] from the given position up until the pilot's current
    /// position.
    #[cfg(not(test))]
    pub fn span(&mut self, start: usize) -> Span {
        Span(start, self.pilot.cursor())
    }

    #[cfg(test)]
    pub fn span(&self, _start: usize) -> Span {
        Span::default()
    }

    pub(super) fn statement(&mut self) -> Result<StatementBox, ParseError> {
        match self.pilot.peek()? {
            Token::Macro(_, _, _) => self.macro_declaration(),
            Token::GmlEnum => self.enum_declaration(),
            Token::Try => self.try_catch(),
            Token::For => self.for_loop(),
            Token::With => self.with(),
            Token::Repeat => self.repeat(),
            Token::Do => self.do_until(),
            Token::While => self.while_loop(),
            Token::If => self.if_statement(),
            Token::Switch => self.switch(),
            Token::LeftBrace => self.block(),
            Token::Return => self.return_statement(),
            Token::Break => self.break_statement(),
            Token::Continue => self.continue_statement(),
            Token::Exit => self.exit(),
            Token::Globalvar => self.globalvar_declaration(),
            Token::Var => self.local_variable_series(),
            _ => self.expression_statement(),
        }
    }

    fn macro_declaration(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.pilot.cursor();
        match self.pilot.take()? {
            Token::Macro(name, config, body) => {
                let mac = if let Some(config) = config {
                    Macro::new_with_config(name, body, config)
                } else {
                    Macro::new(name, body)
                };
                Ok(mac.into_statement_box(self.span(start)))
            }
            token => Err(ParseError::UnexpectedToken(self.span(start), token)),
        }
    }

    fn enum_declaration(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.pilot.cursor();
        self.pilot.require(Token::GmlEnum)?;
        let name = self.pilot.require_identifier()?;
        let mut gml_enum = Enum::new(name);
        self.pilot.require(Token::LeftBrace)?;
        loop {
            if self.pilot.match_take(Token::RightBrace).is_some() {
                break;
            } else {
                let name = self.pilot.require_identifier()?;
                let initializer = if self.pilot.match_take(Token::Equal).is_some() {
                    Some(self.expression()?)
                } else {
                    None
                };
                gml_enum.register_member(name, initializer);
                self.pilot.match_take(Token::Comma);
            }
        }
        self.pilot.match_take_repeating(Token::SemiColon); // todo: maybe lint this in the future
        Ok(Statement::EnumDeclaration(gml_enum).into_box(self.span(start)))
    }

    fn try_catch(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.pilot.cursor();
        self.pilot.require(Token::Try)?;
        let try_body = self.block()?;
        self.pilot.require(Token::Catch)?;
        let catch_expr = self.expression()?;
        let catch_body = self.block()?;
        let try_catch = if self.pilot.match_take(Token::Finally).is_some() {
            TryCatch::new_with_finally(try_body, catch_expr, catch_body, self.block()?)
        } else {
            TryCatch::new(try_body, catch_expr, catch_body)
        };
        Ok(try_catch.into_statement_box(self.span(start)))
    }

    fn for_loop(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.pilot.cursor();
        self.pilot.require(Token::For)?;
        self.pilot.match_take(Token::LeftParenthesis);
        let initializer = self.statement()?;
        let condition = self.expression()?;
        self.pilot.match_take_repeating(Token::SemiColon);
        let tick = self.statement()?;
        self.pilot.match_take(Token::RightParenthesis);
        let body = self.statement()?;
        Ok(ForLoop::new(initializer, condition, tick, body).into_statement_box(self.span(start)))
    }

    fn with(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.pilot.cursor();
        self.pilot.require(Token::With)?;
        let condition = self.expression()?;
        let body = self.statement()?;
        Ok(WithLoop::new(condition, body).into_statement_box(self.span(start)))
    }

    fn repeat(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.pilot.cursor();
        self.pilot.require(Token::Repeat)?;
        let condition = self.expression()?;
        let body = self.statement()?;
        Ok(RepeatLoop::new(condition, body).into_statement_box(self.span(start)))
    }

    fn do_until(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.pilot.cursor();
        self.pilot.require(Token::Do)?;
        let body = self.statement()?;
        self.pilot.require(Token::Until)?;
        let condition = self.expression()?;
        self.pilot.match_take_repeating(Token::SemiColon);
        Ok(DoUntil::new(body, condition).into_statement_box(self.span(start)))
    }

    fn while_loop(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.pilot.cursor();
        self.pilot.require(Token::While)?;
        let condition = self.expression()?;
        let body = self.statement()?;
        Ok(If::new(condition, body).into_statement_box(self.span(start)))
    }

    fn if_statement(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.pilot.cursor();
        self.pilot.require(Token::If)?;
        let condition = self.expression()?;
        let then = self.pilot.match_take(Token::Then);
        let body = self.statement()?;
        let else_statement = if self.pilot.match_take(Token::Else).is_some() {
            Some(self.statement()?)
        } else {
            None
        };
        Ok(If {
            condition,
            body,
            else_statement,
            uses_then_keyword: then.is_some(),
        }
        .into_statement_box(self.span(start)))
    }

    fn switch(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.pilot.cursor();
        fn case_body(parser: &mut Parser) -> Result<Vec<StatementBox>, ParseError> {
            let mut body = vec![];
            loop {
                match parser.pilot.peek()? {
                    Token::Case | Token::Default | Token::RightBrace => break,
                    _ => body.push(parser.statement()?),
                }
            }
            Ok(body)
        }
        self.pilot.require(Token::Switch)?;
        let expression = self.expression()?;
        self.pilot.require(Token::LeftBrace)?;
        let mut members = vec![];
        let mut default = None;
        loop {
            match self.pilot.peek()? {
                Token::Case => {
                    self.pilot.take()?;
                    let identity = self.expression()?;
                    // todo: validate constant
                    self.pilot.require(Token::Colon)?;
                    let body = case_body(self)?;
                    members.push(SwitchCase::new(identity, body))
                }
                Token::Default => {
                    self.pilot.take()?;
                    self.pilot.require(Token::Colon)?;
                    default = Some(case_body(self)?);
                }
                Token::RightBrace => {
                    self.pilot.take()?;
                    break;
                }
                _ => return Err(ParseError::UnexpectedToken(self.span(start), self.pilot.take()?)),
            }
        }
        Ok(Statement::Switch(Switch::new(expression, members, default)).into_box(self.span(start)))
    }

    fn block(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.pilot.cursor();
        self.pilot.require(Token::LeftBrace)?;
        let mut statements: Vec<StatementBox> = vec![];
        while *self.pilot.peek()? != Token::RightBrace {
            statements.push(self.statement()?);
        }
        self.pilot.require(Token::RightBrace)?;
        self.pilot.match_take_repeating(Token::SemiColon);
        Ok(Statement::Block(statements).into_box(self.span(start)))
    }

    fn return_statement(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.pilot.cursor();
        self.pilot.require(Token::Return)?;
        let expression = self.expression().ok();
        self.pilot.match_take_repeating(Token::SemiColon);
        Ok(Statement::Return(expression).into_box(self.span(start)))
    }

    fn break_statement(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.pilot.cursor();
        self.pilot.require(Token::Break)?;
        self.pilot.match_take_repeating(Token::SemiColon);
        Ok(Statement::Break.into_box(self.span(start)))
    }

    fn continue_statement(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.pilot.cursor();
        self.pilot.require(Token::Continue)?;
        self.pilot.match_take_repeating(Token::SemiColon);
        Ok(Statement::Continue.into_box(self.span(start)))
    }

    fn exit(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.pilot.cursor();
        self.pilot.require(Token::Exit)?;
        self.pilot.match_take_repeating(Token::SemiColon);
        Ok(Statement::Exit.into_box(self.span(start)))
    }

    fn globalvar_declaration(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.pilot.cursor();
        self.pilot.require(Token::Globalvar)?;
        let name = self.pilot.require_identifier()?;
        self.pilot.match_take_repeating(Token::SemiColon);
        Ok(Globalvar::new(name).into_statement_box(self.span(start)))
    }

    fn local_variable_series(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.pilot.cursor();
        self.pilot.require(Token::Var)?;
        let mut declarations = vec![];
        loop {
            let name = self.pilot.require_identifier()?;
            let left = Identifier::new(name).into_expression_box(self.span(start));
            let local_variable = if self.pilot.match_take(Token::Equal).is_some() {
                LocalVariable::Initialized(
                    Assignment::new(left, AssignmentOperator::Equal, self.expression()?)
                        .into_expression_box(self.span(start)),
                )
            } else {
                LocalVariable::Uninitialized(left)
            };
            declarations.push(local_variable);
            if self.pilot.match_take(Token::Comma).is_none() {
                break;
            }
            if !matches!(self.pilot.soft_peek(), Some(Token::Identifier(..))) {
                // For some reason, this is valid gml:
                // ```
                // var i = 0,
                // ```
                // Lord have mercy.
                break;
            }
        }
        self.pilot.match_take_repeating(Token::SemiColon);
        Ok(LocalVariableSeries::new(declarations).into_statement_box(self.span(start)))
    }

    fn expression_statement(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.pilot.cursor();
        let expression = self.expression()?;
        match expression.expression() {
            Expression::FunctionDeclaration(..)
            | Expression::Assignment(..)
            | Expression::Postfix(..)
            | Expression::Unary(..)
            | Expression::Grouping(..)
            | Expression::Call(..) => {}

            Expression::Identifier(..) => {
                // Unfortunately, we can't (currently) understand if this is
                // actually a mistake or is a macro.
                // In the future, we may unfold code in an early pass that will
                // help with this.
            }

            // Anything else is invalid.
            _ => {
                return Err(ParseError::IncompleteStatement(self.span(start), expression));
            }
        }
        self.pilot.match_take_repeating(Token::SemiColon);
        Ok(Statement::Expression(expression).into_box(self.span(start)))
    }

    pub(super) fn expression(&mut self) -> Result<ExpressionBox, ParseError> {
        self.function()
    }

    fn function(&mut self) -> Result<ExpressionBox, ParseError> {
        let start = self.pilot.cursor();
        let static_token = self.pilot.match_take(Token::Static);
        if self.pilot.match_take(Token::Function).is_some() {
            let name = self.pilot.match_take_identifier()?;
            self.pilot.require(Token::LeftParenthesis)?;
            let mut parameters = vec![];
            loop {
                match self.pilot.peek()? {
                    Token::RightParenthesis => {
                        self.pilot.take()?;
                        break;
                    }
                    _ => {
                        let name = self.pilot.require_identifier()?;
                        let default_value = if self.pilot.match_take(Token::Equal).is_some() {
                            Some(self.expression()?)
                        } else {
                            None
                        };
                        self.pilot.match_take(Token::Comma);
                        parameters.push(Parameter(name, default_value));
                    }
                }
            }
            let inheritance = if self.pilot.match_take(Token::Colon).is_some() {
                let name = self.identifier()?;
                Some(self.call(Some(name), false)?)
            } else {
                None
            };
            let constructor = if self.pilot.match_take(Token::Constructor).is_some() {
                Some(Constructor(inheritance))
            } else {
                if inheritance.is_some() {
                    // TODO: This is invalid GML. Do we care?
                }
                None
            };
            let body = self.block()?;
            Ok(
                Expression::FunctionDeclaration(name, parameters, constructor, body, static_token.is_some())
                    .into_box(self.span(start)),
            )
        } else {
            self.null_coalecence()
        }
    }

    fn null_coalecence(&mut self) -> Result<ExpressionBox, ParseError> {
        let start = self.pilot.cursor();
        let expression = self.ternary()?;
        if self.pilot.match_take(Token::DoubleInterrobang).is_some() {
            let value = self.expression()?;
            Ok(Expression::NullCoalecence(expression, value).into_box(self.span(start)))
        } else {
            Ok(expression)
        }
    }

    fn ternary(&mut self) -> Result<ExpressionBox, ParseError> {
        let start = self.pilot.cursor();
        let expression = self.logical()?;
        if self.pilot.match_take(Token::Interrobang).is_some() {
            let true_value = self.expression()?;
            self.pilot.require(Token::Colon)?;
            let false_value = self.expression()?;
            Ok(Expression::Ternary(expression, true_value, false_value).into_box(self.span(start)))
        } else {
            Ok(expression)
        }
    }

    fn logical(&mut self) -> Result<ExpressionBox, ParseError> {
        let start = self.pilot.cursor();
        let expression = self.equality()?;
        if let Some(operator) = self.pilot.soft_peek().and_then(|token| token.as_logical_operator()) {
            self.pilot.take()?;
            let right = self.logical()?;
            Ok(Expression::Logical(expression, operator, right).into_box(self.span(start)))
        } else {
            Ok(expression)
        }
    }

    fn equality(&mut self) -> Result<ExpressionBox, ParseError> {
        let start = self.pilot.cursor();
        let expression = self.binary()?;
        if let Some(operator) = self.pilot.soft_peek().and_then(|token| token.as_equality_operator()) {
            self.pilot.take()?;
            let right = self.equality()?;
            Ok(Expression::Equality(expression, operator, right).into_box(self.span(start)))
        } else {
            Ok(expression)
        }
    }

    fn binary(&mut self) -> Result<ExpressionBox, ParseError> {
        let start = self.pilot.cursor();
        let expression = self.bitshift()?;
        if let Some(operator) = self
            .pilot
            .soft_peek()
            .map(|token| token.as_evaluation_operator())
            .filter(|operator| {
                matches!(
                    operator,
                    Some(EvaluationOperator::And) | Some(EvaluationOperator::Or) | Some(EvaluationOperator::Xor)
                )
            })
            .flatten()
        {
            self.pilot.take()?;
            let right = self.binary()?;
            Ok(Expression::Evaluation(expression, operator, right).into_box(self.span(start)))
        } else {
            Ok(expression)
        }
    }

    fn bitshift(&mut self) -> Result<ExpressionBox, ParseError> {
        let start = self.pilot.cursor();
        let expression = self.addition()?;
        if let Some(operator) = self
            .pilot
            .soft_peek()
            .map(|token| token.as_evaluation_operator())
            .filter(|operator| {
                matches!(
                    operator,
                    Some(EvaluationOperator::BitShiftLeft) | Some(EvaluationOperator::BitShiftRight)
                )
            })
            .flatten()
        {
            self.pilot.take()?;
            let right = self.bitshift()?;
            Ok(Expression::Evaluation(expression, operator, right).into_box(self.span(start)))
        } else {
            Ok(expression)
        }
    }

    fn addition(&mut self) -> Result<ExpressionBox, ParseError> {
        let start = self.pilot.cursor();
        let expression = self.multiplication()?;
        if let Some(operator) = self
            .pilot
            .soft_peek()
            .map(|token| token.as_evaluation_operator())
            .filter(|operator| {
                matches!(
                    operator,
                    Some(EvaluationOperator::Plus) | Some(EvaluationOperator::Minus)
                )
            })
            .flatten()
        {
            self.pilot.take()?;
            let right = self.addition()?;
            Ok(Expression::Evaluation(expression, operator, right).into_box(self.span(start)))
        } else {
            Ok(expression)
        }
    }

    fn multiplication(&mut self) -> Result<ExpressionBox, ParseError> {
        let start = self.pilot.cursor();
        let expression = self.assignment()?;
        if let Some(operator) = self
            .pilot
            .soft_peek()
            .map(|token| token.as_evaluation_operator())
            .filter(|operator| {
                matches!(
                    operator,
                    Some(EvaluationOperator::Star)
                        | Some(EvaluationOperator::Slash)
                        | Some(EvaluationOperator::Div)
                        | Some(EvaluationOperator::Modulo)
                )
            })
            .flatten()
        {
            self.pilot.take()?;
            let right = self.multiplication()?;
            Ok(Expression::Evaluation(expression, operator, right).into_box(self.span(start)))
        } else {
            Ok(expression)
        }
    }

    fn assignment(&mut self) -> Result<ExpressionBox, ParseError> {
        let start = self.pilot.cursor();
        let expression = self.unary()?;
        if let Some(operator) = self.pilot.soft_peek().and_then(|token| token.as_assignment_operator()) {
            if !matches!(
                expression.expression(),
                Expression::Identifier(..) | Expression::Access(..) | Expression::Call(..) /* idiotically, this does
                                                                                            * compile in GM. We have
                                                                                            * a lint for this! */
            ) {
                Err(ParseError::InvalidAssignmentTarget(self.span(start), expression))
            } else {
                self.pilot.take()?;
                let right = self.expression()?;
                Ok(Assignment::new(expression, operator, right).into_expression_box(self.span(start)))
            }
        } else {
            Ok(expression)
        }
    }

    fn unary(&mut self) -> Result<ExpressionBox, ParseError> {
        let start = self.pilot.cursor();
        if let Some(operator) = self.pilot.peek()?.as_unary_operator() {
            self.pilot.take()?;
            let right = self.expression()?;
            Ok(Expression::Unary(operator, right).into_box(self.span(start)))
        } else {
            self.postfix()
        }
    }

    fn postfix(&mut self) -> Result<ExpressionBox, ParseError> {
        let start = self.pilot.cursor();
        let expression = self.literal()?;
        if let Some(operator) = self.pilot.soft_peek().and_then(|token| token.as_postfix_operator()) {
            self.pilot.take()?;
            Ok(Expression::Postfix(expression, operator).into_box(self.span(start)))
        } else {
            Ok(expression)
        }
    }

    fn literal(&mut self) -> Result<ExpressionBox, ParseError> {
        let start = self.pilot.cursor();
        if let Some(literal) = self.pilot.peek()?.to_literal() {
            self.pilot.take()?;
            Ok(Expression::Literal(literal).into_box(self.span(start)))
        } else if self.pilot.match_take(Token::LeftSquareBracket).is_some() {
            let mut elements = vec![];
            loop {
                if self.pilot.match_take(Token::RightSquareBracket).is_some() {
                    break Ok(Expression::Literal(Literal::Array(elements)).into_box(self.span(start)));
                } else {
                    elements.push(self.expression()?);
                    self.pilot.match_take(Token::Comma);
                }
            }
        } else if self.pilot.match_take(Token::LeftBrace).is_some() {
            let mut elements = vec![];
            loop {
                if self.pilot.match_take(Token::RightBrace).is_some() {
                    break Ok(Expression::Literal(Literal::Struct(elements)).into_box(self.span(start)));
                } else {
                    let name = self.pilot.require_identifier()?;
                    self.pilot.require(Token::Colon)?;
                    elements.push((name, self.expression()?));
                    self.pilot.match_take(Token::Comma);
                }
            }
        } else {
            self.supreme()
        }
    }

    fn supreme(&mut self) -> Result<ExpressionBox, ParseError> {
        let mut has_new = self.pilot.match_take(Token::New);
        let mut expression = Some(self.call(None, has_new.take().is_some())?);
        loop {
            expression = match self.pilot.soft_peek() {
                Some(Token::LeftParenthesis) => Some(self.call(expression, has_new.take().is_some())?),
                Some(Token::LeftSquareBracket) => Some(self.ds_access(expression)?),
                Some(Token::Dot) => Some(self.dot_access(expression)?),
                _ => break Ok(expression.unwrap()),
            }
        }
    }

    fn call(&mut self, left: Option<ExpressionBox>, has_new: bool) -> Result<ExpressionBox, ParseError> {
        let start = self.pilot.cursor();
        // If we've been provided a leftside expression, we *must* parse for a call.
        // Otherwise, the call is merely possible.
        let left = if let Some(left) = left {
            left
        } else {
            let dot = self.dot_access(None)?;
            if self.pilot.soft_peek() != Some(&Token::LeftParenthesis) {
                return Ok(dot);
            }
            dot
        };
        self.pilot.require(Token::LeftParenthesis)?;
        let mut arguments = vec![];
        if self.pilot.match_take(Token::RightParenthesis).is_none() {
            loop {
                arguments.push(self.expression()?);
                match self.pilot.take()? {
                    Token::Comma => {
                        if self.pilot.match_take(Token::RightParenthesis).is_some() {
                            break;
                        }
                    }
                    Token::RightParenthesis => break,
                    token => return Err(ParseError::UnexpectedToken(self.span(start), token)),
                }
            }
        }
        Ok(Expression::Call(left, arguments, has_new).into_box(self.span(start)))
    }

    fn dot_access(&mut self, expression: Option<ExpressionBox>) -> Result<ExpressionBox, ParseError> {
        let start = self.pilot.cursor();
        let scope = if let Some(expression) = expression {
            Scope::Dot(expression)
        } else {
            match self.pilot.peek()? {
                Token::Global => {
                    self.pilot.take()?;
                    Scope::Global
                }
                Token::SelfKeyword => {
                    self.pilot.take()?;
                    if self.pilot.soft_peek() == Some(&Token::Dot) {
                        Scope::Current
                    } else {
                        // Using self as a referencce!
                        return Ok(Expression::Identifier(Identifier::new("self")).into_box(self.span(start)));
                    }
                }
                _ => {
                    let left = self.ds_access(None)?;
                    if self.pilot.soft_peek() != Some(&Token::Dot) {
                        return Ok(left);
                    }
                    Scope::Dot(left)
                }
            }
        };
        self.pilot.require(Token::Dot)?;
        let right = self.grouping()?;
        Ok(Expression::Access(scope, right).into_box(self.span(start)))
    }

    fn ds_access(&mut self, left: Option<ExpressionBox>) -> Result<ExpressionBox, ParseError> {
        let start = self.pilot.cursor();
        let left = if let Some(left) = left {
            left
        } else {
            let left = self.grouping()?;
            if self.pilot.soft_peek() != Some(&Token::LeftSquareBracket) {
                return Ok(left);
            }
            left
        };
        self.pilot.require(Token::LeftSquareBracket)?;
        let scope = match self.pilot.peek()? {
            Token::DollarSign => {
                self.pilot.take()?;
                Scope::Struct(self.expression()?)
            }
            Token::Interrobang => {
                self.pilot.take()?;
                Scope::Map(self.expression()?)
            }
            Token::Pipe => {
                self.pilot.take()?;
                Scope::List(self.expression()?)
            }
            Token::Hash => {
                self.pilot.take()?;
                let first = self.expression()?;
                self.pilot.require(Token::Comma)?;
                let second = self.expression()?;
                Scope::Grid(first, second)
            }
            _ => {
                let has_accessor = self.pilot.match_take(Token::AtSign).is_some();
                let left = self.expression()?;
                let right = if self.pilot.match_take(Token::Comma).is_some() {
                    Some(self.expression()?)
                } else {
                    None
                };
                Scope::Array(left, right, has_accessor)
            }
        };
        self.pilot.require(Token::RightSquareBracket)?;
        Ok(Expression::Access(scope, left).into_box(self.span(start)))
    }

    fn grouping(&mut self) -> Result<ExpressionBox, ParseError> {
        let start = self.pilot.cursor();
        if self.pilot.match_take(Token::LeftParenthesis).is_some() {
            let expression = self.expression()?;
            self.pilot.require(Token::RightParenthesis)?;
            Ok(Expression::Grouping(expression).into_box(self.span(start)))
        } else {
            self.identifier()
        }
    }

    fn identifier(&mut self) -> Result<ExpressionBox, ParseError> {
        let start = self.pilot.cursor();
        // TODO: TEMPORARY!!!
        self.pilot.match_take(Token::Static);

        let peek = self.pilot.peek()?;
        if let Some(lexeme) = peek.as_identifier().map(|s| s.to_string()) {
            self.pilot.take()?;
            Ok(Identifier::new(lexeme).into_expression_box(self.span(start)))
        } else {
            self.unexpected_token()
        }
    }

    fn unexpected_token(&mut self) -> Result<ExpressionBox, ParseError> {
        let start = self.pilot.cursor();
        // Note: the clone below (instead of .take()) is very intentional!
        // Users should be able to use `self.expression().ok()` and similar patterns
        // without losing tokens.
        Err(ParseError::UnexpectedToken(
            self.span(start),
            self.pilot.soft_peek().unwrap().clone(),
        ))
    }
}
