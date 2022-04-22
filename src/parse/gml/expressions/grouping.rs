use crate::parse::{Expr, ExprKind, IntoExpr, ParseVisitor, Span, Stmt, Token, TokenKind};

/// Representation of a grouping in gml.
#[derive(Debug, PartialEq, Clone)]
pub struct Grouping {
    /// The inner expression contained by this grouping.
    pub inner: Expr,
    /// The parentehsis tokens used in this grouping.
    pub tokens: (Token, Token),
}
impl Grouping {
    /// Creates a new grouping.
    pub fn new(inner: Expr, tokens: (Token, Token)) -> Self {
        Self { inner, tokens }
    }
    /// Creates a new grouping with lazyily generated tokens.
    pub fn lazy(inner: Expr) -> Self {
        Self {
            inner,
            tokens: (
                Token::new(TokenKind::LeftParenthesis, Span::default()),
                Token::new(TokenKind::RightParenthesis, Span::default()),
            ),
        }
    }
    /// Returns the parenthesis in this grouping.
    pub fn parenthesis(&self) -> &(Token, Token) {
        &self.tokens
    }
}
impl From<Grouping> for ExprKind {
    fn from(grouping: Grouping) -> Self {
        Self::Grouping(grouping)
    }
}
impl IntoExpr for Grouping {}
impl ParseVisitor for Grouping {
    fn visit_child_exprs<E: FnMut(&Expr)>(&self, mut visitor: E) {
        visitor(&self.inner);
    }
    fn visit_child_exprs_mut<E: FnMut(&mut Expr)>(&mut self, mut visitor: E) {
        visitor(&mut self.inner);
    }
    fn visit_child_stmts<S: FnMut(&Stmt)>(&self, mut _visitor: S) {}
    fn visit_child_stmts_mut<S: FnMut(&mut Stmt)>(&mut self, _visitor: S) {}
}
