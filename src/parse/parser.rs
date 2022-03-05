use crate::{parse::*, FileId};
use std::{iter::Peekable, ops::Range};

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
    file_id: FileId,
}

// Basic features
impl Parser {
    /// Creates a new parser.
    pub fn new(source_code: &'static str, file_id: FileId) -> Self {
        Self {
            lexer: Lexer::new(source_code).peekable(),
            cursor: 0,
            file_id,
            source_code,
        }
    }

    /// Runs the parser through the entire source, collecting everything into an
    /// Ast and returning it.
    ///
    /// ### Errors
    ///
    /// Returns a [ParseError] if any of the source code caused an error.
    pub fn into_ast(mut self) -> Result<Ast, ParseErrorReport> {
        let mut statements: Ast = vec![];
        while self.soft_peek().is_some() {
            statements.push(self.statement()?);
        }
        Ok(statements)
    }

    /// Wraps an expression in a box.
    pub fn box_expression(&self, expression: impl IntoExpressionBox, start_position: usize) -> ExpressionBox {
        expression.into_expression_box(self.span(start_position), self.file_id)
    }

    /// Wraps an expression in a box.
    pub fn box_statement(&self, statement: impl IntoStatementBox, start_position: usize) -> StatementBox {
        statement.into_statement_box(self.span(start_position), self.file_id)
    }

    /// Creates a [Span] from the given position up until the pilot's current
    /// position.
    #[cfg(not(test))]
    fn span(&self, start: usize) -> Span {
        Span(start, self.cursor + 1)
    }

    #[cfg(test)]
    fn span(&self, _start: usize) -> Span {
        Span::default()
    }

    /// Creates a new parse error report based on the the provided ParseError and current state.
    pub fn report(&self, parse_error: ParseError, start_position: usize) -> ParseErrorReport {
        ParseErrorReport::new(parse_error, self.span(start_position), self.file_id)
    }
}

// Recursive descent
impl Parser {
    pub(super) fn statement(&mut self) -> Result<StatementBox, ParseErrorReport> {
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
            Token::LeftBrace | Token::Begin => self.block(),
            Token::Return => self.return_statement(),
            Token::Break => self.break_statement(),
            Token::Continue => self.continue_statement(),
            Token::Exit => self.exit(),
            Token::Globalvar => self.globalvar_declaration(),
            Token::Var => self.local_variable_series(),
            _ => self.assignment(),
        }
    }

    fn macro_declaration(&mut self) -> Result<StatementBox, ParseErrorReport> {
        let start = self.next_token_boundary();
        match self.take()? {
            Token::Macro(name, config, body) => {
                let mac = if let Some(config) = config {
                    Macro::new_with_config(name, body, config)
                } else {
                    Macro::new(name, body)
                };
                Ok(self.box_statement(mac, start))
            }
            token => Err(self.report(ParseError::UnexpectedToken(token), start)),
        }
    }

    fn enum_declaration(&mut self) -> Result<StatementBox, ParseErrorReport> {
        let start = self.next_token_boundary();
        self.require(Token::Enum)?;
        let name = self.require_identifier()?;
        let mut members = vec![];
        self.require_possibilities(&[Token::LeftBrace, Token::Begin])?;
        loop {
            if self
                .match_take_possibilities(&[Token::RightBrace, Token::End])
                .is_some()
            {
                break;
            } else {
                let member_start = self.next_token_boundary();
                let name = self.require_identifier()?;
                let left = self.box_expression(Identifier::new(name), member_start);
                let enum_member = if let Some(equal) = self.match_take(Token::Equal) {
                    let right = self.expression()?;
                    OptionalInitilization::Initialized(self.box_statement(
                        Assignment::new(left, AssignmentOperator::Equal(equal), right),
                        member_start,
                    ))
                } else {
                    OptionalInitilization::Uninitialized(left)
                };
                members.push(enum_member);
                self.match_take(Token::Comma);
            }
        }
        // GM accepts semicolons here, and as such, so do we.
        // FIXME: create an infastrucutre such that we can lint this?
        self.match_take_repeating(Token::SemiColon);
        Ok(self.box_statement(Enum::new_with_members(name, members), start))
    }

    fn try_catch(&mut self) -> Result<StatementBox, ParseErrorReport> {
        let start = self.next_token_boundary();
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
        Ok(self.box_statement(try_catch, start))
    }

    fn for_loop(&mut self) -> Result<StatementBox, ParseErrorReport> {
        let start = self.next_token_boundary();
        self.require(Token::For)?;
        self.match_take(Token::LeftParenthesis);
        let initializer = self.statement()?;
        let condition = self.expression()?;
        self.match_take_repeating(Token::SemiColon);
        let tick = self.statement()?;
        self.match_take(Token::RightParenthesis);
        let body = self.statement()?;
        Ok(self.box_statement(ForLoop::new(initializer, condition, tick, body), start))
    }

    fn with(&mut self) -> Result<StatementBox, ParseErrorReport> {
        let start = self.next_token_boundary();
        self.require(Token::With)?;
        let condition = self.expression()?;
        let body = self.statement()?;
        Ok(self.box_statement(WithLoop::new(condition, body), start))
    }

    fn repeat(&mut self) -> Result<StatementBox, ParseErrorReport> {
        let start = self.next_token_boundary();
        self.require(Token::Repeat)?;
        let condition = self.expression()?;
        let body = self.statement()?;
        Ok(self.box_statement(RepeatLoop::new(condition, body), start))
    }

    fn do_until(&mut self) -> Result<StatementBox, ParseErrorReport> {
        let start = self.next_token_boundary();
        self.require(Token::Do)?;
        let body = self.statement()?;
        self.require(Token::Until)?;
        let condition = self.expression()?;
        self.match_take_repeating(Token::SemiColon);
        Ok(self.box_statement(DoUntil::new(body, condition), start))
    }

    fn while_loop(&mut self) -> Result<StatementBox, ParseErrorReport> {
        let start = self.next_token_boundary();
        self.require(Token::While)?;
        let condition = self.expression()?;
        let body = self.statement()?;
        Ok(self.box_statement(If::new(condition, body), start))
    }

    fn if_statement(&mut self) -> Result<StatementBox, ParseErrorReport> {
        let start = self.next_token_boundary();
        self.require(Token::If)?;
        let condition = self.expression()?;
        let then = self.match_take(Token::Then);
        let body = self.statement()?;
        let else_statement = if self.match_take(Token::Else).is_some() {
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

    fn switch(&mut self) -> Result<StatementBox, ParseErrorReport> {
        let start = self.next_token_boundary();
        fn case_body(parser: &mut Parser) -> Result<Vec<StatementBox>, ParseErrorReport> {
            let mut body = vec![];
            loop {
                match parser.peek()? {
                    Token::Case | Token::Default | Token::RightBrace | Token::End => break,
                    _ => body.push(parser.statement()?),
                }
            }
            Ok(body)
        }
        self.require(Token::Switch)?;
        let expression = self.expression()?;
        self.require_possibilities(&[Token::LeftBrace, Token::Begin])?;
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
                Token::RightBrace | Token::End => {
                    self.take()?;
                    break;
                }
                _ => {
                    let token = self.take()?;
                    return Err(self.report(ParseError::UnexpectedToken(token), start));
                }
            }
        }
        Ok(self.box_statement(Switch::new(expression, members, default), start))
    }

    fn block(&mut self) -> Result<StatementBox, ParseErrorReport> {
        let start = self.next_token_boundary();
        let opening_delimeter = self.require_possibilities(&[Token::LeftBrace, Token::Begin])?;
        let mut statements: Vec<StatementBox> = vec![];
        let closing_delimiter = loop {
            if let Some(token) = self.match_take_possibilities(&[Token::RightBrace, Token::End]) {
                break token;
            } else {
                statements.push(self.statement()?);
            }
        };
        self.match_take_repeating(Token::SemiColon);
        Ok(self.box_statement(
            Block::new(statements, Some((opening_delimeter, closing_delimiter))),
            start,
        ))
    }

    fn return_statement(&mut self) -> Result<StatementBox, ParseErrorReport> {
        let start = self.next_token_boundary();
        self.require(Token::Return)?;
        let expression = self.expression().ok();
        self.match_take_repeating(Token::SemiColon);
        Ok(self.box_statement(Return::new(expression), start))
    }

    fn break_statement(&mut self) -> Result<StatementBox, ParseErrorReport> {
        let start = self.next_token_boundary();
        self.require(Token::Break)?;
        self.match_take_repeating(Token::SemiColon);
        Ok(self.box_statement(Statement::Break, start))
    }

    fn continue_statement(&mut self) -> Result<StatementBox, ParseErrorReport> {
        let start = self.next_token_boundary();
        self.require(Token::Continue)?;
        self.match_take_repeating(Token::SemiColon);
        Ok(self.box_statement(Statement::Continue, start))
    }

    fn exit(&mut self) -> Result<StatementBox, ParseErrorReport> {
        let start = self.next_token_boundary();
        self.require(Token::Exit)?;
        self.match_take_repeating(Token::SemiColon);
        Ok(self.box_statement(Statement::Exit, start))
    }

    fn globalvar_declaration(&mut self) -> Result<StatementBox, ParseErrorReport> {
        let start = self.next_token_boundary();
        self.require(Token::Globalvar)?;
        let name = self.require_identifier()?;
        self.match_take_repeating(Token::SemiColon);
        Ok(self.box_statement(Globalvar::new(name), start))
    }

    fn local_variable_series(&mut self) -> Result<StatementBox, ParseErrorReport> {
        let start = self.next_token_boundary();
        self.require(Token::Var)?;
        let mut declarations = vec![];
        loop {
            let name = self.require_identifier()?;
            let left = self.box_expression(Identifier::new(name), start);
            let local_variable = if let Some(equal) = self.match_take(Token::Equal) {
                let right = self.expression()?;
                OptionalInitilization::Initialized(
                    self.box_statement(Assignment::new(left, AssignmentOperator::Equal(equal), right), start),
                )
            } else {
                OptionalInitilization::Uninitialized(left)
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
        Ok(self.box_statement(LocalVariableSeries::new(declarations), start))
    }

    fn assignment(&mut self) -> Result<StatementBox, ParseErrorReport> {
        let start = self.next_token_boundary();
        let expression = self.unary()?; // Unaries are the highest possibel assignment expressions

        // Check for an identifier followed by an assignment operator
        let assignment = if let Some(operator) = self.soft_peek().and_then(|token| token.as_assignment_operator()) {
            self.take()?;
            Assignment::new(expression, operator, self.expression()?)
        } else if let Expression::Equality(Equality {
            left,
            operator: EqualityOperator::Equal(Token::Equal),
            right,
        }) = *expression.0
        {
            Assignment::new(left, AssignmentOperator::Equal(Token::Equal), right)
        } else {
            // We can't make an assignment out of this -- create an expression statement instead.
            return self.expression_statement(expression);
        };
        self.match_take_repeating(Token::SemiColon);

        // VALIDATION
        // Note for the below: yes, GM idiotically compiles `foo() = 1` despite it doing absolutely
        // nothing and being extremely misleading. See `assignment_to_call`.
        if !matches!(
            assignment.left.expression(),
            Expression::Identifier(..) | Expression::Access(..) | Expression::Call(..)
        ) {
            Err(self.report(ParseError::InvalidAssignmentTarget(assignment.left), start))
        } else {
            Ok(self.box_statement(assignment, start))
        }
    }

    fn expression_statement(&mut self, expression: ExpressionBox) -> Result<StatementBox, ParseErrorReport> {
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
                return Err(self.report(ParseError::IncompleteStatement(expression), start));
            }
        }
        self.match_take_repeating(Token::SemiColon);
        Ok(self.box_statement(Statement::Expression(expression), start))
    }

    pub(super) fn expression(&mut self) -> Result<ExpressionBox, ParseErrorReport> {
        self.logical()
    }

    fn logical(&mut self) -> Result<ExpressionBox, ParseErrorReport> {
        let start = self.next_token_boundary();
        let expression = self.equality()?;
        if let Some(operator) = self.soft_peek().and_then(|token| token.as_logical_operator()) {
            self.take()?;
            let right = self.logical()?;
            Ok(self.box_expression(Logical::new(expression, operator, right), start))
        } else {
            Ok(expression)
        }
    }

    fn equality(&mut self) -> Result<ExpressionBox, ParseErrorReport> {
        let start = self.next_token_boundary();
        let expression = self.binary()?;
        if let Some(operator) = self.soft_peek().and_then(|token| token.as_equality_operator()) {
            self.take()?;
            let right = self.equality()?;
            Ok(self.box_expression(Equality::new(expression, operator, right), start))
        } else {
            Ok(expression)
        }
    }

    fn binary(&mut self) -> Result<ExpressionBox, ParseErrorReport> {
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
            Ok(self.box_expression(Evaluation::new(expression, operator, right), start))
        } else {
            Ok(expression)
        }
    }

    fn bitshift(&mut self) -> Result<ExpressionBox, ParseErrorReport> {
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
            Ok(self.box_expression(Evaluation::new(expression, operator, right), start))
        } else {
            Ok(expression)
        }
    }

    fn addition(&mut self) -> Result<ExpressionBox, ParseErrorReport> {
        let start = self.next_token_boundary();
        let expression = self.multiplication()?;
        if let Some(operator) = self
            .soft_peek()
            .map(|token| token.as_evaluation_operator())
            .filter(|operator| {
                matches!(
                    operator,
                    Some(EvaluationOperator::Plus(Token::Plus)) | Some(EvaluationOperator::Minus(Token::Minus))
                )
            })
            .flatten()
        {
            self.take()?;
            let right = self.addition()?;
            Ok(self.box_expression(Evaluation::new(expression, operator, right), start))
        } else {
            Ok(expression)
        }
    }

    fn multiplication(&mut self) -> Result<ExpressionBox, ParseErrorReport> {
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
            Ok(self.box_expression(Evaluation::new(expression, operator, right), start))
        } else {
            Ok(expression)
        }
    }

    fn unary(&mut self) -> Result<ExpressionBox, ParseErrorReport> {
        let start = self.next_token_boundary();
        if let Some(operator) = self.peek()?.as_unary_operator() {
            self.take()?;
            let right = self.expression()?;
            Ok(self.box_expression(Unary::new(operator, right), start))
        } else {
            self.postfix()
        }
    }

    fn postfix(&mut self) -> Result<ExpressionBox, ParseErrorReport> {
        let start = self.next_token_boundary();
        let expression = self.null_coalecence()?;
        if let Some(operator) = self.soft_peek().and_then(|token| token.as_postfix_operator()) {
            self.take()?;
            Ok(self.box_expression(Postfix::new(expression, operator), start))
        } else {
            Ok(expression)
        }
    }

    fn null_coalecence(&mut self) -> Result<ExpressionBox, ParseErrorReport> {
        let start = self.next_token_boundary();
        let expression = self.ternary()?;
        if self.match_take(Token::DoubleInterrobang).is_some() {
            let value = self.expression()?;
            Ok(self.box_expression(NullCoalecence::new(expression, value), start))
        } else {
            Ok(expression)
        }
    }

    fn ternary(&mut self) -> Result<ExpressionBox, ParseErrorReport> {
        let start = self.next_token_boundary();
        let expression = self.function()?;
        if self.match_take(Token::Interrobang).is_some() {
            let true_value = self.expression()?;
            self.require(Token::Colon)?;
            let false_value = self.expression()?;
            Ok(self.box_expression(Ternary::new(expression, true_value, false_value), start))
        } else {
            Ok(expression)
        }
    }

    fn function(&mut self) -> Result<ExpressionBox, ParseErrorReport> {
        let start = self.next_token_boundary();
        // TODO: when we do static-analysis, this will be used
        let _static_token = self.match_take(Token::Static);
        if self.match_take(Token::Function).is_some() {
            let name = self.match_take_identifier()?.map(|v| v.to_string());
            self.require(Token::LeftParenthesis)?;
            let mut parameters = vec![];
            loop {
                match self.peek()? {
                    Token::RightParenthesis => {
                        self.take()?;
                        break;
                    }
                    _ => {
                        let parameter_start = self.next_token_boundary();
                        let name = self.require_identifier()?;
                        let name = self.box_expression(Identifier::new(name), parameter_start);
                        if self.match_take(Token::Equal).is_some() {
                            let assignment =
                                Assignment::new(name, AssignmentOperator::Equal(Token::Equal), self.expression()?);
                            parameters.push(OptionalInitilization::Initialized(
                                self.box_statement(assignment, parameter_start),
                            ));
                        } else {
                            parameters.push(OptionalInitilization::Uninitialized(name));
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
                    return Err(self.report(ParseError::UnexpectedToken(Token::Colon), colon_position));
                }
                None
            };
            let body = self.block()?;
            Ok(self.box_expression(
                Function {
                    name,
                    parameters,
                    constructor,
                    body,
                },
                start,
            ))
        } else {
            self.literal()
        }
    }

    fn literal(&mut self) -> Result<ExpressionBox, ParseErrorReport> {
        let start = self.next_token_boundary();
        if let Some(literal) = self.peek()?.to_literal() {
            self.take()?;
            Ok(self.box_expression(literal, start))
        } else if self.match_take(Token::LeftSquareBracket).is_some() {
            let mut elements = vec![];
            loop {
                if self.match_take(Token::RightSquareBracket).is_some() {
                    break Ok(self.box_expression(Literal::Array(elements), start));
                } else {
                    elements.push(self.expression()?);
                    self.match_take(Token::Comma);
                }
            }
        } else if self
            .match_take_possibilities(&[Token::LeftBrace, Token::Begin])
            .is_some()
        {
            let mut elements = vec![];
            loop {
                if self
                    .match_take_possibilities(&[Token::RightBrace, Token::End])
                    .is_some()
                {
                    break Ok(self.box_expression(Literal::Struct(elements), start));
                } else {
                    let name = self.require_identifier()?;
                    self.require(Token::Colon)?;
                    elements.push((name.into(), self.expression()?));
                    self.match_take(Token::Comma);
                }
            }
        } else {
            self.supreme()
        }
    }

    fn supreme(&mut self) -> Result<ExpressionBox, ParseErrorReport> {
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

    fn call(&mut self, left: Option<ExpressionBox>, uses_new: bool) -> Result<ExpressionBox, ParseErrorReport> {
        // If we've been provided a leftside expression, we *must* parse for a call.
        // Otherwise, the call is merely possible.
        let (start, left) = if let Some(left) = left {
            (left.span().0, left)
        } else {
            let start = self.next_token_boundary();

            let dot = self.dot_access(None)?;
            if self.soft_peek() != Some(&Token::LeftParenthesis) {
                return Ok(dot);
            }
            (start, dot)
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
                    token => return Err(self.report(ParseError::UnexpectedToken(token), start)),
                }
            }
        }
        Ok(self.box_expression(
            Call {
                left,
                arguments,
                uses_new,
            },
            start,
        ))
    }

    fn dot_access(&mut self, expression: Option<ExpressionBox>) -> Result<ExpressionBox, ParseErrorReport> {
        let mut start = self.next_token_boundary();
        let access = if let Some(expression) = expression {
            self.require(Token::Dot)?;
            start = expression.span().0;
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
                        return Ok(self.box_expression(Identifier::new("self"), start));
                    }
                }
                Token::Other => {
                    self.take()?;
                    if self.match_take(Token::Dot).is_some() {
                        Access::Other {
                            right: self.grouping()?,
                        }
                    } else {
                        // Using other as a reference!
                        // FIXME: me too!
                        return Ok(self.box_expression(Identifier::new("other"), start));
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
        Ok(self.box_expression(access, start))
    }

    fn ds_access(&mut self, left: Option<ExpressionBox>) -> Result<ExpressionBox, ParseErrorReport> {
        let (start, left) = if let Some(left) = left {
            (left.span().0, left)
        } else {
            let left = self.grouping()?;
            if self.soft_peek() != Some(&Token::LeftSquareBracket) {
                return Ok(left);
            }
            (self.next_token_boundary(), left)
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
        Ok(self.box_expression(access, start))
    }

    fn grouping(&mut self) -> Result<ExpressionBox, ParseErrorReport> {
        let start = self.next_token_boundary();
        if self.match_take(Token::LeftParenthesis).is_some() {
            let expression = self.expression()?;
            self.require(Token::RightParenthesis)?;
            Ok(self.box_expression(Grouping::new(expression), start))
        } else {
            self.identifier()
        }
    }

    fn identifier(&mut self) -> Result<ExpressionBox, ParseErrorReport> {
        // FIXME: This is our slightly ludicrous and temporary solution to the static keyword -- we just eat
        // it. Until we have static analysis, it means nothing to us!
        self.match_take(Token::Static);

        let start = self.next_token_boundary();
        let peek = self.peek()?;
        if let Some(lexeme) = peek.as_identifier().map(|s| s.to_string()) {
            self.take()?;
            Ok(self.box_expression(Identifier::new(lexeme), start))
        } else {
            self.unexpected_token()
        }
    }

    fn unexpected_token(&mut self) -> Result<ExpressionBox, ParseErrorReport> {
        let start = self.next_token_boundary();
        let token = *self.peek()?; // todo gabe here. if you don't know what this means, delete it
        Err(self.report(ParseError::UnexpectedToken(token), start))
    }
}

// Lexing tools
impl Parser {
    /// Get the gml tokens's cursor.
    fn next_token_boundary(&mut self) -> usize {
        self.lexer.peek().map_or(self.cursor, |(c, _)| *c)
    }

    /// Returns the type of the next Token, or returns an error if there is
    /// none.
    fn peek(&mut self) -> Result<&Token, ParseErrorReport> {
        let start = self.next_token_boundary();
        let report = self.report(ParseError::UnexpectedEnd, start); // FIXME: I'm doing this early because of the borrow checker
        let next = self.lexer.peek();
        if let Some(next) = next {
            Ok(&next.1)
        } else {
            Err(report)
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

    /// Consumes and returns the next token if it is within the array of types.
    fn match_take_possibilities(&mut self, tokens: &[Token]) -> Option<Token> {
        if self.peek().map(|token| tokens.contains(token)).unwrap_or(false) {
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
    fn require(&mut self, token: Token) -> Result<Token, ParseErrorReport> {
        let found_token = self.take()?;
        if found_token == token {
            Ok(found_token)
        } else {
            Err(self.report(ParseError::ExpectedToken(token), self.cursor))
        }
    }

    /// Returns the next Token, returning an error if there is none, or if it is
    /// not within the provided array of required types.
    fn require_possibilities(&mut self, tokens: &[Token]) -> Result<Token, ParseErrorReport> {
        let found_token = self.take()?;
        if tokens.contains(&found_token) {
            Ok(found_token)
        } else {
            Err(self.report(ParseError::ExpectedTokens(tokens.to_vec()), self.cursor))
        }
    }

    /// Returns the inner field of the next Token, requiring it to be an
    /// Identifier.
    fn require_identifier(&mut self) -> Result<&'static str, ParseErrorReport> {
        let next = self.take()?;
        if let Token::Identifier(v) = next {
            Ok(v)
        } else {
            Err(self.report(ParseError::UnexpectedToken(next), self.cursor))
        }
    }

    /// Returns the inner field of the next Token if it is an Identifier.
    fn match_take_identifier(&mut self) -> Result<Option<&str>, ParseErrorReport> {
        if matches!(self.peek()?, Token::Identifier(_)) {
            Ok(Some(self.require_identifier()?))
        } else {
            Ok(None)
        }
    }

    /// Returns the next Token, returning an error if there is none.
    fn take(&mut self) -> Result<Token, ParseErrorReport> {
        if let Some((position, token)) = self.lexer.next() {
            self.cursor = position;
            Ok(token)
        } else {
            Err(self.report(ParseError::UnexpectedEnd, self.cursor))
        }
    }
}

/// A start and end cursor measured in characters, used for expressing small sections of source
/// code.
#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct Span(pub usize, pub usize);
impl From<Span> for Range<usize> {
    fn from(span: Span) -> Self {
        span.0..span.1
    }
}

/// A location for something in gml, combining a span and a file id.
#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct Location(pub FileId, pub Span);
