use crate::parse::{Expression, ExpressionBox, IntoExpressionBox, ParseVisitor, Span, StatementBox};

/// Representation of an identifier in gml, which could be any variable.
#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    /// The name of this identifier
    pub lexeme: String,
    /// The span that came from the original token
    pub span: Span,
}
impl Identifier {
    /// Creates a new identifier.
    pub fn new(lexeme: impl Into<String>, span: Span) -> Self {
        Self {
            lexeme: lexeme.into(),
            span,
        }
    }

    /// Creates a new identifier with a default span.
    #[cfg(test)]
    pub fn lazy(lexeme: impl Into<String>) -> Self {
        Self::new(lexeme, Span::default())
    }
}
impl From<Identifier> for Expression {
    fn from(iden: Identifier) -> Self {
        Self::Identifier(iden)
    }
}
impl IntoExpressionBox for Identifier {}
impl ParseVisitor for Identifier {
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, mut _visitor: E) {}
    fn visit_child_expressions_mut<E: FnMut(&mut ExpressionBox)>(&mut self, _visitor: E) {}
    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, mut _visitor: S) {}
    fn visit_child_statements_mut<S: FnMut(&mut StatementBox)>(&mut self, _visitor: S) {}
}
