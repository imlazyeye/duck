use crate::parse::{Expr, ExprType, IntoExpr, ParseVisitor, Span, Stmt};

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
    pub fn lazy(lexeme: impl Into<String>) -> Self {
        Self::new(lexeme, Span::default())
    }
}
impl From<Identifier> for ExprType {
    fn from(iden: Identifier) -> Self {
        Self::Identifier(iden)
    }
}
impl IntoExpr for Identifier {}

impl ParseVisitor for Identifier {
    fn visit_child_exprs<E: FnMut(&Expr)>(&self, mut _visitor: E) {}
    fn visit_child_exprs_mut<E: FnMut(&mut Expr)>(&mut self, _visitor: E) {}
    fn visit_child_stmts<S: FnMut(&Stmt)>(&self, mut _visitor: S) {}
    fn visit_child_stmts_mut<S: FnMut(&mut Stmt)>(&mut self, _visitor: S) {}
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(&self.lexeme)
    }
}
