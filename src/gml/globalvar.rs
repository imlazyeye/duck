use crate::prelude::{IntoStatementBox, Statement};

/// Representation of a globalvar in gml.
#[derive(Debug, PartialEq, Clone)]
pub struct Globalvar {
    /// The name of the declared globalvar.
    pub name: String,
}
impl Globalvar {
    /// Creates a new globalvar with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}
impl From<Globalvar> for Statement {
    fn from(glob: Globalvar) -> Self {
        Statement::GlobalvarDeclaration(glob)
    }
}
impl IntoStatementBox for Globalvar {}
