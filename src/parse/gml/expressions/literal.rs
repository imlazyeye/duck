use serde::{Serialize, Serializer};

use crate::parse::{Expr, ExprKind, IntoExpr, ParseVisitor, Stmt};

use super::Identifier;

/// Representation of a literal in gml, aka a constant compile-time value.
#[derive(Debug, PartialEq, Clone, serde::Serialize)]
#[serde(rename_all = "snake_case")]
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
    Struct(#[serde(serialize_with = "serialize_struct_fields")] Vec<StructField>),
    /// Any GML constant that we are aware of but do not have specific use for.
    Misc(String),
}

type StructField = (Identifier, Expr);
#[derive(serde::Serialize)]
struct SerializedStructField {
    name: Identifier,
    value: Expr,
}
fn serialize_struct_fields<S>(fields: &[StructField], serde: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    fields
        .iter()
        .cloned()
        .map(|(name, value)| SerializedStructField { name, value })
        .collect::<Vec<SerializedStructField>>()
        .serialize(serde)
}

impl From<Literal> for ExprKind {
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
                for (_, value) in members.iter() {
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
                for (_, value) in members.iter_mut() {
                    visitor(value)
                }
            }
            _ => {}
        }
    }
    fn visit_child_stmts<S: FnMut(&Stmt)>(&self, mut _visitor: S) {}
    fn visit_child_stmts_mut<S: FnMut(&mut Stmt)>(&mut self, _visitor: S) {}
}
