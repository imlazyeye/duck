use crate::{parsing::*, utils::Span};
use std::{iter::Peekable, path::PathBuf};

/// A collection of statements.
pub type Ast = Vec<StatementBox>;

/// Recursively decsends Gml source, incremently returning various statements
/// and expressions.
pub struct Parser {
    lexer: Peekable<Lexer>,
    cursor: usize,

    // rust analyzer mishaps below
    #[allow(dead_code)]
    source_code: &'static str,
    #[allow(dead_code)]
    resource_path: PathBuf,
}

impl Parser {
    /// Creates a new parser.
    pub fn new(source_code: &'static str, resource_path: PathBuf) -> Self {
        Self {
            lexer: Lexer::new(source_code).peekable(),
            cursor: 0,
            resource_path,
            source_code,
        }
    }

    /// Runs the parser through the entire source, collecting everything into an
    /// Ast and returning it.
    ///
    /// ### Errors
    ///
    /// Returns a [ParseError] if any of the source code caused an error.
    pub fn into_ast(mut self) -> Result<Ast, ParseError> {
        let mut statements: Ast = vec![];
        while self.soft_peek().is_some() {
            statements.push(self.statement()?);
        }
        Ok(statements)
    }

    /// Creates a [Span] from the given position up until the pilot's current
    /// position.
    #[cfg(not(test))]
    pub fn span(&mut self, start: usize) -> Span {
        Span(start, self.cursor())
    }

    #[cfg(test)]
    pub fn span(&self, _start: usize) -> Span {
        Span::default()
    }

    pub(super) fn statement(&mut self) -> Result<StatementBox, ParseError> {
        match self.peek()? {
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
        let start = self.cursor();
        match self.take()? {
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
        let start = self.cursor();
        self.require(Token::Enum)?;
        let name = self.require_identifier()?;
        let mut gml_enum = Enum::new(name);
        self.require(Token::LeftBrace)?;
        loop {
            if self.match_take(Token::RightBrace).is_some() {
                break;
            } else {
                let name = self.require_identifier()?;
                let initializer = if self.match_take(Token::Equal).is_some() {
                    Some(self.expression()?)
                } else {
                    None
                };
                gml_enum.register_member(name, initializer);
                self.match_take(Token::Comma);
            }
        }
        // GM accepts semicolons here, and as such, so do we.
        // FIXME: create an infastrucutre such that we can lint this?
        self.match_take_repeating(Token::SemiColon);
        Ok(gml_enum.into_statement_box(self.span(start)))
    }

    fn try_catch(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.cursor();
        self.require(Token::Try)?;
        let try_body = self.block()?;
        self.require(Token::Catch)?;
        let catch_expr = self.expression()?;
        let catch_body = self.block()?;
        let try_catch = if self.match_take(Token::Finally).is_some() {
            TryCatch::new_with_finally(try_body, catch_expr, catch_body, self.block()?)
        } else {
            TryCatch::new(try_body, catch_expr, catch_body)
        };
        Ok(try_catch.into_statement_box(self.span(start)))
    }

    fn for_loop(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.cursor();
        self.require(Token::For)?;
        self.match_take(Token::LeftParenthesis);
        let initializer = self.statement()?;
        let condition = self.expression()?;
        self.match_take_repeating(Token::SemiColon);
        let tick = self.statement()?;
        self.match_take(Token::RightParenthesis);
        let body = self.statement()?;
        Ok(ForLoop::new(initializer, condition, tick, body).into_statement_box(self.span(start)))
    }

    fn with(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.cursor();
        self.require(Token::With)?;
        let condition = self.expression()?;
        let body = self.statement()?;
        Ok(WithLoop::new(condition, body).into_statement_box(self.span(start)))
    }

    fn repeat(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.cursor();
        self.require(Token::Repeat)?;
        let condition = self.expression()?;
        let body = self.statement()?;
        Ok(RepeatLoop::new(condition, body).into_statement_box(self.span(start)))
    }

    fn do_until(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.cursor();
        self.require(Token::Do)?;
        let body = self.statement()?;
        self.require(Token::Until)?;
        let condition = self.expression()?;
        self.match_take_repeating(Token::SemiColon);
        Ok(DoUntil::new(body, condition).into_statement_box(self.span(start)))
    }

    fn while_loop(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.cursor();
        self.require(Token::While)?;
        let condition = self.expression()?;
        let body = self.statement()?;
        Ok(If::new(condition, body).into_statement_box(self.span(start)))
    }

    fn if_statement(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.cursor();
        self.require(Token::If)?;
        let condition = self.expression()?;
        let then = self.match_take(Token::Then);
        let body = self.statement()?;
        let else_statement = if self.match_take(Token::Else).is_some() {
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
        let start = self.cursor();
        fn case_body(parser: &mut Parser) -> Result<Vec<StatementBox>, ParseError> {
            let mut body = vec![];
            loop {
                match parser.peek()? {
                    Token::Case | Token::Default | Token::RightBrace => break,
                    _ => body.push(parser.statement()?),
                }
            }
            Ok(body)
        }
        self.require(Token::Switch)?;
        let expression = self.expression()?;
        self.require(Token::LeftBrace)?;
        let mut members = vec![];
        let mut default = None;
        loop {
            match self.peek()? {
                Token::Case => {
                    self.take()?;
                    let identity = self.expression()?;
                    self.require(Token::Colon)?;
                    let body = case_body(self)?;
                    members.push(SwitchCase::new(identity, body))
                }
                Token::Default => {
                    self.take()?;
                    self.require(Token::Colon)?;
                    default = Some(case_body(self)?);
                }
                Token::RightBrace => {
                    self.take()?;
                    break;
                }
                _ => return Err(ParseError::UnexpectedToken(self.span(start), self.take()?)),
            }
        }
        Ok(Switch::new(expression, members, default).into_statement_box(self.span(start)))
    }

    fn block(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.cursor();
        self.require(Token::LeftBrace)?;
        let mut statements: Vec<StatementBox> = vec![];
        while *self.peek()? != Token::RightBrace {
            statements.push(self.statement()?);
        }
        self.require(Token::RightBrace)?;
        self.match_take_repeating(Token::SemiColon);
        Ok(Block::new(statements).into_statement_box(self.span(start)))
    }

    fn return_statement(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.cursor();
        self.require(Token::Return)?;
        let expression = self.expression().ok();
        self.match_take_repeating(Token::SemiColon);
        Ok(Return::new(expression).into_statement_box(self.span(start)))
    }

    fn break_statement(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.cursor();
        self.require(Token::Break)?;
        self.match_take_repeating(Token::SemiColon);
        Ok(Statement::Break.into_statement_box(self.span(start)))
    }

    fn continue_statement(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.cursor();
        self.require(Token::Continue)?;
        self.match_take_repeating(Token::SemiColon);
        Ok(Statement::Continue.into_statement_box(self.span(start)))
    }

    fn exit(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.cursor();
        self.require(Token::Exit)?;
        self.match_take_repeating(Token::SemiColon);
        Ok(Statement::Exit.into_statement_box(self.span(start)))
    }

    fn globalvar_declaration(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.cursor();
        self.require(Token::Globalvar)?;
        let name = self.require_identifier()?;
        self.match_take_repeating(Token::SemiColon);
        Ok(Globalvar::new(name).into_statement_box(self.span(start)))
    }

    fn local_variable_series(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.cursor();
        self.require(Token::Var)?;
        let mut declarations = vec![];
        loop {
            let name = self.require_identifier()?;
            let left = Identifier::new(name).into_expression_box(self.span(start));
            let local_variable = if self.match_take(Token::Equal).is_some() {
                LocalVariable::Initialized(
                    Assignment::new(left, AssignmentOperator::Equal, self.expression()?)
                        .into_expression_box(self.span(start)),
                )
            } else {
                LocalVariable::Uninitialized(left)
            };
            declarations.push(local_variable);
            if self.match_take(Token::Comma).is_none() {
                break;
            }
            if !matches!(self.soft_peek(), Some(Token::Identifier(..))) {
                // For some reason, this is valid gml:
                // ```
                // var i = 0,
                // ```
                // Lord have mercy.
                break;
            }
        }
        self.match_take_repeating(Token::SemiColon);
        Ok(LocalVariableSeries::new(declarations).into_statement_box(self.span(start)))
    }

    fn expression_statement(&mut self) -> Result<StatementBox, ParseError> {
        let start = self.cursor();
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
        self.match_take_repeating(Token::SemiColon);
        Ok(Statement::Expression(expression).into_statement_box(self.span(start)))
    }

    pub(super) fn expression(&mut self) -> Result<ExpressionBox, ParseError> {
        self.function()
    }

    fn function(&mut self) -> Result<ExpressionBox, ParseError> {
        let start = self.cursor();
        // TODO: when we do static-analysis, this will be used
        let _static_token = self.match_take(Token::Static);
        if self.match_take(Token::Function).is_some() {
            let name = self.match_take_identifier()?;
            self.require(Token::LeftParenthesis)?;
            let mut parameters = vec![];
            loop {
                match self.peek()? {
                    Token::RightParenthesis => {
                        self.take()?;
                        break;
                    }
                    _ => {
                        let name = self.require_identifier()?;
                        if self.match_take(Token::Equal).is_some() {
                            parameters.push(Parameter::new_with_default(name, self.expression()?));
                        } else {
                            parameters.push(Parameter::new(name));
                        };
                        self.match_take(Token::Comma);
                    }
                }
            }
            let colon_position = self.cursor;
            let inheritance = if self.match_take(Token::Colon).is_some() {
                let name = self.identifier()?;
                Some(self.call(Some(name), false)?)
            } else {
                None
            };
            let constructor = if self.match_take(Token::Constructor).is_some() {
                match inheritance {
                    Some(inheritance) => Some(Constructor::WithInheritance(inheritance)),
                    None => Some(Constructor::WithoutInheritance),
                }
            } else {
                if inheritance.is_some() {
                    return Err(ParseError::UnexpectedToken(self.span(colon_position), Token::Colon));
                }
                None
            };
            let body = self.block()?;
            Ok(Function {
                name,
                parameters,
                constructor,
                body,
            }
            .into_expression_box(self.span(start)))
        } else {
            self.null_coalecence()
        }
    }

    fn null_coalecence(&mut self) -> Result<ExpressionBox, ParseError> {
        let start = self.cursor();
        let expression = self.ternary()?;
        if self.match_take(Token::DoubleInterrobang).is_some() {
            let value = self.expression()?;
            Ok(NullCoalecence::new(expression, value).into_expression_box(self.span(start)))
        } else {
            Ok(expression)
        }
    }

    fn ternary(&mut self) -> Result<ExpressionBox, ParseError> {
        let start = self.cursor();
        let expression = self.logical()?;
        if self.match_take(Token::Interrobang).is_some() {
            let true_value = self.expression()?;
            self.require(Token::Colon)?;
            let false_value = self.expression()?;
            Ok(Ternary::new(expression, true_value, false_value).into_expression_box(self.span(start)))
        } else {
            Ok(expression)
        }
    }

    fn logical(&mut self) -> Result<ExpressionBox, ParseError> {
        let start = self.cursor();
        let expression = self.equality()?;
        if let Some(operator) = self.soft_peek().and_then(|token| token.as_logical_operator()) {
            self.take()?;
            let right = self.logical()?;
            Ok(Logical::new(expression, operator, right).into_expression_box(self.span(start)))
        } else {
            Ok(expression)
        }
    }

    fn equality(&mut self) -> Result<ExpressionBox, ParseError> {
        let start = self.cursor();
        let expression = self.binary()?;
        if let Some(operator) = self.soft_peek().and_then(|token| token.as_equality_operator()) {
            self.take()?;
            let right = self.equality()?;
            Ok(Equality::new(expression, operator, right).into_expression_box(self.span(start)))
        } else {
            Ok(expression)
        }
    }

    fn binary(&mut self) -> Result<ExpressionBox, ParseError> {
        let start = self.cursor();
        let expression = self.bitshift()?;
        if let Some(operator) = self
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
            self.take()?;
            let right = self.binary()?;
            Ok(Evaluation::new(expression, operator, right).into_expression_box(self.span(start)))
        } else {
            Ok(expression)
        }
    }

    fn bitshift(&mut self) -> Result<ExpressionBox, ParseError> {
        let start = self.cursor();
        let expression = self.addition()?;
        if let Some(operator) = self
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
            self.take()?;
            let right = self.bitshift()?;
            Ok(Evaluation::new(expression, operator, right).into_expression_box(self.span(start)))
        } else {
            Ok(expression)
        }
    }

    fn addition(&mut self) -> Result<ExpressionBox, ParseError> {
        let start = self.cursor();
        let expression = self.multiplication()?;
        if let Some(operator) = self
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
            self.take()?;
            let right = self.addition()?;
            Ok(Evaluation::new(expression, operator, right).into_expression_box(self.span(start)))
        } else {
            Ok(expression)
        }
    }

    fn multiplication(&mut self) -> Result<ExpressionBox, ParseError> {
        let start = self.cursor();
        let expression = self.assignment()?;
        if let Some(operator) = self
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
            self.take()?;
            let right = self.multiplication()?;
            Ok(Evaluation::new(expression, operator, right).into_expression_box(self.span(start)))
        } else {
            Ok(expression)
        }
    }

    fn assignment(&mut self) -> Result<ExpressionBox, ParseError> {
        let start = self.cursor();
        let expression = self.unary()?;
        if let Some(operator) = self.soft_peek().and_then(|token| token.as_assignment_operator()) {
            // Note for the below: yes, GM idiotically compiles `foo() = 1` despite it doing absolutely nothing
            // and being extremely misleading. See `assignment_to_call`.
            if !matches!(
                expression.expression(),
                Expression::Identifier(..) | Expression::Access(..) | Expression::Call(..)
            ) {
                Err(ParseError::InvalidAssignmentTarget(self.span(start), expression))
            } else {
                self.take()?;
                let right = self.expression()?;
                Ok(Assignment::new(expression, operator, right).into_expression_box(self.span(start)))
            }
        } else {
            Ok(expression)
        }
    }

    fn unary(&mut self) -> Result<ExpressionBox, ParseError> {
        let start = self.cursor();
        if let Some(operator) = self.peek()?.as_unary_operator() {
            self.take()?;
            let right = self.expression()?;
            Ok(Unary::new(operator, right).into_expression_box(self.span(start)))
        } else {
            self.postfix()
        }
    }

    fn postfix(&mut self) -> Result<ExpressionBox, ParseError> {
        let start = self.cursor();
        let expression = self.literal()?;
        if let Some(operator) = self.soft_peek().and_then(|token| token.as_postfix_operator()) {
            self.take()?;
            Ok(Postfix::new(expression, operator).into_expression_box(self.span(start)))
        } else {
            Ok(expression)
        }
    }

    fn literal(&mut self) -> Result<ExpressionBox, ParseError> {
        let start = self.cursor();
        if let Some(literal) = self.peek()?.to_literal() {
            self.take()?;
            Ok(literal.into_expression_box(self.span(start)))
        } else if self.match_take(Token::LeftSquareBracket).is_some() {
            let mut elements = vec![];
            loop {
                if self.match_take(Token::RightSquareBracket).is_some() {
                    break Ok(Literal::Array(elements).into_expression_box(self.span(start)));
                } else {
                    elements.push(self.expression()?);
                    self.match_take(Token::Comma);
                }
            }
        } else if self.match_take(Token::LeftBrace).is_some() {
            let mut elements = vec![];
            loop {
                if self.match_take(Token::RightBrace).is_some() {
                    break Ok(Literal::Struct(elements).into_expression_box(self.span(start)));
                } else {
                    let name = self.require_identifier()?;
                    self.require(Token::Colon)?;
                    elements.push((name, self.expression()?));
                    self.match_take(Token::Comma);
                }
            }
        } else {
            self.supreme()
        }
    }

    fn supreme(&mut self) -> Result<ExpressionBox, ParseError> {
        let mut has_new = self.match_take(Token::New);
        let mut expression = Some(self.call(None, has_new.take().is_some())?);
        loop {
            expression = match self.soft_peek() {
                Some(Token::LeftParenthesis) => Some(self.call(expression, has_new.take().is_some())?),
                Some(Token::LeftSquareBracket) => Some(self.ds_access(expression)?),
                Some(Token::Dot) => Some(self.dot_access(expression)?),
                _ => break Ok(expression.unwrap()),
            }
        }
    }

    fn call(&mut self, left: Option<ExpressionBox>, uses_new: bool) -> Result<ExpressionBox, ParseError> {
        let start = self.cursor();
        // If we've been provided a leftside expression, we *must* parse for a call.
        // Otherwise, the call is merely possible.
        let left = if let Some(left) = left {
            left
        } else {
            let dot = self.dot_access(None)?;
            if self.soft_peek() != Some(&Token::LeftParenthesis) {
                return Ok(dot);
            }
            dot
        };
        self.require(Token::LeftParenthesis)?;
        let mut arguments = vec![];
        if self.match_take(Token::RightParenthesis).is_none() {
            loop {
                arguments.push(self.expression()?);
                match self.take()? {
                    Token::Comma => {
                        if self.match_take(Token::RightParenthesis).is_some() {
                            break;
                        }
                    }
                    Token::RightParenthesis => break,
                    token => return Err(ParseError::UnexpectedToken(self.span(start), token)),
                }
            }
        }
        Ok(Call {
            left,
            arguments,
            uses_new,
        }
        .into_expression_box(self.span(start)))
    }

    fn dot_access(&mut self, expression: Option<ExpressionBox>) -> Result<ExpressionBox, ParseError> {
        let start = self.cursor();
        let access = if let Some(expression) = expression {
            self.require(Token::Dot)?;
            Access::Dot {
                left: expression,
                right: self.grouping()?,
            }
        } else {
            match self.peek()? {
                Token::Global => {
                    self.take()?;
                    self.require(Token::Dot)?;
                    Access::Global {
                        right: self.grouping()?,
                    }
                }
                Token::SelfKeyword => {
                    self.take()?;
                    if self.match_take(Token::Dot).is_some() {
                        Access::Current {
                            right: self.grouping()?,
                        }
                    } else {
                        // Using self as a referencce!
                        // FIXME: this gives me bad vibes and I feel like is a sign of bad architecting
                        return Ok(Identifier::new("self").into_expression_box(self.span(start)));
                    }
                }
                _ => {
                    let left = self.ds_access(None)?;
                    if self.match_take(Token::Dot).is_some() {
                        Access::Dot {
                            left,
                            right: self.grouping()?,
                        }
                    } else {
                        return Ok(left);
                    }
                }
            }
        };
        Ok(access.into_expression_box(self.span(start)))
    }

    fn ds_access(&mut self, left: Option<ExpressionBox>) -> Result<ExpressionBox, ParseError> {
        let start = self.cursor();
        let left = if let Some(left) = left {
            left
        } else {
            let left = self.grouping()?;
            if self.soft_peek() != Some(&Token::LeftSquareBracket) {
                return Ok(left);
            }
            left
        };
        self.require(Token::LeftSquareBracket)?;
        let access = match self.peek()? {
            Token::DollarSign => {
                self.take()?;
                Access::Struct {
                    left,
                    key: self.expression()?,
                }
            }
            Token::Interrobang => {
                self.take()?;
                Access::Map {
                    left,
                    key: self.expression()?,
                }
            }
            Token::Pipe => {
                self.take()?;
                Access::List {
                    left,
                    index: self.expression()?,
                }
            }
            Token::Hash => {
                self.take()?;
                let index_one = self.expression()?;
                self.require(Token::Comma)?;
                let index_two = self.expression()?;
                Access::Grid {
                    left,
                    index_one,
                    index_two,
                }
            }
            _ => {
                let using_accessor = self.match_take(Token::AtSign).is_some();
                let index_one = self.expression()?;
                let index_two = if self.match_take(Token::Comma).is_some() {
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
        self.require(Token::RightSquareBracket)?;
        Ok(access.into_expression_box(self.span(start)))
    }

    fn grouping(&mut self) -> Result<ExpressionBox, ParseError> {
        let start = self.cursor();
        if self.match_take(Token::LeftParenthesis).is_some() {
            let expression = self.expression()?;
            self.require(Token::RightParenthesis)?;
            Ok(Grouping::new(expression).into_expression_box(self.span(start)))
        } else {
            self.identifier()
        }
    }

    fn identifier(&mut self) -> Result<ExpressionBox, ParseError> {
        let start = self.cursor();
        // FIXME: This is our slightly ludicrous and temporary solution to the static keyword -- we just eat
        // it. Until we have static analysis, it means nothing to us!
        self.match_take(Token::Static);

        let peek = self.peek()?;
        if let Some(lexeme) = peek.as_identifier().map(|s| s.to_string()) {
            self.take()?;
            Ok(Identifier::new(lexeme).into_expression_box(self.span(start)))
        } else {
            self.unexpected_token()
        }
    }

    fn unexpected_token(&mut self) -> Result<ExpressionBox, ParseError> {
        let start = self.cursor();
        // Note: the clone below (instead of .take()) is very intentional!
        // Users should be able to use `self.expression().ok()` and similar patterns
        // without losing tokens.
        Err(ParseError::UnexpectedToken(
            self.span(start),
            self.soft_peek().unwrap().clone(),
        ))
    }
}

// Lexing tools
impl Parser {
    /// Get the gml tokens's cursor.
    fn cursor(&mut self) -> usize {
        self.lexer.peek().map_or(self.cursor, |(c, _)| *c)
    }

    /// Returns the type of the next Token, or returns an error if there is
    /// none.
    fn peek(&mut self) -> Result<&Token, ParseError> {
        let span = self.span(self.cursor);
        if let Some((_, token)) = self.lexer.peek() {
            Ok(token)
        } else {
            Err(ParseError::UnexpectedEnd(span))
        }
    }

    /// Returns the type of the next Token if there is one. Used for situations
    /// where no tokens remaining would be valid.
    fn soft_peek(&mut self) -> Option<&Token> {
        if let Some((_, token)) = self.lexer.peek() {
            Some(token)
        } else {
            None
        }
    }

    /// Consumes and returns the next token if it is the given type.
    fn match_take(&mut self, token: Token) -> Option<Token> {
        if self.peek() == Ok(&token) {
            Some(self.take().unwrap())
        } else {
            None
        }
    }

    /// Continously eats next token if it is the given type.
    fn match_take_repeating(&mut self, token: Token) {
        loop {
            if self.peek() != Ok(&token) {
                break;
            } else {
                self.take().unwrap();
            }
        }
    }

    /// Returns the next Token, returning an error if there is none, or if it is
    /// not of the required type.
    fn require(&mut self, token: Token) -> Result<Token, ParseError> {
        let found_token = self.take()?;
        if found_token == token {
            Ok(found_token)
        } else {
            Err(ParseError::ExpectedToken(self.span(self.cursor), token))
        }
    }

    /// Returns the inner field of the next Token, requiring it to be an
    /// Identifier.
    fn require_identifier(&mut self) -> Result<String, ParseError> {
        let next = self.take()?;
        if let Token::Identifier(v) = next {
            Ok(v)
        } else {
            Err(ParseError::UnexpectedToken(self.span(self.cursor), next))
        }
    }

    /// Returns the inner field of the next Token if it is an Identifier.
    fn match_take_identifier(&mut self) -> Result<Option<String>, ParseError> {
        if matches!(self.peek()?, Token::Identifier(_)) {
            Ok(Some(self.require_identifier()?))
        } else {
            Ok(None)
        }
    }

    /// Returns the next Token, returning an error if there is none.
    fn take(&mut self) -> Result<Token, ParseError> {
        if let Some((position, token)) = self.lexer.next() {
            self.cursor = position;
            Ok(token)
        } else {
            Err(ParseError::UnexpectedEnd(self.span(self.cursor)))
        }
    }
}
