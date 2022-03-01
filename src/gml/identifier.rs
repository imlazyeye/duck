use crate::prelude::{Expression, IntoExpressionBox};

/// Representation of an identifier in gml, which could be any variable.
#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    /// The name of this identifier.
    pub name: String,
}
impl Identifier {
    /// Creates a new identifier.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}
impl From<Identifier> for Expression {
    fn from(iden: Identifier) -> Self {
        Expression::Identifier(iden)
    }
}
impl IntoExpressionBox for Identifier {}
