use std::path::PathBuf;

use crate::parsing::{expression::EvaluationOperator, ParseError};

use super::{
    expression::{
        AccessScope, Constructor, DSAccess, Expression, ExpressionBox, Function, Parameter,
    },
    statement::{Case, Statement, StatementBox},
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
            Token::Exit => self.exit(),
            Token::Globalvar => self.globalvar_declaration(),
            Token::Var => self.local_variable_declaration(),
            _ => self.expression_statement(),
        }
    }

    fn macro_declaration(&mut self) -> Result<StatementBox, ParseError> {
        match self.pilot.take()? {
            Token::Macro(name, config, body) => {
                Ok(Statement::MacroDeclaration(name, config, body).into())
            }
            token => Err(ParseError::UnexpectedToken(self.pilot.cursor(), token)),
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

    fn try_catch(&mut self) -> Result<StatementBox, ParseError> {
        self.pilot.require(Token::Try)?;
        let try_body = self.block()?;
        self.pilot.require(Token::Catch)?;
        let catch_expr = self.expression()?;
        let catch_body = self.block()?;
        Ok(Statement::TryCatch(try_body, catch_expr, catch_body).into())
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
        self.pilot.match_take_repeating(Token::SemiColon);
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
        self.pilot.match_take_repeating(Token::SemiColon); // yes, GM allows this...
        Ok(Statement::Block(statements).into())
    }

    fn return_statement(&mut self) -> Result<StatementBox, ParseError> {
        self.pilot.require(Token::Return)?;
        let expression = self.expression().ok();
        self.pilot.match_take_repeating(Token::SemiColon);
        Ok(Statement::Return(expression).into())
    }

    fn break_statement(&mut self) -> Result<StatementBox, ParseError> {
        self.pilot.require(Token::Break)?;
        self.pilot.match_take_repeating(Token::SemiColon);
        Ok(Statement::Break.into())
    }

    fn exit(&mut self) -> Result<StatementBox, ParseError> {
        self.pilot.require(Token::Exit)?;
        self.pilot.match_take_repeating(Token::SemiColon);
        Ok(Statement::Exit.into())
    }

    fn globalvar_declaration(&mut self) -> Result<StatementBox, ParseError> {
        self.pilot.require(Token::Globalvar)?;
        let name = self.pilot.require_identifier()?;
        self.pilot.match_take_repeating(Token::SemiColon);
        Ok(Statement::GlobalvarDeclaration(name).into())
    }

    fn local_variable_declaration(&mut self) -> Result<StatementBox, ParseError> {
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
        }
        self.pilot.match_take_repeating(Token::SemiColon);
        Ok(Statement::LocalVariableSeries(declarations).into())
    }

    fn expression_statement(&mut self) -> Result<StatementBox, ParseError> {
        let expression = self.expression()?;
        self.pilot.match_take_repeating(Token::SemiColon);
        Ok(Statement::Expression(expression).into())
    }

    pub fn expression(&mut self) -> Result<ExpressionBox, ParseError> {
        self.function()
    }

    fn function(&mut self) -> Result<ExpressionBox, ParseError> {
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
            Ok(Expression::FunctionDeclaration(function).into())
        } else {
            self.null_coalecence()
        }
    }

    fn null_coalecence(&mut self) -> Result<ExpressionBox, ParseError> {
        let expression = self.ternary()?;
        if self.pilot.match_take(Token::DoubleInterrobang).is_some() {
            let value = self.expression()?;
            Ok(Expression::NullCoalecence(expression, value).into())
        } else {
            Ok(expression)
        }
    }

    fn ternary(&mut self) -> Result<ExpressionBox, ParseError> {
        let expression = self.logical()?;
        if self.pilot.match_take(Token::Interrobang).is_some() {
            let true_value = self.expression()?;
            self.pilot.require(Token::Colon)?;
            let false_value = self.expression()?;
            Ok(Expression::Ternary(expression, true_value, false_value).into())
        } else {
            Ok(expression)
        }
    }

    fn logical(&mut self) -> Result<ExpressionBox, ParseError> {
        let expression = self.equality()?;
        if let Some(operator) = self
            .pilot
            .soft_peek()
            .map(|token| token.as_logical_operator())
            .flatten()
        {
            self.pilot.take()?;
            let right = self.logical()?;
            Ok(Expression::Logical(expression, operator, right).into())
        } else {
            Ok(expression)
        }
    }

    fn equality(&mut self) -> Result<ExpressionBox, ParseError> {
        let expression = self.binary()?;
        if let Some(operator) = self
            .pilot
            .soft_peek()
            .map(|token| token.as_equality_operator())
            .flatten()
        {
            self.pilot.take()?;
            let right = self.binary()?;
            Ok(Expression::Equality(expression, operator, right).into())
        } else {
            Ok(expression)
        }
    }

    fn binary(&mut self) -> Result<ExpressionBox, ParseError> {
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
            Ok(Expression::Evaluation(expression, operator, right).into())
        } else {
            Ok(expression)
        }
    }

    fn bitshift(&mut self) -> Result<ExpressionBox, ParseError> {
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
            Ok(Expression::Evaluation(expression, operator, right).into())
        } else {
            Ok(expression)
        }
    }

    fn addition(&mut self) -> Result<ExpressionBox, ParseError> {
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
            Ok(Expression::Evaluation(expression, operator, right).into())
        } else {
            Ok(expression)
        }
    }

    fn multiplication(&mut self) -> Result<ExpressionBox, ParseError> {
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
            Ok(Expression::Evaluation(expression, operator, right).into())
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
            if !matches!(
                *expression,
                Expression::Identifier(_)
                    | Expression::DotAccess(_, _)
                    | Expression::DSAccess(_, _)
            ) {
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
        let expression = self.array_literal()?;
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
            self.dot_access()
        }
    }

    fn dot_access(&mut self) -> Result<ExpressionBox, ParseError> {
        let access = match self.pilot.soft_peek() {
            Some(Token::Global) => {
                self.pilot.take()?;
                AccessScope::Global
            }
            Some(Token::SelfKeyword) => {
                self.pilot.take()?;
                if self.pilot.soft_peek() != Some(&Token::Dot) {
                    return Ok(Expression::Identifier("self".into()).into());
                } else {
                    AccessScope::Current
                }
            }
            _ => {
                let expression = self.ds_access()?;
                if self.pilot.soft_peek() != Some(&Token::Dot) {
                    return Ok(expression);
                } else {
                    AccessScope::Other(expression)
                }
            }
        };
        self.pilot.require(Token::Dot)?;
        let right = self.expression()?;
        Ok(Expression::DotAccess(access, right).into())
    }

    fn ds_access(&mut self) -> Result<ExpressionBox, ParseError> {
        let mut expression = self.grouping()?;
        while self.pilot.match_take(Token::LeftSquareBracket).is_some() {
            let ds_access = match self.pilot.peek()? {
                Token::DollarSign => {
                    self.pilot.take()?;
                    DSAccess::Struct(self.expression()?)
                }
                Token::Interrobang => {
                    self.pilot.take()?;
                    DSAccess::Map(self.expression()?)
                }
                Token::Pipe => {
                    self.pilot.take()?;
                    DSAccess::List(self.expression()?)
                }
                Token::Hash => {
                    self.pilot.take()?;
                    let first = self.expression()?;
                    self.pilot.require(Token::Comma)?;
                    let second = self.expression()?;
                    DSAccess::Grid(first, second)
                }
                _ => DSAccess::Array(self.expression()?),
            };
            self.pilot.require(Token::RightSquareBracket)?;
            expression = Expression::DSAccess(expression, ds_access).into();
        }
        Ok(expression)
    }

    fn grouping(&mut self) -> Result<ExpressionBox, ParseError> {
        if self.pilot.match_take(Token::LeftParenthesis).is_some() {
            let expression = self.expression()?;
            self.pilot.require(Token::RightParenthesis)?;
            Ok(Expression::Grouping(expression).into())
        } else {
            self.call()
        }
    }

    fn call(&mut self) -> Result<ExpressionBox, ParseError> {
        let new_found = self.pilot.match_take(Token::New).is_some();
        let expression = self.literal()?;
        if matches!(*expression, Expression::Identifier(_))
            && self.pilot.match_take(Token::LeftParenthesis).is_some()
        {
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
                        token => {
                            return Err(ParseError::UnexpectedToken(self.pilot.cursor(), token))
                        }
                    }
                }
            }
            Ok(Expression::Call(expression, arguments, new_found).into())
        } else if new_found {
            Err(ParseError::InvalidNewTarget(
                self.pilot.cursor(),
                expression,
            ))
        } else {
            Ok(expression)
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
