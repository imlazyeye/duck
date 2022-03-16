use hashbrown::HashMap;

use crate::{
    analyze::Type,
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
impl Literal {
    /// Returns the type of this literal, inferring as much as it can confidently.
    pub fn as_type(&self) -> Type {
        match self {
            Literal::True | Literal::False => Type::Bool,
            Literal::Undefined => Type::Undefined,
            Literal::Noone => Type::Noone,
            Literal::String(_) => Type::String,
            Literal::Real(_) | Literal::Hex(_) => Type::Real,
            Literal::Array(exprs) => {
                // If all of the expressions have a consistent type, then we can be confident
                match exprs.first().map(|v| &v.tpe) {
                    Some(potential_type) => {
                        for expr in exprs.iter().skip(1) {
                            if &expr.tpe != potential_type {
                                return Type::Array(Box::new(Type::Unknown));
                            }
                        }
                        Type::Array(Box::new(potential_type.clone()))
                    }
                    None => Type::Array(Box::new(Type::Unknown)),
                }
            }
            Literal::Struct(declarations) => {
                // We can construct a type for this since we'll know the structure of the fields,
                // even if we don't know the type of the fields themselves
                let mut fields = HashMap::default();
                for declaration in declarations {
                    fields.insert(declaration.0.lexeme.clone(), declaration.1.tpe.clone());
                }
                Type::Struct(fields)
            }
            Literal::Misc(_) => Type::Unknown,
        }
    }
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
