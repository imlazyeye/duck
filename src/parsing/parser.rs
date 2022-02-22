use std::path::PathBuf;

use crate::{
    parsing::{expression::EvaluationOperator, ParseError},
    Span,
};

use super::{
    expression::{AccessScope, Constructor, Expression, ExpressionBox, Literal, Parameter},
    statement::{Case, Statement, StatementBox},
    token_pilot::TokenPilot,
    Token,
};

pub type Ast = Vec<StatementBox>;

pub struct Parser<'a> {
    pilot: TokenPilot<'a>,

    // rust analyzer mishaps below
    #[allow(dead_code)]
    source_code: &'a str,
    #[allow(dead_code)]
    resource_path: PathBuf,
}

impl<'a> Parser<'a> {
    pub fn new(source_code: &'a str, resource_path: PathBuf) -> Self {
        Self {
            pilot: TokenPilot::new(source_code),
            resource_path,
            source_code,
        }
    }

    pub fn into_ast(mut self) -> Result<Ast, ParseError> {
        let mut statements: Ast = vec![];
        while self.pilot.soft_peek().is_some() {
            statements.push(self.statement()?);
        }
        Ok(statements)
    }

    #[cfg(not(test))]
    pub fn span(&mut self, start: usize) -> Span {
        Span(start, self.pilot.cursor())
    }

    #[cfg(test)]
    pub fn span(&self, _start: usize) -> Span {
        Span::default()
    }

    pub fn statement(&mut self) -> Result<StatementBox, ParseError> {
        match self.pilot.peek()? {
            Token::Macro(_, _, _) => self.macro_declaration(),
            Token::Enum => self.enum_declaration(),
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
                Ok(Statement::MacroDeclaration(name, config, body).into_box(self.span(start)))
            }
            token => Err(ParseError::UnexpectedToken(self.span(start), token)),
        }
    }

    fn enum_declaration(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.pilot.cursor();
        self.pilot.require(Token::Enum)?;
        let name = self.pilot.require_identifier()?;
        self.pilot.require(Token::LeftBrace)?;
        let mut members = vec![];
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
                members.push((name, initializer));
                self.pilot.match_take(Token::Comma);
            }
        }
        Ok(Statement::EnumDeclaration(name, members).into_box(self.span(start)))
    }

    fn try_catch(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.pilot.cursor();
        self.pilot.require(Token::Try)?;
        let try_body = self.block()?;
        self.pilot.require(Token::Catch)?;
        let catch_expr = self.expression()?;
        let catch_body = self.block()?;
        Ok(Statement::TryCatch(try_body, catch_expr, catch_body).into_box(self.span(start)))
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
        Ok(Statement::For(initializer, condition, tick, body).into_box(self.span(start)))
    }

    fn with(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.pilot.cursor();
        self.pilot.require(Token::With)?;
        let condition = self.expression()?;
        let body = self.statement()?;
        Ok(Statement::With(condition, body).into_box(self.span(start)))
    }

    fn repeat(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.pilot.cursor();
        self.pilot.require(Token::Repeat)?;
        let condition = self.expression()?;
        let body = self.statement()?;
        Ok(Statement::Repeat(condition, body).into_box(self.span(start)))
    }

    fn do_until(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.pilot.cursor();
        self.pilot.require(Token::Do)?;
        let body = self.statement()?;
        self.pilot.require(Token::Until)?;
        let condition = self.expression()?;
        self.pilot.match_take_repeating(Token::SemiColon);
        Ok(Statement::DoUntil(body, condition).into_box(self.span(start)))
    }

    fn while_loop(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.pilot.cursor();
        self.pilot.require(Token::While)?;
        let condition = self.expression()?;
        let body = self.statement()?;
        Ok(Statement::While(condition, body).into_box(self.span(start)))
    }

    fn if_statement(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.pilot.cursor();
        self.pilot.require(Token::If)?;
        let condition = self.expression()?;
        let body = self.statement()?;
        let else_branch = if self.pilot.match_take(Token::Else).is_some() {
            Some(self.statement()?)
        } else {
            None
        };
        Ok(Statement::If(condition, body, else_branch).into_box(self.span(start)))
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
                    members.push(Case(identity, body))
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
                _ => {
                    return Err(ParseError::UnexpectedToken(
                        self.span(start),
                        self.pilot.take()?,
                    ))
                }
            }
        }
        Ok(Statement::Switch(expression, members, default).into_box(self.span(start)))
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
        Ok(Statement::GlobalvarDeclaration(name).into_box(self.span(start)))
    }

    fn local_variable_series(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.pilot.cursor();
        self.pilot.require(Token::Var)?;
        let mut declarations = vec![];
        loop {
            let name = self.pilot.require_identifier()?;
            let initializer = if self.pilot.match_take(Token::Equal).is_some() {
                Some(self.expression()?)
            } else {
                None
            };
            declarations.push((name, initializer));
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
        Ok(Statement::LocalVariableSeries(declarations).into_box(self.span(start)))
    }

    fn expression_statement(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.pilot.cursor();
        let expression = self.expression()?;
        match expression.expression() {
            Expression::FunctionDeclaration(..)
            | Expression::Assignment(..)
            | Expression::Postfix(..)
            | Expression::Unary(..)
            | Expression::Call(..) => {}

            // We have to do this to allow:
            // ```
            // foo.bar();
            // ```
            // Because, while that *really* is a call, it's a dot access...
            // that should change...
            Expression::Access(..) => {}

            _ => {
                // Temporarily ignoring!!!
                if expression.expression() != &Expression::Identifier("unsafe".into()) {
                    return Err(ParseError::IncompleteStatement(
                        self.span(start),
                        expression,
                    ));
                }
            }
        }
        self.pilot.match_take_repeating(Token::SemiColon);
        Ok(Statement::Expression(expression).into_box(self.span(start)))
    }

    pub fn expression(&mut self) -> Result<ExpressionBox, ParseError> {
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
                Some(self.call(name, false)?)
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
            Ok(Expression::FunctionDeclaration(
                name,
                parameters,
                constructor,
                body,
                static_token.is_some(),
            )
            .into_box(self.span(start)))
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
        if let Some(operator) = self
            .pilot
            .soft_peek()
            .map(|token| token.as_logical_operator())
            .flatten()
        {
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
        if let Some(operator) = self
            .pilot
            .soft_peek()
            .map(|token| token.as_equality_operator())
            .flatten()
        {
            self.pilot.take()?;
            let right = self.binary()?;
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
                    Some(EvaluationOperator::And)
                        | Some(EvaluationOperator::Or)
                        | Some(EvaluationOperator::Xor)
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
                    Some(EvaluationOperator::BitShiftLeft)
                        | Some(EvaluationOperator::BitShiftRight)
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
        if let Some(operator) = self
            .pilot
            .soft_peek()
            .map(|token| token.as_assignment_operator())
            .flatten()
        {
            if !matches!(
                expression.expression(),
                Expression::Identifier(..) | Expression::Access(..) | Expression::Call(..) // idiotically, this does compile in GM. We have a lint for this!
            ) {
                Err(ParseError::InvalidAssignmentTarget(
                    self.span(start),
                    expression,
                ))
            } else {
                self.pilot.take()?;
                let right = self.expression()?;
                Ok(Expression::Assignment(expression, operator, right).into_box(self.span(start)))
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
        let expression = self.grouping()?;
        if let Some(operator) = self
            .pilot
            .soft_peek()
            .map(|token| token.as_postfix_operator())
            .flatten()
        {
            self.pilot.take()?;
            Ok(Expression::Postfix(expression, operator).into_box(self.span(start)))
        } else {
            Ok(expression)
        }
    }

    fn grouping(&mut self) -> Result<ExpressionBox, ParseError> {
        let start = self.pilot.cursor();
        if self.pilot.match_take(Token::LeftParenthesis).is_some() {
            let expression = self.expression()?;
            self.pilot.require(Token::RightParenthesis)?;
            Ok(Expression::Grouping(expression).into_box(self.span(start)))
        } else {
            self.literal()
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
                    break Ok(
                        Expression::Literal(Literal::Array(elements)).into_box(self.span(start))
                    );
                } else {
                    elements.push(self.expression()?);
                    self.pilot.match_take(Token::Comma);
                }
            }
        } else if self.pilot.match_take(Token::LeftBrace).is_some() {
            let mut elements = vec![];
            loop {
                if self.pilot.match_take(Token::RightBrace).is_some() {
                    break Ok(
                        Expression::Literal(Literal::Struct(elements)).into_box(self.span(start))
                    );
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
        let start = self.pilot.cursor();
        // Get the first expression...
        let mut has_new = self.pilot.match_take(Token::New).is_some();
        let mut expression = match self.pilot.peek()? {
            Token::Global => {
                self.pilot.take()?;
                self.dot_access(AccessScope::Global)?
            }
            Token::SelfKeyword => {
                self.pilot.take()?;
                if self.pilot.soft_peek() == Some(&Token::Dot) {
                    self.dot_access(AccessScope::Current)?
                } else {
                    Expression::Identifier("self".into()).into_box(self.span(start))
                }
            }
            _ => self.identifier()?,
        };

        // Now continue to chain more things on as long as we need!
        loop {
            expression = match self.pilot.soft_peek() {
                Some(Token::Dot) => self.dot_access(AccessScope::Dot(expression))?,
                Some(Token::LeftSquareBracket) => self.ds_access(expression)?,
                Some(Token::LeftParenthesis) => {
                    let call = self.call(expression, has_new)?;
                    has_new = false;
                    call
                }
                _ => break Ok(expression),
            };
        }
    }

    fn call(&mut self, left: ExpressionBox, has_new: bool) -> Result<ExpressionBox, ParseError> {
        let start = self.pilot.cursor();
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

    fn dot_access(&mut self, access: AccessScope) -> Result<ExpressionBox, ParseError> {
        let start = self.pilot.cursor();
        self.pilot.require(Token::Dot)?;
        let right = self.expression()?;
        Ok(Expression::Access(right, access).into_box(self.span(start)))
    }

    fn ds_access(&mut self, left: ExpressionBox) -> Result<ExpressionBox, ParseError> {
        let start = self.pilot.cursor();
        self.pilot.require(Token::LeftSquareBracket)?;
        let ds_access = match self.pilot.peek()? {
            Token::DollarSign => {
                self.pilot.take()?;
                AccessScope::Struct(self.expression()?)
            }
            Token::Interrobang => {
                self.pilot.take()?;
                AccessScope::Map(self.expression()?)
            }
            Token::Pipe => {
                self.pilot.take()?;
                AccessScope::List(self.expression()?)
            }
            Token::Hash => {
                self.pilot.take()?;
                let first = self.expression()?;
                self.pilot.require(Token::Comma)?;
                let second = self.expression()?;
                AccessScope::Grid(first, second)
            }
            _ => {
                let has_accessor = self.pilot.match_take(Token::AtSign).is_some();
                let left = self.expression()?;
                let right = if self.pilot.match_take(Token::Comma).is_some() {
                    Some(self.expression()?)
                } else {
                    None
                };
                AccessScope::Array(left, right, has_accessor)
            }
        };
        self.pilot.require(Token::RightSquareBracket)?;
        Ok(Expression::Access(left, ds_access).into_box(self.span(start)))
    }

    fn identifier(&mut self) -> Result<ExpressionBox, ParseError> {
        let start = self.pilot.cursor();
        // TODO: TEMPORARY!!!
        self.pilot.match_take(Token::Static);

        if let Some(lexeme) = self.pilot.peek()?.as_identifier().map(|s| s.to_string()) {
            self.pilot.take()?;
            Ok(Expression::Identifier(lexeme).into_box(self.span(start)))
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
