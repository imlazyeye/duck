use crate::parse::{Expression, ExpressionBox, IntoExpressionBox, ParseVisitor, StatementBox};

use super::Identifier;

/// Representation of a literal in gml, aka a constant compile-time value.
#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    /// true
    True,
    /// false
    False,
    /// undefined
    Undefined,
    /// noone
    Noone,
    /// A string literal
    String(String),
    /// Any number
    Real(f64),
    /// A hex-format number
    Hex(String),
    /// An array literal ([0, 1, 2])
    Array(Vec<ExpressionBox>),
    /// A struct literal ({a: 0, b: 0})
    Struct(Vec<(Identifier, ExpressionBox)>),
    /// Any GML constant that we are aware of but do not have specific use for.
    Misc(String),
}

impl From<Literal> for Expression {
    fn from(literal: Literal) -> Self {
        Self::Literal(literal)
    }
}
impl IntoExpressionBox for Literal {}
impl ParseVisitor for Literal {
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, mut _visitor: E) {}
    fn visit_child_expressions_mut<E: FnMut(&mut ExpressionBox)>(&mut self, _visitor: E) {}
    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, mut _visitor: S) {}
    fn visit_child_statements_mut<S: FnMut(&mut StatementBox)>(&mut self, _visitor: S) {}
}
