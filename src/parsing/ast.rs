use std::str::ParseBoolError;

use crate::ParseError;

use super::{
    expression::{Expression, ExpressionBox, Operator},
    statement::StatementBox,
    token_pilot::TokenPilot,
    Token,
};

pub struct Ast {
    pilot: TokenPilot,
}
impl Ast {
    fn expression_statement(&mut self) -> Result<StatementBox, ParseError> {
        self.expression()?;
        todo!()
    }

    fn expression(&mut self) -> Result<ExpressionBox, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<ExpressionBox, ParseError> {
        let expression = self.binary()?;
        if self.pilot.match_take(Token::Equals).is_some() {
            let value = self.expression()?;
            Ok(Expression::Assign(expression, value).to_box())
        } else {
            Ok(expression)
        }
    }

    fn binary(&mut self) -> Result<ExpressionBox, ParseError> {
        let expression = self.unary()?;
        if let Some(operator) = self
            .pilot
            .peek()
            .ok()
            .map(|token| token.to_operator())
            .flatten()
        {
            self.pilot.take()?;
            let right = self.expression()?;
            Ok(Expression::Binary(expression, operator, right).to_box())
        } else {
            Ok(expression)
        }
    }

    fn unary(&mut self) -> Result<ExpressionBox, ParseError> {
        if let Some(operator) = self
            .pilot
            .peek()?
            .to_operator()
            .filter(|operator| matches!(operator, Operator::Bang | Operator::Minus))
        {
            self.pilot.take()?;
            let right = self.expression()?;
            Ok(Expression::Unary(operator, right).to_box())
        } else {
            self.call()
        }
    }

    fn call(&mut self) -> Result<ExpressionBox, ParseError> {
        let expression = self.access()?;
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
            Ok(Expression::Call(expression, arguments).to_box())
        } else {
            Ok(expression)
        }
    }

    fn access(&mut self) -> Result<ExpressionBox, ParseError> {
        let expression = self.primary()?;
        if self.pilot.match_take(Token::Dot).is_some() {
            let right = match self.pilot.take()? {
                Token::Identifier(lexeme) => lexeme,
                token => return Err(ParseError::UnexpectedToken(self.pilot.cursor(), token)),
            };
            Ok(Expression::Access(expression, right).to_box())
        } else {
            Ok(expression)
        }
    }

    fn primary(&mut self) -> Result<ExpressionBox, ParseError> {
        let token = self.pilot.take()?;
        if let Some(literal) = token.to_literal() {
            Ok(Expression::Literal(literal).to_box())
        } else {
            match token {
                Token::Identifier(name) => Ok(ExpressionBox::new(Expression::Identifier(name))),
                Token::LeftParenthesis => {
                    let expression = self.expression()?;
                    self.pilot.require(Token::RightParenthesis)?;
                    Ok(ExpressionBox::new(Expression::Grouping(expression)))
                }
                _ => Err(ParseError::UnexpectedToken(self.pilot.cursor(), token)),
            }
        }
    }
}

#[cfg(test)]
mod ast_tests {
    use crate::parsing::{
        expression::{Expression, ExpressionBox, Literal, Operator},
        Parser, Token,
    };

    use super::Ast;

    fn harness_expr(source: &str, expected: Expression) {
        let parser = Parser::new(source.into(), "test".into());
        let mut ast = Ast {
            pilot: parser.token_pilot(),
        };
        assert_eq!(*ast.expression().unwrap(), expected);
    }

    #[test]
    fn assignment() {
        harness_expr(
            "foo = 0",
            Expression::Assign(
                Expression::Identifier("foo".into()).to_box(),
                Expression::Literal(Literal::Real(0.0)).to_box(),
            ),
        );
    }

    #[test]
    fn binary() {
        harness_expr(
            "1 + 1",
            Expression::Binary(
                Expression::Literal(Literal::Real(1.0)).to_box(),
                Operator::Plus,
                Expression::Literal(Literal::Real(1.0)).to_box(),
            ),
        );
        harness_expr(
            "1 - 1",
            Expression::Binary(
                Expression::Literal(Literal::Real(1.0)).to_box(),
                Operator::Minus,
                Expression::Literal(Literal::Real(1.0)).to_box(),
            ),
        );
        harness_expr(
            "1 * 1",
            Expression::Binary(
                Expression::Literal(Literal::Real(1.0)).to_box(),
                Operator::Star,
                Expression::Literal(Literal::Real(1.0)).to_box(),
            ),
        );
        harness_expr(
            "1 / 1",
            Expression::Binary(
                Expression::Literal(Literal::Real(1.0)).to_box(),
                Operator::Slash,
                Expression::Literal(Literal::Real(1.0)).to_box(),
            ),
        );
        harness_expr(
            "1 mod 1",
            Expression::Binary(
                Expression::Literal(Literal::Real(1.0)).to_box(),
                Operator::Mod,
                Expression::Literal(Literal::Real(1.0)).to_box(),
            ),
        );
        harness_expr(
            "1 % 1",
            Expression::Binary(
                Expression::Literal(Literal::Real(1.0)).to_box(),
                Operator::Mod,
                Expression::Literal(Literal::Real(1.0)).to_box(),
            ),
        );
        harness_expr(
            "1 div 1",
            Expression::Binary(
                Expression::Literal(Literal::Real(1.0)).to_box(),
                Operator::Div,
                Expression::Literal(Literal::Real(1.0)).to_box(),
            ),
        );
        harness_expr(
            "1 < 1",
            Expression::Binary(
                Expression::Literal(Literal::Real(1.0)).to_box(),
                Operator::LessThan,
                Expression::Literal(Literal::Real(1.0)).to_box(),
            ),
        );
        harness_expr(
            "1 <= 1",
            Expression::Binary(
                Expression::Literal(Literal::Real(1.0)).to_box(),
                Operator::LessThanOrEqual,
                Expression::Literal(Literal::Real(1.0)).to_box(),
            ),
        );
        harness_expr(
            "1 > 1",
            Expression::Binary(
                Expression::Literal(Literal::Real(1.0)).to_box(),
                Operator::GreaterThan,
                Expression::Literal(Literal::Real(1.0)).to_box(),
            ),
        );
        harness_expr(
            "1 >= 1",
            Expression::Binary(
                Expression::Literal(Literal::Real(1.0)).to_box(),
                Operator::GreaterThanOrEqual,
                Expression::Literal(Literal::Real(1.0)).to_box(),
            ),
        );
        harness_expr(
            "1 && 1",
            Expression::Binary(
                Expression::Literal(Literal::Real(1.0)).to_box(),
                Operator::And,
                Expression::Literal(Literal::Real(1.0)).to_box(),
            ),
        );
        harness_expr(
            "1 || 1",
            Expression::Binary(
                Expression::Literal(Literal::Real(1.0)).to_box(),
                Operator::Or,
                Expression::Literal(Literal::Real(1.0)).to_box(),
            ),
        );
        harness_expr(
            "1 == 1",
            Expression::Binary(
                Expression::Literal(Literal::Real(1.0)).to_box(),
                Operator::Equals,
                Expression::Literal(Literal::Real(1.0)).to_box(),
            ),
        );
    }

    #[test]
    fn unary() {
        harness_expr(
            "!foo",
            Expression::Unary(
                Operator::Bang,
                Expression::Identifier("foo".into()).to_box(),
            ),
        );
        harness_expr(
            "-1",
            Expression::Unary(
                Operator::Minus,
                Expression::Literal(Literal::Real(1.0)).to_box(),
            ),
        );
    }

    #[test]
    fn calls() {
        harness_expr(
            "foo()",
            Expression::Call(Expression::Identifier("foo".into()).to_box(), vec![]),
        );
        harness_expr(
            "foo(0, 1, 2)",
            Expression::Call(
                Expression::Identifier("foo".into()).to_box(),
                vec![
                    Expression::Literal(Literal::Real(0.0)).to_box(),
                    Expression::Literal(Literal::Real(1.0)).to_box(),
                    Expression::Literal(Literal::Real(2.0)).to_box(),
                ],
            ),
        );
    }

    #[test]
    fn access() {
        harness_expr(
            "foo.bar",
            Expression::Access(Expression::Identifier("foo".into()).to_box(), "bar".into()),
        );
    }

    #[test]
    fn grouping() {
        harness_expr(
            "(0)",
            Expression::Grouping(Expression::Literal(Literal::Real(0.0)).to_box()),
        );
    }

    #[test]
    fn identifier() {
        harness_expr("foo", Expression::Identifier("foo".into()));
    }

    #[test]
    fn literals() {
        harness_expr("0", Expression::Literal(Literal::Real(0.0)));
        harness_expr("true", Expression::Literal(Literal::True));
        harness_expr(
            "\"foo\"",
            Expression::Literal(Literal::String("foo".into())),
        );
    }
}
