use crate::parse::{Expression, ExpressionBox, IntoExpressionBox, ParseVisitor, Span, StatementBox, Token, TokenType};

/// Representation of a grouping in gml.
#[derive(Debug, PartialEq, Clone)]
pub struct Grouping {
    /// The inner expression contained by this grouping.
    pub inner: ExpressionBox,
    /// The parentehsis tokens used in this grouping.
    pub tokens: (Token, Token),
}
impl Grouping {
    /// Creates a new grouping.
    pub fn new(inner: ExpressionBox, tokens: (Token, Token)) -> Self {
        Self { inner, tokens }
    }
    /// Creates a new grouping with lazyily generated tokens.
    pub fn lazy(inner: ExpressionBox) -> Self {
        Self {
            inner,
            tokens: (
                Token::new(TokenType::LeftParenthesis, Span::default()),
                Token::new(TokenType::RightParenthesis, Span::default()),
            ),
        }
    }
    /// Returns the parenthesis in this grouping.
    pub fn parenthesis(&self) -> &(Token, Token) {
        &self.tokens
    }
}
impl From<Grouping> for Expression {
    fn from(grouping: Grouping) -> Self {
        Self::Grouping(grouping)
    }
}
impl IntoExpressionBox for Grouping {}
impl ParseVisitor for Grouping {
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, mut visitor: E) {
        visitor(&self.inner);
    }
    fn visit_child_expressions_mut<E: FnMut(&mut ExpressionBox)>(&mut self, mut visitor: E) {
        visitor(&mut self.inner);
    }
    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, mut _visitor: S) {}
    fn visit_child_statements_mut<S: FnMut(&mut StatementBox)>(&mut self, _visitor: S) {}
}
