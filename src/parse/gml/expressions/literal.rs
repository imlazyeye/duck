use hashbrown::HashMap;

use crate::{
    analyze::{Type, Term},
    parse::{Expr, ExprType, IntoExpr, ParseVisitor, Stmt},
};

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
    Array(Vec<Expr>),
    /// A struct literal ({a: 0, b: 0})
    Struct(Vec<(Identifier, Expr)>),
    /// Any GML constant that we are aware of but do not have specific use for.
    Misc(String),
}

impl From<Literal> for ExprType {
    fn from(literal: Literal) -> Self {
        Self::Literal(literal)
    }
}
impl IntoExpr for Literal {}
impl ParseVisitor for Literal {
    fn visit_child_exprs<E: FnMut(&Expr)>(&self, mut visitor: E) {
        match self {
            Literal::Array(members) => {
                for member in members.iter() {
                    visitor(member)
                }
            }
            Literal::Struct(members) => {
                for (iden, value) in members.iter() {
                    visitor(value)
                }
            }
            _ => {}
        }
    }
    fn visit_child_exprs_mut<E: FnMut(&mut Expr)>(&mut self, mut visitor: E) {
        match self {
            Literal::Array(members) => {
                for member in members.iter_mut() {
                    visitor(member)
                }
            }
            Literal::Struct(members) => {
                for (iden, value) in members.iter_mut() {
                    visitor(value)
                }
            }
            _ => {}
        }
    }
    fn visit_child_stmts<S: FnMut(&Stmt)>(&self, mut _visitor: S) {}
    fn visit_child_stmts_mut<S: FnMut(&mut Stmt)>(&mut self, _visitor: S) {}
}
