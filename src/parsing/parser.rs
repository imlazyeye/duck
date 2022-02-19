use std::path::PathBuf;

use crate::parsing::ParseError;

use super::{
    expression::{DsAccess, Expression, ExpressionBox},
    statement::{Case, Constructor, Function, Parameter, Statement, StatementBox},
    token_pilot::TokenPilot,
    Token,
};

pub type Ast = Vec<StatementBox>;

pub struct Parser<'a> {
    source_code: &'a str,
    pilot: TokenPilot<'a>,
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

    pub fn statement(&mut self) -> Result<StatementBox, ParseError> {
        match self.pilot.peek()? {
            Token::Enum => self.enum_declaration(),
            Token::Function => self.function(),
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
            Token::Exit => self.exit(),
            Token::Var => self.local_variable_declaration(),
            _ => self.expression_statement(),
        }
    }

    fn enum_declaration(&mut self) -> Result<StatementBox, ParseError> {
        self.pilot.require(Token::Enum)?;
        let name = self.pilot.require_identifier()?;
        self.pilot.require(Token::LeftBrace)?;
        let mut members = vec![];
        loop {
            if self.pilot.match_take(Token::RightBrace).is_some() {
                break;
            } else {
                members.push(self.expression()?);
                self.pilot.match_take(Token::Comma);
            }
        }
        Ok(Statement::EnumDeclaration(name, members).into())
    }

    fn function(&mut self) -> Result<StatementBox, ParseError> {
        self.pilot.require(Token::Function)?;
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
                    let default_value = if self.pilot.match_take(Token::Equals).is_some() {
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
            Some(self.call()?)
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
        let function = if let Some(name) = name {
            Function::Named(name, parameters, constructor, body)
        } else {
            Function::Anonymous(parameters, constructor, body)
        };
        Ok(Statement::FunctionDeclaration(function).into())
    }

    fn for_loop(&mut self) -> Result<StatementBox, ParseError> {
        self.pilot.require(Token::For)?;
        self.pilot.match_take(Token::LeftParenthesis);
        let initializer = self.statement()?;
        let condition = self.statement()?;
        let tick = self.statement()?;
        self.pilot.match_take(Token::RightParenthesis);
        let body = self.statement()?;
        Ok(Statement::For(initializer, condition, tick, body).into())
    }

    fn with(&mut self) -> Result<StatementBox, ParseError> {
        self.pilot.require(Token::With)?;
        let condition = self.expression()?;
        let body = self.statement()?;
        Ok(Statement::With(condition, body).into())
    }

    fn repeat(&mut self) -> Result<StatementBox, ParseError> {
        self.pilot.require(Token::Repeat)?;
        let condition = self.expression()?;
        let body = self.statement()?;
        Ok(Statement::Repeat(condition, body).into())
    }

    fn do_until(&mut self) -> Result<StatementBox, ParseError> {
        self.pilot.require(Token::Do)?;
        let body = self.statement()?;
        self.pilot.require(Token::Until)?;
        let condition = self.expression()?;
        Ok(Statement::DoUntil(body, condition).into())
    }

    fn while_loop(&mut self) -> Result<StatementBox, ParseError> {
        self.pilot.require(Token::While)?;
        let condition = self.expression()?;
        let body = self.statement()?;
        Ok(Statement::While(condition, body).into())
    }

    fn if_statement(&mut self) -> Result<StatementBox, ParseError> {
        self.pilot.require(Token::If)?;
        let condition = self.expression()?;
        let body = self.statement()?;
        let else_branch = if self.pilot.match_take(Token::Else).is_some() {
            Some(self.statement()?)
        } else {
            None
        };
        Ok(Statement::If(condition, body, else_branch).into())
    }

    fn switch(&mut self) -> Result<StatementBox, ParseError> {
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
                    let identity = self.literal()?;
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
                        self.pilot.cursor(),
                        self.pilot.take()?,
                    ))
                }
            }
        }
        Ok(Statement::Switch(expression, members, default).into())
    }

    fn block(&mut self) -> Result<StatementBox, ParseError> {
        self.pilot.require(Token::LeftBrace)?;
        let mut statements: Vec<StatementBox> = vec![];
        while *self.pilot.peek()? != Token::RightBrace {
            statements.push(self.statement()?);
        }
        self.pilot.require(Token::RightBrace)?;
        Ok(Statement::Block(statements).into())
    }

    fn return_statement(&mut self) -> Result<StatementBox, ParseError> {
        self.pilot.require(Token::Return)?;
        let expression = self.expression().ok();
        self.pilot.match_take(Token::SemiColon);
        Ok(Statement::Return(expression).into())
    }

    fn break_statement(&mut self) -> Result<StatementBox, ParseError> {
        self.pilot.require(Token::Break)?;
        self.pilot.match_take(Token::SemiColon);
        Ok(Statement::Break.into())
    }

    fn exit(&mut self) -> Result<StatementBox, ParseError> {
        self.pilot.require(Token::Exit)?;
        self.pilot.match_take(Token::SemiColon);
        Ok(Statement::Exit.into())
    }

    fn local_variable_declaration(&mut self) -> Result<StatementBox, ParseError> {
        self.pilot.require(Token::Var)?;
        let name = self.pilot.require_identifier()?;
        let initializer = if self.pilot.match_take(Token::Equals).is_some() {
            Some(self.expression()?)
        } else {
            None
        };
        self.pilot.match_take(Token::SemiColon);
        // todo: support series
        Ok(Statement::VariableDeclaration(name, initializer).into())
    }

    fn expression_statement(&mut self) -> Result<StatementBox, ParseError> {
        let expression = self.expression()?;
        self.pilot.match_take(Token::SemiColon);
        Ok(Statement::Expression(expression).into())
    }

    pub fn expression(&mut self) -> Result<ExpressionBox, ParseError> {
        self.evaluation()
    }

    fn evaluation(&mut self) -> Result<ExpressionBox, ParseError> {
        let expression = self.ternary()?;
        if let Some(operator) = self
            .pilot
            .soft_peek()
            .map(|token| token.as_evaluation_operator())
            .flatten()
        {
            self.pilot.take()?;
            let right = self.expression()?;
            Ok(Expression::Evaluation(expression, operator, right).into())
        } else {
            Ok(expression)
        }
    }

    fn ternary(&mut self) -> Result<ExpressionBox, ParseError> {
        let expression = self.assignment()?;
        if self.pilot.match_take(Token::Interrobang).is_some() {
            let true_value = self.expression()?;
            self.pilot.require(Token::Colon)?;
            let false_value = self.expression()?;
            Ok(Expression::Ternary(expression, true_value, false_value).into())
        } else {
            Ok(expression)
        }
    }

    fn assignment(&mut self) -> Result<ExpressionBox, ParseError> {
        let expression = self.unary()?;
        if let Some(operator) = self
            .pilot
            .soft_peek()
            .map(|token| token.as_assignment_operator())
            .flatten()
        {
            if !matches!(*expression, Expression::Identifier(_)) {
                Err(ParseError::InvalidAssignmentTarget(
                    self.pilot.cursor(),
                    expression,
                ))
            } else {
                self.pilot.take()?;
                let right = self.expression()?;
                Ok(Expression::Assignment(expression, operator, right).into())
            }
        } else {
            Ok(expression)
        }
    }

    fn unary(&mut self) -> Result<ExpressionBox, ParseError> {
        if let Some(operator) = self.pilot.peek()?.as_unary_operator() {
            self.pilot.take()?;
            let right = self.expression()?;
            Ok(Expression::Unary(operator, right).into())
        } else {
            self.postfix()
        }
    }

    fn postfix(&mut self) -> Result<ExpressionBox, ParseError> {
        let expression = self.call()?;
        if let Some(operator) = self
            .pilot
            .soft_peek()
            .map(|token| token.as_postfix_operator())
            .flatten()
        {
            self.pilot.take()?;
            Ok(Expression::Postfix(expression, operator).into())
        } else {
            Ok(expression)
        }
    }

    fn call(&mut self) -> Result<ExpressionBox, ParseError> {
        let expression = self.array_literal()?;
        if matches!(*expression, Expression::Identifier { .. })
            && self.pilot.match_take(Token::LeftParenthesis).is_some()
        {
            let mut arguments = vec![];
            if self.pilot.match_take(Token::RightParenthesis).is_none() {
                loop {
                    arguments.push(self.expression()?);
                    match self.pilot.take()? {
                        Token::Comma => {}
                        Token::RightParenthesis => break,
                        token => {
                            return Err(ParseError::UnexpectedToken(self.pilot.cursor(), token))
                        }
                    }
                }
            }
            Ok(Expression::Call(expression, arguments).into())
        } else {
            Ok(expression)
        }
    }

    fn array_literal(&mut self) -> Result<ExpressionBox, ParseError> {
        if self.pilot.match_take(Token::LeftSquareBracket).is_some() {
            let mut elements = vec![];
            loop {
                if self.pilot.match_take(Token::RightSquareBracket).is_some() {
                    break Ok(Expression::ArrayLiteral(elements).into());
                } else {
                    elements.push(self.expression()?);
                    self.pilot.match_take(Token::Comma);
                }
            }
        } else {
            self.struct_literal()
        }
    }

    fn struct_literal(&mut self) -> Result<ExpressionBox, ParseError> {
        if self.pilot.match_take(Token::LeftBrace).is_some() {
            let mut elements = vec![];
            loop {
                if self.pilot.match_take(Token::RightBrace).is_some() {
                    break Ok(Expression::StructLiteral(elements).into());
                } else {
                    let name = self.pilot.require_identifier()?;
                    self.pilot.require(Token::Colon)?;
                    elements.push((name, self.expression()?));
                    self.pilot.match_take(Token::Comma);
                }
            }
        } else {
            self.ds_access()
        }
    }

    fn ds_access(&mut self) -> Result<ExpressionBox, ParseError> {
        let expression = self.dot_access()?;
        if self.pilot.match_take(Token::LeftSquareBracket).is_some() {
            let ds_access = match self.pilot.peek()? {
                Token::Interrobang => {
                    self.pilot.take()?;
                    DsAccess::Map(self.expression()?)
                }
                Token::BitwiseOr => {
                    self.pilot.take()?;
                    DsAccess::List(self.expression()?)
                }
                Token::Hash => {
                    self.pilot.take()?;
                    let first = self.expression()?;
                    self.pilot.require(Token::Comma)?;
                    let second = self.expression()?;
                    DsAccess::Grid(first, second)
                }
                _ => DsAccess::Array(self.expression()?),
            };
            self.pilot.require(Token::RightSquareBracket)?;
            Ok(Expression::DSAccess(expression, ds_access).into())
        } else {
            Ok(expression)
        }
    }

    fn dot_access(&mut self) -> Result<ExpressionBox, ParseError> {
        let expression = self.grouping()?;
        if self.pilot.match_take(Token::Dot).is_some() {
            let right = self.call()?;
            Ok(Expression::DotAccess(expression, right).into())
        } else {
            Ok(expression)
        }
    }

    fn grouping(&mut self) -> Result<ExpressionBox, ParseError> {
        if self.pilot.match_take(Token::LeftParenthesis).is_some() {
            let expression = self.expression()?;
            self.pilot.require(Token::RightParenthesis)?;
            Ok(Expression::Grouping(expression).into())
        } else {
            self.literal()
        }
    }

    fn literal(&mut self) -> Result<ExpressionBox, ParseError> {
        if let Some(literal) = self.pilot.peek()?.to_literal() {
            self.pilot.take()?;
            Ok(Expression::Literal(literal).into())
        } else {
            self.identifier()
        }
    }

    fn identifier(&mut self) -> Result<ExpressionBox, ParseError> {
        if let Some(lexeme) = self.pilot.peek()?.as_identifier().map(|s| s.to_string()) {
            self.pilot.take()?;
            Ok(Expression::Identifier(lexeme).into())
        } else {
            self.unexpected_token()
        }
    }

    fn unexpected_token(&mut self) -> Result<ExpressionBox, ParseError> {
        Err(ParseError::UnexpectedToken(
            self.pilot.cursor(),
            self.pilot.take()?,
        ))
    }
}
